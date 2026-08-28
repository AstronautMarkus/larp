pub mod packages;

use crate::theatrics;

pub enum Step {
    Line(String),
    Blank,
    Spinner(String, u64, u64),
    Progress(String),
}

#[derive(Default)]
pub struct Script {
    pub steps: Vec<Step>,
}

impl Script {
    pub fn new() -> Self {
        Script::default()
    }

    pub fn line(mut self, s: impl Into<String>) -> Self {
        self.steps.push(Step::Line(s.into()));
        self
    }

    pub fn blank(mut self) -> Self {
        self.steps.push(Step::Blank);
        self
    }

    pub fn spinner(mut self, s: impl Into<String>, min_ms: u64, max_ms: u64) -> Self {
        self.steps.push(Step::Spinner(s.into(), min_ms, max_ms));
        self
    }

    pub fn progress(mut self, s: impl Into<String>) -> Self {
        self.steps.push(Step::Progress(s.into()));
        self
    }
}

pub fn run(script: &Script) {
    for step in &script.steps {
        match step {
            Step::Line(s) => theatrics::line(s),
            Step::Blank => theatrics::blank(),
            Step::Spinner(s, min, max) => theatrics::spinner(s, *min, *max),
            Step::Progress(s) => theatrics::progress_bar(s),
        }
    }
}
