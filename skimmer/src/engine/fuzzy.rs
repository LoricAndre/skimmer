use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;

use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::item::RankBuilder;
use crate::{CaseMatching, MatchEngine};
use crate::{MatchRange, MatchResult, SkimItem};
use bitflags::_core::cmp::min;

//------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, Default)]
pub enum FuzzyAlgorithm {
    #[default]
    SkimV2,
    SkimV1,
    Clangd,
}

impl FuzzyAlgorithm {
    pub fn of(algorithm: &str) -> Self {
        match algorithm.to_ascii_lowercase().as_ref() {
            "skim_v1" => FuzzyAlgorithm::SkimV1,
            "skim_v2" | "skim" => FuzzyAlgorithm::SkimV2,
            "clangd" => FuzzyAlgorithm::Clangd,
            _ => FuzzyAlgorithm::SkimV2,
        }
    }
}

impl clap::ValueEnum for FuzzyAlgorithm {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::SkimV1, Self::SkimV2, Self::Clangd]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::SkimV1 => Some(clap::builder::PossibleValue::new("skim_v1")),
            Self::SkimV2 => Some(clap::builder::PossibleValue::new("skim_v2")),
            Self::Clangd => Some(clap::builder::PossibleValue::new("clangd")),
        }
    }
}

const BYTES_1M: usize = 1024 * 1024 * 1024;

//------------------------------------------------------------------------------
// Fuzzy engine
#[derive(Default)]
pub struct FuzzyEngineBuilder {
    query: String,
    case: CaseMatching,
    algorithm: FuzzyAlgorithm,
    rank_builder: Arc<RankBuilder>,
}

impl FuzzyEngineBuilder {
    pub fn query(mut self, query: &str) -> Self {
        self.query = query.to_string();
        self
    }

    pub fn case(mut self, case: CaseMatching) -> Self {
        self.case = case;
        self
    }

    pub fn algorithm(mut self, algorithm: FuzzyAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn rank_builder(mut self, rank_builder: Arc<RankBuilder>) -> Self {
        self.rank_builder = rank_builder;
        self
    }

    #[allow(deprecated)]
    pub fn build(self) -> FuzzyEngine {
        use fuzzy_matcher::skim::SkimMatcher;
        let matcher: Box<dyn FuzzyMatcher> = match self.algorithm {
            FuzzyAlgorithm::SkimV1 => Box::new(SkimMatcher::default()),
            FuzzyAlgorithm::SkimV2 => {
                let matcher = SkimMatcherV2::default().element_limit(BYTES_1M);
                let matcher = match self.case {
                    CaseMatching::Respect => matcher.respect_case(),
                    CaseMatching::Ignore => matcher.ignore_case(),
                    CaseMatching::Smart => matcher.smart_case(),
                };
                Box::new(matcher)
            }
            FuzzyAlgorithm::Clangd => {
                let matcher = ClangdMatcher::default();
                let matcher = match self.case {
                    CaseMatching::Respect => matcher.respect_case(),
                    CaseMatching::Ignore => matcher.ignore_case(),
                    CaseMatching::Smart => matcher.smart_case(),
                };
                Box::new(matcher)
            }
        };

        FuzzyEngine {
            matcher,
            query: self.query,
            rank_builder: self.rank_builder,
        }
    }
}

pub struct FuzzyEngine {
    query: String,
    matcher: Box<dyn FuzzyMatcher>,
    rank_builder: Arc<RankBuilder>,
}

impl FuzzyEngine {
    pub fn builder() -> FuzzyEngineBuilder {
        FuzzyEngineBuilder::default()
    }

    fn fuzzy_match(&self, choice: &str, pattern: &str) -> Option<(i64, Vec<usize>)> {
        if pattern.is_empty() {
            return Some((0, Vec::new()));
        } else if choice.is_empty() {
            return None;
        }

        self.matcher.fuzzy_indices(choice, pattern)
    }
}

impl MatchEngine for FuzzyEngine {
    fn match_item(&self, item: Arc<dyn SkimItem>) -> Option<MatchResult> {
        // iterate over all matching fields:
        let mut matched_result = None;
        let item_text = item.text();
        let default_range = [(0, item_text.len())];
        for &(start, end) in item.get_matching_ranges().unwrap_or(&default_range) {
            let start = min(start, item_text.len());
            let end = min(end, item_text.len());
            matched_result = self.fuzzy_match(&item_text[start..end], &self.query).map(|(s, vec)| {
                if start != 0 {
                    let start_char = &item_text[..start].chars().count();
                    (s, vec.iter().map(|x| x + start_char).collect())
                } else {
                    (s, vec)
                }
            });

            if matched_result.is_some() {
                break;
            }
        }

        matched_result.as_ref()?;

        let (score, matched_range) = matched_result.unwrap();

        let begin = *matched_range.first().unwrap_or(&0);
        let end = *matched_range.last().unwrap_or(&0);

        let item_len = item_text.len();
        Some(MatchResult {
            rank: self.rank_builder.build_rank(score as i32, begin, end, item_len),
            matched_range: MatchRange::Chars(matched_range),
        })
    }
}

impl Display for FuzzyEngine {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(Fuzzy: {})", self.query)
    }
}