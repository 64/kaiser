use crate::Buffer;
use crate::ciphers::Decrypt;
use crate::score::ScoreMethod;
use super::{Metaheuristic, HeuristicTarget, CrackResults};

pub struct BruteForce;

impl BruteForce {
    pub fn new() -> Self { Self }
}

impl Metaheuristic for BruteForce {
    fn crack_ciphertext<T: HeuristicTarget>(
        &mut self,
        text: Buffer,
        param: T::KeyParam,
        score_method: ScoreMethod,
        num_results: usize,
    ) -> Result<CrackResults<T>, <T as Decrypt>::Error> {

        let mut cur_key = None;
        let mut results = CrackResults::new(num_results);

        while let Some(key) = T::next_key(cur_key, param) {
            let buf = key.decrypt(text.clone())?;

            // Store it if we need
            results.process_result(buf, key.clone(), score_method);

            // Update
            cur_key = Some(key);
        }

        Ok(results)
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

        let results = BruteForce::new().crack_ciphertext::<crate::ciphers::Caesar>(
            ciphertext,
            (), // No KeyParam for Caesar ciphers
            ScoreMethod::Quadgrams,
            10,
        )
        .unwrap();

        assert_eq!(results[0].buf, plaintext);
    }
}
