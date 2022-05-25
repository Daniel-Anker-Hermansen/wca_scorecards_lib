use std::{sync::{Arc, Mutex}, collections::HashMap};
use crate::wcif::{oauth::OAuth, json::parse};
use crate::wcif::*;
use warp::{Filter, hyper::Response};

use self::html::event_list_to_html;

mod html;

pub fn init(id: String) {
    //Url to approve the Oauth application
    let auth_url = "https://www.worldcubeassociation.org/oauth/authorize?client_id=TDg_ARkGANTJB_z0oeUWBVl66a1AYdYAxc-jPJIhSfY&redirect_uri=http%3A%2F%2Flocalhost%3A5000%2F&response_type=code&scope=public+manage_competitions";

    //Mutex for storing the authentification code for async reasons.
    let code: Arc<Mutex<Option<OAuth>>> = Arc::new(Mutex::new(None));
    let wcif: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let local_wcif = wcif.clone();
    let local_wcif2 = wcif.clone();
    //Handling the get request from authentification. HTTP no s, super secure, everything is awesome. The API said that https is not required for localhost so it is fine.
    let root = warp::path::end()
        .and(warp::query::query())
        .map(move |s: HashMap<String, String>|{
            let code_clone = code.clone();
            let wcif_clone = wcif.clone();
            let id = id.clone();
            std::thread::spawn(move ||{
                s.iter().for_each(|(_,v)|{
                    let oauth = OAuth::get_auth(v.to_string());
                    let json = oauth.get_wcif(id.as_str());
                    let mut wcif_guard = wcif_clone.lock().unwrap();
                    *wcif_guard = json.clone();
                    drop(wcif_guard);
                    let mut guard = code_clone.lock().unwrap();
                    *guard = Some(oauth);
                    drop(guard);
                });
            });
            loop {
                std::thread::sleep(std::time::Duration::new(1,0) / 120);
                let wcif_guard = wcif.lock().unwrap();
                if (*wcif_guard).is_some() {
                    let json = (*wcif_guard).clone().unwrap();
                    drop(wcif_guard);
                    let wcif = parse(json);
                    let body = format!("{}", event_list_to_html(get_rounds(wcif)));
                    let response = Response::builder()
                        .header("content-type", "text/html")
                        .body(body);
                    return response;
                }
            }
        })
        .with(warp::cors().allow_any_origin());

    let pdf = warp::path!("round" / "pdf")
        .and(warp::query::query())
        .map(move |s: HashMap<String, String>|{
            let (eventid, round, group) = s.iter().fold(("", 0, ""),|(e, r, g), (k, v)|{
                match k.as_str() {
                    "eventid" => (v, r, g),
                    "round" => (e, usize::from_str_radix(v, 10).unwrap(), g),
                    "groups" => (e, r, v),
                    _ => panic!("Invalid query")
                }
            });
            let wcif_guard = local_wcif2.lock().unwrap();
            let json = (*wcif_guard).clone().unwrap();
            drop(wcif_guard);
            let wcif = parse(json);
            let groups: Vec<Vec<_>> = group.split("$")
                .map(|group|{
                    if group == "" {
                        vec![]
                    }
                    else {
                        group.split("s")
                            .map(|id|{
                                usize::from_str_radix(id, 10).unwrap()
                            })
                            .collect()
                    }
                })
                .collect();

            let bytes = crate::pdf::run_from_wcif(wcif, eventid, round, groups);
            
            let response = Response::builder()
                .header("content-type", "application/pdf")
                .body(bytes);
            return response;
        })
        .with(warp::cors().allow_any_origin());

    let round = warp::path!("round")
        .and(warp::query::query())
        .map(move |s: HashMap<String,String>|{
            let (eventid, round) = s.iter().fold(("", 0),|(e, r), (k, v)|{
                match k.as_str() {
                    "eventid" => (v, r),
                    "round" => (e, usize::from_str_radix(v, 10).unwrap()),
                    _ => panic!("Invalid query")
                }
            });
            let wcif_guard = local_wcif.lock().unwrap();
            let json = (*wcif_guard).clone().unwrap();
            drop(wcif_guard);
            let wcif = parse(json);
            let (competitors, map, _, _) = super::wcif::get_scorecard_info_for_round(wcif, eventid, round);
            let str = competitors.iter()
                .rev()
                .map(|id|{
                    format!("{}\\r{}", id, map[id])
                })
                .collect::<Vec<_>>()
                .join("\\n");
            let response = Response::builder()
                .header("content-type", "text/html; charset=utf-8")
                .body(crate::compiled::js_replace(&str, competitors.len(), eventid, round));
            return response;
        })
        .with(warp::cors().allow_any_origin());

    let wasm_js = warp::path!("round" / "pkg" / "group_menu.js")
    .map(|| Response::builder()
        .header("content-type", "text/javascript")
        .body(crate::compiled::WASM_JS));

    let js = warp::path!("round" / "pkg" / "snippets" / "group_menu-c33353fa00f3dafb" / "src" / "js.js")
        .map(|| Response::builder()
            .header("content-type", "text/javascript")
            .body(crate::compiled::JS));
    
    let wasm = warp::path!("round" / "pkg" / "group_menu_bg.wasm")
    .map(|| Response::builder()
        .header("content-type", "text/wasm")
        .body(crate::compiled::WASM));

    let routes = root
        .or(round)
        .or(pdf)
        .or(wasm_js)
        .or(js)
        .or(wasm);
    //Mutex for knowing when to force open authentification url. Opening before server is listening will break the server for some reason which i do not understand.
    let is_hosting = Arc::new(Mutex::new(false));
    let closure_is_hosting = is_hosting.clone();
    std::thread::spawn(move ||{
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let mut guard = closure_is_hosting.lock().unwrap();
            *guard = true;
            drop(guard);
            warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
        };
        rt.block_on(future);
    });

    //Checking whether code has been received and whether it is time to open authentification url at 120 tps.
    loop {
        std::thread::sleep(std::time::Duration::new(1,0) / 120);
        let mut guard = is_hosting.lock().unwrap();
        if *guard {
            *guard = false;
            drop(guard);

            //Try opening in browser. In case of fail write the url to the terminal
            match open::that(auth_url) {
                Err(_) => {
                    println!("Please open the following website and follow the instructions:");
                    println!("{}", auth_url);
                }
                Ok(_) => ()
            }
        }
    }
}