use super::game::CharAlignment;
use rand::Rng;

pub trait Mutator {
    fn mutate(&self, cmatch: CharAlignment, rng: &mut impl Rng) -> CharAlignment;
}

#[derive(Default)]
pub struct NoopMutator {}

impl Mutator for NoopMutator {
    fn mutate(&self, cmatch: CharAlignment, _rng: &mut impl Rng) -> CharAlignment {
        cmatch
    }
}

pub struct StepProbMutator {
    exact: f64,
    not_found: f64,
    misplaced: f64,
}

impl StepProbMutator {
    fn new(exact: f64, misplaced: f64, not_found: f64) -> Self {
        Self {
            exact,
            misplaced,
            not_found,
        }
    }
}

impl Default for StepProbMutator {
    fn default() -> Self {
        Self {
            exact: 0.2,
            misplaced: 0.4,
            not_found: 0.2,
        }
    }
}

impl Mutator for StepProbMutator {
    fn mutate(&self, cmatch: CharAlignment, rng: &mut impl Rng) -> CharAlignment {
        let prob = match cmatch {
            CharAlignment::Exact => self.exact,
            CharAlignment::Misplaced => self.misplaced,
            CharAlignment::NotFound => self.not_found,
        };

        let should_mutate = rng.gen_bool(prob);

        if should_mutate {
            match cmatch {
                CharAlignment::Exact => CharAlignment::Misplaced,
                CharAlignment::NotFound => CharAlignment::Misplaced,
                CharAlignment::Misplaced => {
                    if rng.gen_bool(0.5) {
                        CharAlignment::Exact
                    } else {
                        CharAlignment::NotFound
                    }
                }
            }
        } else {
            cmatch
        }
    }
}
