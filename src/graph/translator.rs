use super::StateSpace;

pub trait Translator {
    fn translate(&self) -> StateSpace;
}

pub struct SPINTranslator {
    pub states: Vec<String>,
    pub transitions: Vec<String>,
}

impl Translator for SPINTranslator {
    fn translate(&self) -> StateSpace {
        todo!()
    }
}

pub struct SLIMTranslator {
    pub states: Vec<String>,
    pub transitions: Vec<String>,
}

impl Translator for SLIMTranslator {
    fn translate(&self) -> StateSpace {
        todo!()
    }
}