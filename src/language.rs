pub struct Language {
    pub round: String,
    pub group: String,
    pub scram: String,
    pub result: String,
    pub judge: String,
    pub comp: String,
    pub extra_attempts: String,
    pub time_limit: String,
    pub cumulative_limit: String,
    pub for_scl: String,
    pub and_scl: String,
    pub curoff: String,
    pub multi_tl: String,
    pub e333: String,
    pub e444: String,
    pub e555: String,
    pub e666: String,
    pub e777: String,
    pub e222: String,
    pub e333oh: String,
    pub eclock: String,
    pub eminx: String,
    pub epyram: String,
    pub e333bf: String,
    pub e444bf: String,
    pub e555bf: String,
    pub e333mbf: String,
    pub esq1: String,
    pub eskewb: String
}

impl Language {
    pub fn english() -> Self {
        Language { 
            round: format!("Round"), 
            group: format!("Group"), 
            scram: format!("scr"), 
            result: format!("result"), 
            judge: format!("judge"), 
            comp: format!("comp"), 
            extra_attempts: format!("Extra attempts"),
            time_limit: format!("Time limit"), 
            cumulative_limit: format!("Cumulative limit"), 
            for_scl: format!("for"), 
            and_scl: format!("and"), 
            curoff: format!("Two attempts to get below"), 
            multi_tl: format!("10:00 per cube up to 60:00"), 
            e333: format!("3x3x3 Cube"), 
            e444: format!("4x4x4 Cube"), 
            e555: format!("5x5x5 Cube"), 
            e666: format!("6x6x6 Cube"), 
            e777: format!("7x7x7 Cube"), 
            e222: format!("2x2x2 Cube"), 
            e333oh: format!("3x3x3 One Handed"), 
            eclock: format!("Clock"), 
            eminx: format!("Megaminx"), 
            epyram: format!("Pyraminx"), 
            e333bf: format!("3x3x3 Blindfolded"), 
            e444bf: format!("4x4x4 Blindfolded"), 
            e555bf: format!("5x5x5 Blindfolded"), 
            e333mbf: format!("3x3x3 Multi-Blind"), 
            esq1: format!("Square 1"), 
            eskewb: format!("Skewb")
        }
    }
}
