use super::{Decrypt, Encrypt, PartialDecrypt, PartialEncrypt};
use crate::meta::HeuristicTarget;
use crate::{Buffer, Char, PartialBuffer};
use rand::Rng;
use simple_error::SimpleError;

#[derive(Debug, Clone)]
pub struct Substitution {
    key: [Char; Char::MAX as usize],
    inverse_key: [Char; Char::MAX as usize],
}

impl Substitution {
    pub fn compute_inverse(key: [Char; Char::MAX as usize]) -> [Char; Char::MAX as usize] {
        let mut out = [Char::from('a'); Char::MAX as usize];

        for (i, &ch) in key.iter().enumerate() {
            out[usize::from(u8::from(ch))] = Char::from(i as u8);
        }

        out
    }

    pub fn from_word(word: &str) -> Self {
        // TODO: This can be cleaned up quite a lot, probably
        let mut duplicate_buffer = [false; Char::MAX as usize];
        let mut buffer = [Char::from('a'); Char::MAX as usize];
        let mut buf_pos = 0;

        for ch in word.chars() {
            if !ch.is_ascii() {
                continue;
            }

            let as_char = Char::from(ch);
            let as_usize = usize::from(u8::from(as_char));

            if !duplicate_buffer[as_usize] {
                duplicate_buffer[as_usize] = true;
                buffer[buf_pos] = Char::from(ch);
                buf_pos += 1;
            }
        }

        let mut dup_buf_pos = 0;
        while buf_pos < Char::MAX as usize {
            while duplicate_buffer[dup_buf_pos] {
                dup_buf_pos += 1;
            }

            buffer[buf_pos] = Char::from(dup_buf_pos as u8);
            buf_pos += 1;
            dup_buf_pos += 1;
        }

        assert!(buf_pos == Char::MAX as usize);

        Self {
            key: buffer.clone(),
            inverse_key: Substitution::compute_inverse(buffer),
        }
    }

    pub fn from_alphabet(alphabet: &str) -> Self {
        assert!(alphabet.len() == Char::MAX as usize);

        let mut buffer = [Char::from('a'); Char::MAX as usize];
        let mut buf_pos = 0;

        for ch in alphabet.chars() {
            buffer[buf_pos] = Char::from(ch);
            buf_pos += 1;
        }

        Self {
            key: buffer.clone(),
            inverse_key: Substitution::compute_inverse(buffer),
        }
    }
}

impl PartialEq for Substitution {
    fn eq(&self, other: &Substitution) -> bool {
        self.key == other.key
    }
}

impl Eq for Substitution {}

impl PartialEncrypt for Substitution {
    fn encrypt_partial(&self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        for x in &mut buf {
            *x = self.key[usize::from(u8::from(*x))];
        }

        Ok(buf)
    }
}

impl PartialDecrypt for Substitution {
    fn decrypt_partial(&self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        for x in &mut buf {
            *x = self.inverse_key[usize::from(u8::from(*x))];
        }

        Ok(buf)
    }
}

derive_encrypt_decrypt!(Substitution, SimpleError);

impl HeuristicTarget for Substitution {
    type KeyParam = ();

    fn rand_key<R: Rng + ?Sized>(_param: Self::KeyParam, rng: &mut R) -> Self {
        let mut buffer = [Char::from('a'); Char::MAX as usize];
        for (i, c) in (0..Char::MAX)
            .map(|_| Char::from(rng.gen_range(0, Char::MAX)))
            .enumerate()
        {
            buffer[i] = c;
        }

        Substitution {
            key: [Char::from('a'); Char::MAX as usize], // We don't need to worry about the encrypting portion of this key
            inverse_key: buffer,
        }
    }

    fn tweak_key<R: Rng + ?Sized>(&self, _param: Self::KeyParam, rng: &mut R) -> Self {
        let mut s = self.clone();
        let c1 = rng.gen_range(0, Char::MAX as usize);
        let c2 = rng.gen_range(0, Char::MAX as usize);
        s.inverse_key.swap(c1, c2);
        s
    }

    fn next_key(_key: Option<Self>, _param: Self::KeyParam) -> Option<Self> {
        unimplemented!() // If you even try and use this, you are too insane
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_word() {
        let sub1 = Substitution::from_word("ZEBRAS");
        let sub2 = Substitution::from_alphabet("ZEBRASCDFGHIJKLMNOPQTUVWXY");
        let sub3 = Substitution::from_word("ZEBRASCDFGHIJKLMNOPQTUVWXY");
        assert_eq!(sub1, sub2);
        assert_eq!(sub2, sub3);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let buf = Buffer::from("Hello world!");

        let subst = Substitution::from_word("ZEBRAS");

        let buf = subst.encrypt(buf).unwrap();
        assert_eq!("Daiil vloir!", buf.to_string());

        let buf = subst.decrypt(buf).unwrap();
        assert_eq!("Hello world!", buf.to_string());
    }

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

        use crate::meta::Metaheuristic;
        use rand::seq::SliceRandom;
        let results = crate::meta::hillclimb::HillClimb::stop_after(10000)
            .crack_ciphertext::<crate::ciphers::Substitution>(
                ciphertext,
                (), // No KeyParam for Caesar ciphers
                crate::score::ScoreMethod::Quadgrams,
                10,
            )
            .unwrap();

        for result in &results {
            println!("{:?}: {}", result.score, result.buf);
        }

        println!("{:?}", plaintext.score(crate::score::ScoreMethod::Quadgrams));

        assert_eq!(results[0].buf, plaintext);
    }
}*/
