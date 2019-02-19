use crate::ciphers::{Decrypt, Encrypt};
use crate::score::{Score, ScoreMethod};
use crate::{Buffer, score};
use rand::Rng;
use std::collections::BTreeMap;

pub trait HeuristicTarget: Encrypt + Decrypt + Sized + Clone {
    type KeyParam: Copy; // This might be a key length, range of key lengths, matrix size etc. Differs per cipher

    // Used for stochastic/non-deterministic searching
    fn rand_key<R: Rng + ?Sized>(param: Self::KeyParam, rng: &mut R) -> Self;
    fn tweak_key<R: Rng + ?Sized>(&mut self, param: Self::KeyParam, rng: &mut R);

    // Used for brute force (linear search) - pass 1st param None to get 1st key
    // TODO: Can we use iterators somehow?
    fn next_key(key: Option<Self>, param: Self::KeyParam) -> Option<Self>;
}

pub struct CrackResult<K: HeuristicTarget> {
    buf: Buffer,
    key: K,
}

pub trait Metaheuristic {
    fn crack_ciphertext<T: HeuristicTarget>(
        text: Buffer,
        param: T::KeyParam,
        heuristic: ScoreMethod,
        num_guesses: usize,
    ) -> Result<BTreeMap<Score, CrackResult<T>>, <T as Decrypt>::Error>;
}

pub struct BruteForce;

impl Metaheuristic for BruteForce {
    fn crack_ciphertext<T: HeuristicTarget>(
        text: Buffer,
        param: T::KeyParam,
        heuristic: ScoreMethod,
        num_results: usize,
    ) -> Result<BTreeMap<Score, CrackResult<T>>, <T as Decrypt>::Error> {
        assert!(num_results != 0);

        let mut cur_key = None;
        let mut min_saved_score = score::MIN_SCORE;
        let mut map = BTreeMap::new();

        while let Some(key) = T::next_key(cur_key, param) {
            // encrypt
            let buf = key.decrypt(text.clone())?;

            // see if we ought to store it
            // TODO: Make a custom container for this
            let score = score::score(&buf, heuristic.clone());
            if score > min_saved_score {
                map.insert(score, CrackResult { buf, key: key.clone() });
                if map.len() > num_results {
                    // Remove min score and find next
                    map.remove(&min_saved_score);
                    min_saved_score = *map.iter().next().unwrap().0;
                }
            }
             
            // update next key
            cur_key = Some(key);
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute() {
        let plaintext: Buffer = "SINGLONGHERWAYSIZEWAITEDENDMUTUALMISSEDMYSELFTHELITTLE\
                               SISTERONESOINPOINTEDORCHICKENCHEEREDNEITHERSPIRITSINVI\
                               TEDMARIANNEANDHIMLAUGHTERCIVILITYFORMERLYHANDSOMESEXUS\
                               EPROSPECTHENCEWEDOORSISGIVENRAPIDSCALEABOVEAMDIFFICULT\
                               YEMRDELIVEREDBEHAVIOURBYANIFTHEIRWOMANCOULDDOWOUNDONYO\
                               UFOLLYTASTEHOPEDTHEIRABOVEAREANDBUTATOURSELVESDIRECTIO\
                               NBELIEVINGDOHEDEPARTURECELEBRATEDHERHADSENTIMENTSUNDER\
                               STOODAREPROJECTIONSETPOSSESSIONYENOMRUNAFFECTEDREMARKA\
                               BLYATWROTEHOUSEINNEVERFRUITUPPASTUREIMAGINEMYGARRETSAN\
                               HEHOWEVERDISTANTSHEREQUESTBEHAVEDSEENOTHINGTALKINGSETT\
                               LEDATPLEASEDANOFMEBROTHERWEATHERINONANNOUNCINGIFOFCOMP\
                               ARISONPIANOFORTEPROJECTIONMAIDSHOPEDGAYYETBEDASKEDBLIN\
                               DDRIEDPOINTONABROADDANGERLIKELYREGRETTWENTYEDWARDDOTOO\
                               HORRIBLECONSIDERFOLLOWEDMAYDIFFEREDAGEANRESTIFMOREFIVE\
                               MROFAGEJUSTHERRANKMETDOWNWAYATTENDEDREQUIREDSOINCHEERF\
                               ULANDOMESTICREPLYINGSHERESOLVEDHIMFORDIDRATHERINLASTED"
            .into();

        let ciphertext: Buffer = "ZPUNSVUNOLYDHFZPGLDHPALKLUKTBABHSTPZZLKTFZLSMAOLSPAASL\
                               ZPZALYVULZVPUWVPUALKVYJOPJRLUJOLLYLKULPAOLYZWPYPAZPUCP\
                               ALKTHYPHUULHUKOPTSHBNOALYJPCPSPAFMVYTLYSFOHUKZVTLZLEBZ\
                               LWYVZWLJAOLUJLDLKVVYZPZNPCLUYHWPKZJHSLHIVCLHTKPMMPJBSA\
                               FLTYKLSPCLYLKILOHCPVBYIFHUPMAOLPYDVTHUJVBSKKVDVBUKVUFV\
                               BMVSSFAHZALOVWLKAOLPYHIVCLHYLHUKIBAHAVBYZLSCLZKPYLJAPV\
                               UILSPLCPUNKVOLKLWHYABYLJLSLIYHALKOLYOHKZLUAPTLUAZBUKLY\
                               ZAVVKHYLWYVQLJAPVUZLAWVZZLZZPVUFLUVTYBUHMMLJALKYLTHYRH\
                               ISFHADYVALOVBZLPUULCLYMYBPABWWHZABYLPTHNPULTFNHYYLAZHU\
                               OLOVDLCLYKPZAHUAZOLYLXBLZAILOHCLKZLLUVAOPUNAHSRPUNZLAA\
                               SLKHAWSLHZLKHUVMTLIYVAOLYDLHAOLYPUVUHUUVBUJPUNPMVMJVTW\
                               HYPZVUWPHUVMVYALWYVQLJAPVUTHPKZOVWLKNHFFLAILKHZRLKISPU\
                               KKYPLKWVPUAVUHIYVHKKHUNLYSPRLSFYLNYLAADLUAFLKDHYKKVAVV\
                               OVYYPISLJVUZPKLYMVSSVDLKTHFKPMMLYLKHNLHUYLZAPMTVYLMPCL\
                               TYVMHNLQBZAOLYYHURTLAKVDUDHFHAALUKLKYLXBPYLKZVPUJOLLYM\
                               BSHUKVTLZAPJYLWSFPUNZOLYLZVSCLKOPTMVYKPKYHAOLYPUSHZALK"
            .into();

        let results = BruteForce::crack_ciphertext::<crate::ciphers::Caesar>(
            ciphertext,
            (),
            ScoreMethod::Quadgrams,
            5
        ).unwrap();

        assert!(results.iter().next_back().unwrap().1.buf == plaintext);
    }
}
