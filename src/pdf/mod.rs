use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use crate::{language::Language, wcif::json::Wcif};
use scorecard::{Scorecard, TimeLimit, scorecards_to_pdf};

pub mod scorecard;
mod font;

pub fn run(groups_csv: &str, limit_csv: &str, competition: &str, language: Language) {
    let mut groups_csv = groups_csv.lines();
    //Header describing csv file formatting. First two are fixed and therfore skipped.
    //Unwrap cannot fail because the first element of lines always exists, although skip can lead
    //to panic later when used.
    let header = groups_csv.next().unwrap().split(",").skip(2);
    let mut map = HashMap::new();
    let mut k = groups_csv
        //Filter off empty lines. Fixes annoying EOF issues.
        .filter(|x|*x != "")
        //Map each person to each event they compete in.
        //Enumerate for panic messages
        .enumerate()
        .map(|(line, person)|{
            let mut iter = person.split(",");
            let name = match iter.next() {
                None => panic!("Line {} in csv missing name", line + 2),
                Some(v) => v
            };
            let id = match iter.next() {
                None => panic!("Line {} in csv missing id", line + 2),
                Some(v) => v
            };
            let id = match usize::from_str_radix(id, 10) {
                Err(_) => panic!("Id for {} in line {} is not a positive integer", name, line + 2),
                Ok(v) => v
            };
            //Insert the competitor into the id to name map.
            map.insert(id, name.to_string());
            //Zipping with header (clone) to know the order of events.
            iter.zip(header.clone())
                .filter_map(move |(asign, event)|{
                //Test whether competitor is assigned.
                if asign == "" {
                    return None
                }
                else {
                    let mut info = asign.split(";");
                    let pre_group = info.next()?;
                    let group = match usize::from_str_radix(pre_group, 10) {
                        Err(_) => panic!("Group number for event {} in line {} is nut a positive integer", event, line + 2),
                        Ok(v) => v
                    };
                    let station = info.next().map(|v| match usize::from_str_radix(v, 10) {
                        Err(_) => panic!("Sation number for event {} in line {} is not a positive integer", event, line + 2),
                        Ok(v) => v
                    });
                    Some((id, event, group, station))
                }
            })
        })
        .flatten()
        .map(|(id, event, group, station)|{
            Scorecard {
                id,
                group,
                round: 1,
                station,
                event
            }
        })
        .collect::<Vec<_>>();
    //Sort scorecards by event, round, group, station (Definition order) 
    k.sort();


    //Parse time limits
    let mut limit = limit_csv.lines();
    //Header cannot fail because first in lines
    let event_list = limit.next().unwrap().split(",");
    let limit_data = match limit.next() {
        None => panic!("No time limits given in time limit csv"),
        Some(v) => v
    }.split(",");

    let mut limits = HashMap::new();
    limit_data.zip(event_list).for_each(|(x, event)|{
        let mut iter = x.split(";");
        let v = match iter.next() {
            None => {
                limits.insert(event, TimeLimit::None);
                return;
            }
            Some(v) => v,
        };
        match v {
            "T" => limits.insert(event, TimeLimit::Single(usize_from_iter(&mut iter))),
            "C" => limits.insert(event, TimeLimit::Cumulative(usize_from_iter(&mut iter))),
            "K" => limits.insert(event, TimeLimit::Cutoff(usize_from_iter(&mut iter), usize_from_iter(&mut iter))),
            "S" => limits.insert(event, TimeLimit::SharedCumulative(usize_from_iter(&mut iter), iter.map(|x|x.to_string()).collect::<Vec<_>>())),
            "M" => limits.insert(event, TimeLimit::Multi),
            _ => panic!("Malformatted time limit for event: {}", event)
        };
    });

    //Generate pdf
    let doc = scorecards_to_pdf(k, competition, &map, &limits, language);

    //Saving pdf
    doc.save(&mut BufWriter::new(File::create(competition.split_ascii_whitespace().collect::<String>() + "_scorecards.pdf").unwrap())).unwrap();
}

pub fn run_from_wcif(wcif: Wcif, event: &str, round: usize, groups: Vec<Vec<usize>>) -> Vec<u8> {
    let (_, map, limit, competition) = super::wcif::get_scorecard_info_for_round(wcif, event, round);

    let mut limits = HashMap::new();
    limits.insert(event, limit);

    let k = groups.into_iter()
        .enumerate()
        .map(|(n, group)|{
            group.into_iter()
                .enumerate()
                .map(move |(station, id)|{
                    Scorecard {
                        event,
                        round,
                        group: n + 1,
                        station: Some(station + 1),
                        id
                    }
                })
        }).flatten()
        .collect::<Vec<_>>();
    
    let doc = scorecards_to_pdf(k, &competition, &map, &limits, Language::english());
    doc.save_to_bytes().unwrap()
}

fn usize_from_iter<'a, I>(iter: &mut I) -> usize where I: Iterator<Item = &'a str> {
    match usize::from_str_radix(match iter.next() {
        None => panic!("Malformatted input file. Missing data, where integer was expected"),
        Some(v) => v
    }, 10) {
        Err(_) => panic!("Malformatted input file. Expected positive integer, but received other charachters"),
        Ok(v) => v
    }
}
