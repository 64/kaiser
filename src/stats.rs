use crate::{Buffer, Char, CharStream};
use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref QUADGRAMS: &'static [f32] = {
        let buf = include_bytes!("../data/quadgram_scores.raw");
        // TODO: Fix potential alignment concern, maybe assert the alignment of buf.as_ptr() ?

        unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const f32, 26 * 26 * 26 * 26) }
    };
}

pub fn letter_frequencies<'a, T: CharStream<'a>>(buf: &'a T) -> [u32; Char::MAX as usize] {
    let mut out = [0; Char::MAX as usize];

    for &b in buf.iter() {
        out[u8::from(b) as usize] += 1;
    }

    out
}

pub fn chi_squared<'a, T: CharStream<'a>>(buf: &'a T) -> f64 {
    let english_freqs = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
        0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
        0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
    ];

    let freqs = letter_frequencies(buf);
    let len_f = buf.len() as f64;

    freqs
        .iter()
        .enumerate()
        .map(|(i, &f)| {
            let e_count = len_f * english_freqs[i];
            let diff = f as f64 - e_count;
            (diff * diff) / e_count
        })
        .sum()
}

pub fn index_of_coincidence<'a, T: CharStream<'a>>(buf: &'a T) -> f64 {
    let freqs = letter_frequencies(buf);

    let total = freqs
        .iter()
        .filter(|&f| *f > 0)
        .map(|&f| f * (f - 1))
        .sum::<u32>() as f64;

    let len = buf.len();
    let denominator = (len * (len - 1)) as f64 / Char::MAX as f64;

    total / denominator
}

pub fn quadgram_score(buf: &Buffer) -> f64 {
    let mut score = 0.0_f64;

    for (c1, c2, c3, c4) in buf.into_iter().tuple_windows() {
        let hash = (u8::from(*c1) as usize * 26_usize.pow(3))
            + (u8::from(*c2) as usize * 26_usize.pow(2))
            + (u8::from(*c3) as usize * 26_usize.pow(1))
            + (u8::from(*c4) as usize * 26_usize.pow(0));
        score += QUADGRAMS[hash] as f64; // TODO: Remove bounds checks
    }

    score / (buf.len() as f64) // Normalise based on text length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristics() {
        let buf = Buffer::from("Rust is the best programming language");

        assert_eq!(1.310483870967742, index_of_coincidence(&buf));
        assert_eq!(29.514280393617323, chi_squared(&buf));
        // TODO: Quadgram score check?
    }

    #[test]
    fn test_freqs() {
        let buf = Buffer::from("Rust is the best programming language");

        let expected = [
            3, 1, 0, 0, 3, 0, 4, 1, 2, 0, 0, 1, 2, 2, 1, 1, 0, 3, 3, 3, 2, 0, 0, 0, 0, 0,
        ];
        assert_eq!(expected, letter_frequencies(&buf));
    }
}
