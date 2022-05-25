use pdf::run;
use language::Language;

pub mod language;
mod pdf;
pub mod wcif;
mod localhost;
mod compiled;

pub fn print_round_1<I>(args: &mut I) where I: Iterator<Item = String> {
    print_round_1_with_language(args, Language::english());
}

pub fn print_round_1_with_language<I>(args: &mut I, language: Language) where I: Iterator<Item = String> {
    let a = args.next().unwrap();
    let a = std::fs::read_to_string(a).unwrap();
    let b = args.next().unwrap();
    let b = std::fs::read_to_string(b).unwrap();
    let c = args.next().unwrap();
    run(&a, &b, &c, language);
}

pub fn print_subsequent_rounds(competition_id: String) {
    localhost::init(competition_id);
}

#[allow(unused)]
#[deprecated]
pub fn print_event_round(id: &str, event: &str, round: usize, max_group_size: usize) {
    unimplemented!();
}

#[cfg(test)]
mod test {
    #[test]
    fn everything() {
        crate::print_round_1(&mut ["files/OstervangOpen2022stationNumbers.csv", "files/OstervangOpen2022timeLimits.csv", "Østervang Open 2022"].map(|x|x.to_string()).into_iter());
        //crate::print_subsequent_rounds("dastrupsleepover2022".to_string());
    }
}
