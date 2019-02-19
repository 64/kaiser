use crate::{stats, Buffer};
use float_ord::FloatOrd;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ScoreMethod {
    ChiSquared,
    IOC,
    Quadgrams,
}

// Score: greater is better, lower is worse
// e.g: a > b implies a has better score than b
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Score(FloatOrd<f64>);

pub const MAX_SCORE: Score = Score(FloatOrd(std::f64::INFINITY));
pub const MIN_SCORE: Score = Score(FloatOrd(std::f64::NEG_INFINITY));

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 .0) // uhh
    }
}

pub fn score(buf: &Buffer, heur: ScoreMethod) -> Score {
    match heur {
        ScoreMethod::ChiSquared => Score(FloatOrd(-stats::chi_squared(buf))), // Chi Squared test -> lower is better
        ScoreMethod::IOC => {
            // Negative distance between expected english and given text IOC
            Score(FloatOrd(-(1.73 - stats::index_of_coincidence(buf)).abs()))
        }
        ScoreMethod::Quadgrams => Score(FloatOrd(stats::quadgram_score(buf))),
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
