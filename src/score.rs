use crate::{stats, Buffer};

pub enum ScoreMethod {
    ChiSquared,
    IOC,
    Quadgrams,
}

// ScoreScore: greater is better, lower is worse
// e.g: a > b implies a has better score than b
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Score(f64);

pub const MAX_SCORE: Score = Score(std::f64::INFINITY);
pub const MIN_SCORE: Score = Score(std::f64::NEG_INFINITY);

pub fn score(buf: &Buffer, heur: ScoreMethod) -> Score {
    match heur {
        ScoreMethod::ChiSquared => Score(-stats::chi_squared(buf)), // Chi Squared test -> lower is better
        ScoreMethod::IOC => {
            // Negative distance between expected english and given text IOC
            Score(-(1.73 - stats::index_of_coincidence(buf)).abs())
        }
        ScoreMethod::Quadgrams => Score(stats::quadgram_score(buf)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(gibberish.score(ScoreMethod::IOC) > MIN_SCORE);
        assert!(english.score(ScoreMethod::IOC) < MAX_SCORE);
        assert!(MIN_SCORE < MAX_SCORE);

        // Check that our heuristics are giving sane results
        assert!(gibberish.score(ScoreMethod::IOC) < english.score(ScoreMethod::IOC));
        assert!(gibberish.score(ScoreMethod::ChiSquared) < english.score(ScoreMethod::ChiSquared));
        assert!(gibberish.score(ScoreMethod::Quadgrams) < english.score(ScoreMethod::Quadgrams));
    }
}
