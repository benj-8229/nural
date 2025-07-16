use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn score_options(options: Vec<String>, query: String) -> Vec<(String, i64)> {
    let mut results: Vec<(String, i64)> = Vec::new();
    let matcher = SkimMatcherV2::default();

    for option in options {
        let score;
        match matcher.fuzzy_match(&option, &query) {
            Some(rank) => { score = rank; },
            None => { score = -1; }
        };
         results.push((option.clone().to_string(), score))
    }

    results.sort_by(|a, b| b.1.cmp(&a.1));
    results
}
