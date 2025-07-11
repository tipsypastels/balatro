use crate::ScoreBuilder;
use rand::RngCore;

pub struct RunIndependentEvent<'a> {
    pub rng: &'a mut dyn RngCore,
    pub score: &'a mut ScoreBuilder,
}
