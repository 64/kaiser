use crate::meta::{CrackResults, Metaheuristic, HeuristicTarget};
use crate::score::ScoreMethod;
use crate::ciphers::Decrypt;
use crate::Buffer;
use rand::thread_rng;
use std::num::NonZeroUsize;

pub struct HillClimb {
    stop_after: NonZeroUsize,
}

impl HillClimb {
    pub fn stop_after(stop_after: usize) -> Self {
        Self {
            stop_after: NonZeroUsize::new(stop_after).expect("stop_after 0 provided")
        }
    }
}

impl Metaheuristic for HillClimb {
    fn crack_ciphertext<T: HeuristicTarget>(
        &mut self,
        text: Buffer,
        param: T::KeyParam,
        score_method: ScoreMethod,
        num_results: usize,
    ) -> Result<CrackResults<T>, <T as Decrypt>::Error> {
        let mut results = CrackResults::new(num_results);
        let mut rng = thread_rng();
        let mut iters_since_change = 0;

        let (mut parent, mut parent_score) = {
            let key = T::rand_key(param, &mut rng);
            let buf = key.decrypt(text.clone())?;
            (key.clone(), results.process_result(buf, key, score_method))
        };

        while iters_since_change < self.stop_after.get() {
            let key = parent.tweak_key(param, &mut rng);
            let buf = key.decrypt(text.clone())?;
            let score = results.process_result(buf, key.clone(), score_method);

            if score > parent_score {
                parent_score = score;
                parent = key;
                iters_since_change = 0;
            }

            iters_since_change += 1;
        }

        Ok(results)
    }
}