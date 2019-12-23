use serde::{Serialize, Deserialize};
use rand::prelude::*;
// pub trait Rule {
//     fn is_match(&self, predecessor: &char) -> bool;
//     fn get_successor(&self, predecessor: &char) -> String;
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag="ruletype")]
pub enum Rule {
    #[serde(rename="simple")]
    Simple{predecessor: char, successor: String},
    #[serde(rename="stochastic")]
    Stochastic{predecessor: char, successors: Vec<StochasticSuccessor>},
}

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct SimpleRule {
//     pub predecessor: char,
//     pub successor: String
// }

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct StochasticRule {
//     pub predecessor: char,
//     pub successors: Vec<StochasticSuccessor>
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StochasticSuccessor {
    pub probability: f64,
    pub successor: String
}

impl Rule {
    pub fn is_match(&self, p: &char) -> bool {
        match *self {
            Rule::Simple {ref predecessor, ref successor} => predecessor == p,
            Rule::Stochastic {ref predecessor, ref successors} => predecessor == p
        }
    }

    pub fn get_successor(&self) -> String {
        match *self {
            Rule::Simple {ref predecessor, ref successor} => successor.to_string(),
            Rule::Stochastic {ref predecessor, ref successors} => {
                let mut rng = thread_rng();
                successors.choose_weighted(&mut rng, |item| item.probability).unwrap().successor.to_string()
            }
        }
    }
}

