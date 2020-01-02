use std::fs::File;
use serde::{Serialize, Deserialize};

use super::rule::Rule;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Struct to describe an L-System and it's various components
pub struct LSystem {
    // Name of the lsystem, used when writing out files
    pub name: String,
    // Axiom, or beginning seed of the system
    pub axiom: String,
    // Vec of rules that describe the transformations that can happen to the axiom
    pub rules: Vec<Rule>,
    // Turn angle for the turtle
    pub turn_angle: i64,
    // Initial angle for the turtle
    pub init_angle: i64,
    #[serde(default)]
    // The final string that contains the symbols to be interpreted by the turtle
    // we declare to the serde framework that this gets a default value since it will
    // only be populated after the system has been run.
    pub endstring: String
}

impl LSystem {
    #[allow(dead_code)]
    /// Returns a new lsystem object given a few initial variables 
    /// This is a convenience a
    pub fn new(name: &str, axiom: &str, turn_angle: i64) -> LSystem {
        LSystem {
            name: name.to_string(),
            axiom: axiom.to_string(),
            rules: Vec::new(),
            turn_angle: turn_angle,
            init_angle: 0,
            endstring: String::from("")
        }
    }

    pub fn from_yml(filename: &str) -> Result<LSystem, serde_yaml::Error> {
        let deserialized_lsystem: LSystem = serde_yaml::from_reader(File::open(filename).unwrap())?;
        Ok(deserialized_lsystem)
    }
}

impl LSystem {

    fn apply_rule(&self, input: &char) -> String {
        let mut output = String::from("");
        for rule in self.rules.iter() {
            if rule.is_match(input) {
                output = rule.get_successor().to_string();
                break;
            }
            else {
                output = input.to_string();
            }
        }
        return output;
    }

    pub fn generate_system(&mut self, num_iterations: i32) {
        self.endstring = self.axiom.clone();
        for i in 0..num_iterations {
            println!("Iteration: {}", i);
            self.endstring = self.process_string(&self.endstring);
        }
    }

    fn process_string(&self, string: &str) -> String {
        let mut new_string = String::from("");
        for character in string.chars() {
            new_string.push_str(&self.apply_rule(&character));
        }
        return new_string
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::lsystem::rule::StochasticSuccessor;

    #[test]
    fn create_lsystem() {
        let system = LSystem::new("Test", "A", 90);
        assert_eq!(system.axiom, "A");
        assert_eq!(system.turn_angle, 90);
        assert_eq!(system.rules.is_empty(), true);
    }

    #[test]
    fn test_apply_rule() {
        let mut system = LSystem::new("Test", "A", 90);
        system.rules.push(
            Rule::Simple{
                predecessor: 'A',
                successor: String::from("AB")
            }
        );

        let output = system.apply_rule(&'A');
        assert_eq!(output, "AB");
    }

    #[test]
    fn test_apply_stochactic_rule() {
        let mut system = LSystem::new("Test", "A", 90);
        system.rules.push(
            Rule::Stochastic{
                predecessor: 'A',
                successors: vec!(
                    StochasticSuccessor{
                        probability: 1.0,
                        successor: "AB".to_string()
                    }
                )
            }
        );

        let output = system.apply_rule(&'A');
        assert_eq!(output, "AB");
    }

    #[test]
    fn test_process_string() {
        let mut system = LSystem::new("Test", "A", 90);
        system.rules.push(
            Rule::Simple{
                predecessor: 'A',
                successor: String::from("AB")
            }
        );
        system.rules.push(
            Rule::Simple{
                predecessor: 'B',
                successor: String::from("BA")
            }
        );


        let input = String::from("AB"); 
        let output = system.process_string(&input);
        assert_eq!(output, "ABBA");
    }

    #[test]
    fn test_generate_system() {
        let mut system = LSystem::new("Test", "AB", 90);
        system.rules.push(
            Rule::Simple{
                predecessor: 'A',
                successor: String::from("AB")
            }
        );
        system.rules.push(
            Rule::Simple{
                predecessor: 'B',
                successor: String::from("BA")
            }
        );

        system.generate_system(3);
        assert_eq!(system.endstring, "ABBABAABBAABABBA");
    }
}