use crate::{Buffer, stats};

pub enum Heuristic {
    ChiSquared,
    IOC,
    Quadgrams,
}

// HeuristicScore: greater is better, lower is worse
// e.g: a > b implies a has better score than b
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct HeuristicScore(f64);

pub const MAX_SCORE: HeuristicScore = HeuristicScore(std::f64::INFINITY);
pub const MIN_SCORE: HeuristicScore = HeuristicScore(std::f64::NEG_INFINITY);

pub fn score(buf: &Buffer, heur: Heuristic) -> HeuristicScore {
    match heur {
        Heuristic::ChiSquared => HeuristicScore(-stats::chi_squared(buf)), // Chi Squared test -> lower is better
        Heuristic::IOC => {
            // Negative distance between expected english and given text IOC
            HeuristicScore(-(1.73 - stats::index_of_coincidence(buf)).abs())
        }
        Heuristic::Quadgrams => HeuristicScore(stats::quadgram_score(buf)),
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
        assert!(gibberish.score(Heuristic::IOC) > MIN_SCORE);
        assert!(english.score(Heuristic::IOC) < MAX_SCORE);
        assert!(MIN_SCORE < MAX_SCORE);

        // Check that our heuristics are giving sane results
        assert!(gibberish.score(Heuristic::IOC) < english.score(Heuristic::IOC));
        assert!(gibberish.score(Heuristic::ChiSquared) < english.score(Heuristic::ChiSquared));
        assert!(gibberish.score(Heuristic::Quadgrams) < english.score(Heuristic::Quadgrams));
    }
}
