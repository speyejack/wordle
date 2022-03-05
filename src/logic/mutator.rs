use super::CharAlignment;
use rand::{prelude::StdRng, thread_rng, Rng, SeedableRng};

pub trait Mutator {
    fn mutate(&mut self, cmatch: CharAlignment) -> CharAlignment;
}

#[derive(Default)]
pub struct NoopMutator {}

impl Mutator for NoopMutator {
    fn mutate(&mut self, cmatch: CharAlignment) -> CharAlignment {
        cmatch
    }
}

pub struct StepProbMutator<R>
where
    R: Rng,
{
    rng: R,
    exact: f64,
    not_found: f64,
    misplaced: f64,
}

impl<R> StepProbMutator<R>
where
    R: Rng,
{
    fn new(rng: R, exact: f64, misplaced: f64, not_found: f64) -> Self {
        Self {
            rng,
            exact,
            misplaced,
            not_found,
        }
    }
}

impl Default for StepProbMutator<StdRng> {
    fn default() -> Self {
        Self::new(
            StdRng::from_rng(thread_rng())
                .expect("Step probability mutator failed to initalize rng"),
            0.2,
            0.4,
            0.2,
        )
    }
}

impl<R> Mutator for StepProbMutator<R>
where
    R: Rng,
{
    fn mutate(&mut self, cmatch: CharAlignment) -> CharAlignment {
        let prob = match cmatch {
            CharAlignment::Exact => self.exact,
            CharAlignment::Misplaced => self.misplaced,
            CharAlignment::NotFound => self.not_found,
        };

        let should_mutate = self.rng.gen_bool(prob);

        if should_mutate {
            match cmatch {
                CharAlignment::Exact => CharAlignment::Misplaced,
                CharAlignment::NotFound => CharAlignment::Misplaced,
                CharAlignment::Misplaced => {
                    if self.rng.gen_bool(0.5) {
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
