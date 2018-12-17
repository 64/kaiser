use crate::{Buffer, Char};

pub enum Heuristic {
    ChiSquared,
    IOC,
}

// HeuristicScore: greater is better, lower is worse
// e.g: a > b implies a has better score than b
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct HeuristicScore(f64);

pub const MAX_SCORE: HeuristicScore = HeuristicScore(std::f64::INFINITY);
pub const MIN_SCORE: HeuristicScore = HeuristicScore(std::f64::NEG_INFINITY);

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

pub fn score(buf: &Buffer, heur: Heuristic) -> HeuristicScore {
    match heur {
        Heuristic::ChiSquared => HeuristicScore(-chi_squared(buf)), // Chi Squared test -> lower is better
        Heuristic::IOC => {
            // Negative distance between expected english and given text IOC
            HeuristicScore(-(1.73 - index_of_coincidence(buf)).abs())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristics() {
        let buf = Buffer::from("Rust is the best programming language");

        assert_eq!(1.310483870967742, index_of_coincidence(&buf));
        assert_eq!(29.514280393617323, chi_squared(&buf));
    }

    #[test]
    fn test_scores() {
        let gibberish: Buffer = "WKJGGTOLIOTBBZXFPJCRDNCWWIAODDBRPFPSIAVEIXVPKTAFDFBVPF\
                                 ZEWWBLNMZSLZFIWKFUWGOIDMFGMYVITNKLIISPJCMMHROJQWPNXJOZ\
                                 PQTNIRULXXKXBACQMYWXFIDTLAZTKQTOYCRXEGKUHKMUHUCTZHCWIK\
                                 AJNRRPARZXBWDWRMVZNLFXEBBMTHEQMUHRIOEQNELQIGGGGJYNZSLY\
                                 FXGTXUGOOBEUCSWGTFFRIBDVAKDHWVKNFKWJNLSIRIZERYDFDXKEGH\
                                 VTYMMESSAHEUIQXCGLETPGGTIYUUNPMFRNLMBKKAGEUDZMCTJIJDVR\
                                 UUGZKFNANUHCXCTXGLSEGFPELPLUXLXNDTRZKHGQSGVIAMAFCWYVJG\
                                 APGZLQXMTGRUJWUAZQXAAURTBGYWNAZUWOEIKXTGBTEPMYBBKKSEZK\
                                 XRPPMFXDLUQAHBHGYZUOVZCXLRDHLKSQZAELIGKKRCOMWZQPHIXZLW\
                                 RGANGJEONGASRVOGZUKGRPTMYSSTGMMZRFKVIBGTPMZLIIVAVVRCPJ\
                                 DBWVEQCMUZTARLMQCDMZQIMAIRXOEZIEHJRAUDTKBTCHOPPDPEQTUB\
                                 PMPMVEUHTKVJLZVCRQRXIMXKAGLVEVWODZCHOMQXEOJTPAOUWQCGKH\
                                 JTVJWOPLKGEDZMOGBFCDYRKQSTEXFDUUCAORLUJLLFEBRNCZCEVJZM\
                                 KPCYFGNYNOEYSQERPBGYAQAPLGGYWCOLSISSRWBDGCRHLTEGVIAQID\
                                 EQKODHJHZMFOJHKFGJEXMZONMQKNNDRNPZPXHMANOKLFRHHYPVGWLL\
                                 RNRNKVRSIGOZWAWPKQDJMVSWHSRRHBTOOIVOKQNLOUYCYAXFNVHDDA"
            .into();

        let english: Buffer = "SINGLONGHERWAYSIZEWAITEDENDMUTUALMISSEDMYSELFTHELITTLE\
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

        // Check that our mins and max are in order
        assert!(gibberish.score(Heuristic::IOC) > MIN_SCORE);
        assert!(english.score(Heuristic::IOC) < MAX_SCORE);
        assert!(MIN_SCORE < MAX_SCORE);

        // Check that our heuristics are giving sane results
        assert!(gibberish.score(Heuristic::IOC) < english.score(Heuristic::IOC));
        assert!(gibberish.score(Heuristic::ChiSquared) < english.score(Heuristic::ChiSquared));
    }
}
