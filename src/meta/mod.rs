use crate::ciphers::Decrypt;
use crate::score::{Score, ScoreMethod};
use crate::Buffer;
use rand::Rng;
use std::ops::Index;

pub mod brute;
pub mod hillclimb;

pub trait HeuristicTarget: Decrypt + Sized + Clone + PartialEq {
    type KeyParam: Copy; // This might be a key length, range of key lengths, matrix size etc. Differs per cipher

    // Used for stochastic/non-deterministic searching
    fn rand_key<R: Rng + ?Sized>(param: Self::KeyParam, rng: &mut R) -> Self;
    fn tweak_key<R: Rng + ?Sized>(&self, param: Self::KeyParam, rng: &mut R) -> Self;

    // Used for brute force (linear search) - pass 1st param None to get initial key
    // TODO: Can we use iterators somehow? Better API
    fn next_key(key: Option<Self>, param: Self::KeyParam) -> Option<Self>;
}

// TODO: Display?
pub struct CrackResult<K: HeuristicTarget> {
    pub score: Score,
    pub buf: Buffer,
    pub key: K,
}

pub struct CrackResults<K: HeuristicTarget> {
    data: Vec<CrackResult<K>>,
    results: usize,
}

pub trait Metaheuristic {
    fn crack_ciphertext<T: HeuristicTarget>(
        &mut self,
        text: Buffer,
        param: T::KeyParam,
        score_method: ScoreMethod,
        num_results: usize,
    ) -> Result<CrackResults<T>, <T as Decrypt>::Error>;
}

impl<K: HeuristicTarget> CrackResults<K> {
    pub fn new(num_results: usize) -> Self {
        assert!(num_results > 0, "num_results was zero");

        CrackResults {
            data: Vec::new(),
            results: num_results,
        }
    }

    pub fn process_result(&mut self, buf: Buffer, key: K, method: ScoreMethod) -> Score {
        let score = buf.score(method);

        let min_score = self
            .data
            .last()
            .map(|cr| cr.score)
            .unwrap_or(crate::score::MIN_SCORE);

        if score > min_score {
            // Remove the lowest scoring item if we're out of space
            if self.results == self.data.len() {
                self.data.pop();
            }

            // Find insertion point
            let insert_pos = match self.data.binary_search_by(|cr| score.cmp(&cr.score)) {
                Ok(pos) => {
                    if self.data[pos].key == key {
                        None // Don't insert duplicates
                    } else {
                        Some(pos)
                    }
                }
                Err(pos) => Some(pos),
            };

            if let Some(insert_pos) = insert_pos {
                self.data
                    .insert(insert_pos, CrackResult { buf, key, score });
            }
        }

        score
    }
}

impl<'a, K: HeuristicTarget> IntoIterator for &'a CrackResults<K> {
    type Item = &'a CrackResult<K>;
    type IntoIter = std::slice::Iter<'a, CrackResult<K>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<K: HeuristicTarget> Index<usize> for CrackResults<K> {
    type Output = CrackResult<K>;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}
