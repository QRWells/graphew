pub mod transition;
pub mod state;
pub mod translator;

pub struct StateSpace {
    pub states: Vec<state::State>,
    pub transitions: Vec<transition::Transition>,
}