use crate::{Buffer, Char};
use lazy_static::lazy_static;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

// TODO: Very hacky, find a safer way
lazy_static! {
    static ref QUADGRAMS: Vec<f32> = {
        let len = 26 * 26 * 26 * 26 * 4;
        let mut buf = Vec::with_capacity(len);

        let mut file = File::open("data/quadgram_scores.raw").expect("unable to locate quadgram scores file");
        file.read_to_end(&mut buf).expect("error reading from quadgram file");
        buf.shrink_to_fit();
        debug_assert_eq!(buf.len(), len);

        let out;
        unsafe {
            out = Vec::from_raw_parts(buf.as_mut_ptr() as *mut f32, len, len);
            std::mem::forget(buf);
        };

        out
    };
}

pub fn chi_squared(buf: &Buffer) -> f64 {
    let english_freqs = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
        0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
        0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
    ];

    let freqs = buf.letter_frequencies();
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

pub fn index_of_coincidence(buf: &Buffer) -> f64 {
    let freqs = buf.letter_frequencies();

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
        let hash = (u8::from(*c1) as usize * 26_usize.pow(3)) +
                   (u8::from(*c2) as usize * 26_usize.pow(2)) +
                   (u8::from(*c3) as usize * 26_usize.pow(1)) +
                   (u8::from(*c4) as usize * 26_usize.pow(0));
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
}
