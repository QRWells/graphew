use crate::graph::{state::State, transition::Transition};

use super::StateSpace;

pub trait Translator {
    type ErrorType;
    fn translate(str: &str) -> Result<StateSpace, Self::ErrorType>;
}

pub struct SPINTranslator {
    pub states: Vec<String>,
    pub transitions: Vec<String>,
}

impl Translator for SPINTranslator {
    type ErrorType = ();
    fn translate(str: &str) -> Result<StateSpace, ()> {
        todo!()
    }
}

pub struct SLIMTranslator {
    pub states: Vec<String>,
    pub transitions: Vec<String>,
}

impl Translator for SLIMTranslator {
    type ErrorType = ();
    fn translate(str: &str) -> Result<StateSpace, ()> {
        let mut lines = str.lines().into_iter().peekable();
        loop {
            if let Some(line) = lines.next() {
                if line.trim().starts_with("States") {
                    break;
                }
            }
        }
        // check if there is a next line but don't consume it
        if lines.peek().is_none() {
            return Err(());
        }
        let mut nodes = vec![];
        loop {
            if let Some(line) = lines.next() {
                let states = line.split("::").collect::<Vec<&str>>();
                if states.len() < 2 {
                    break;
                }
                let id = states[0].parse::<usize>().unwrap();
                let state = states[1..].join("::");
                nodes.push(State {
                    index: id,
                    info: state,
                });
            } else {
                break;
            }
        }
        loop {
            if let Some(line) = lines.next() {
                if line.trim().starts_with("Transitions") {
                    break;
                }
            }
        }
        if lines.next().is_none() {
            return Err(());
        }
        let mut edges = vec![];
        loop {
            if let Some(line) = lines.next() {
                let states = line.split("::").collect::<Vec<&str>>();
                if states.len() < 2 {
                    break;
                }
                let from = states[0].parse::<usize>().unwrap();
                let joins = states[1..].join("::");
                let targets = joins.split(',').collect::<Vec<&str>>();
                if (targets.len() == 1) && targets[0].is_empty() {
                    continue;
                }
                for target in targets {
                    let to = target.parse::<usize>().unwrap();
                    edges.push(Transition { from, to });
                }
            } else {
                break;
            }
        }
        Ok(StateSpace {
            states: nodes,
            transitions: edges,
        })
    }
}
