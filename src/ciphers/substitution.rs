use super::{Decrypt, Encrypt, PartialDecrypt, PartialEncrypt};
use crate::meta::HeuristicTarget;
use crate::{Buffer, Char, PartialBuffer};
use rand::{seq::SliceRandom, Rng};
use simple_error::SimpleError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Substitution {
    key: [Char; Char::MAX as usize],
    encrypt_mode: bool,
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
            key: buffer,
            encrypt_mode: true,
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
            key: buffer,
            encrypt_mode: true,
        }
    }
}

impl PartialEq for Substitution {
    fn eq(&self, other: &Substitution) -> bool {
        if self.encrypt_mode == other.encrypt_mode {
            self.key == other.key
        } else {
            self.key == Substitution::compute_inverse(other.key.clone())
        }
    }
}

impl Eq for Substitution {}

impl PartialEncrypt for Substitution {
    fn encrypt_partial(&mut self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        if !self.encrypt_mode {
            self.encrypt_mode = true;
            self.key = Substitution::compute_inverse(self.key);
        }

        for x in &mut buf {
            *x = self.key[usize::from(u8::from(*x))];
        }

        Ok(buf)
    }
}

impl PartialDecrypt for Substitution {
    fn decrypt_partial(&mut self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        if self.encrypt_mode {
            self.encrypt_mode = false;
            self.key = Substitution::compute_inverse(self.key);
        }

        for x in &mut buf {
            *x = self.key[usize::from(u8::from(*x))];
        }

        Ok(buf)
    }
}

derive_encrypt_decrypt!(Substitution, SimpleError);

impl fmt::Display for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let true_key = if self.encrypt_mode {
            self.key
        } else {
            Substitution::compute_inverse(self.key)
        };

        let out = true_key.iter().map(|&c| char::from(c)).collect::<String>();
        write!(f, "{}", out)
    }
}

impl HeuristicTarget for Substitution {
    type KeyParam = ();

    fn rand_key<R: Rng + ?Sized>(_param: Self::KeyParam, rng: &mut R) -> Self {
        let mut buffer = [
            Char::from('A'),
            Char::from('B'),
            Char::from('C'),
            Char::from('D'),
            Char::from('E'),
            Char::from('F'),
            Char::from('G'),
            Char::from('H'),
            Char::from('I'),
            Char::from('J'),
            Char::from('K'),
            Char::from('L'),
            Char::from('M'),
            Char::from('N'),
            Char::from('O'),
            Char::from('P'),
            Char::from('Q'),
            Char::from('R'),
            Char::from('S'),
            Char::from('T'),
            Char::from('U'),
            Char::from('V'),
            Char::from('W'),
            Char::from('X'),
            Char::from('Y'),
            Char::from('Z'),
        ];

        buffer.shuffle(rng);

        Substitution {
            key: buffer,
            encrypt_mode: false,
        }
    }

    fn tweak_key<R: Rng + ?Sized>(&self, _param: Self::KeyParam, rng: &mut R) -> Self {
        let mut s = self.clone();
        let c1 = rng.gen_range(0, Char::MAX as usize);
        let c2 = rng.gen_range(0, Char::MAX as usize);
        s.key.swap(c1, c2);
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

        let mut subst = Substitution::from_word("ZEBRAS");

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
        let results = crate::meta::hillclimb::HillClimb::new(1000, 500)
            .crack_ciphertext::<crate::ciphers::Substitution>(
                ciphertext,
                (), // No KeyParam
                crate::score::ScoreMethod::Quadgrams,
                10,
            )
            .unwrap();

        for result in &results {
            println!("{:?} - {}: {}", result.score, result.key, result.buf);
        }

        println!("{:?}", plaintext.score(crate::score::ScoreMethod::Quadgrams));

        assert!(results[0].buf != plaintext);
    }
}*/
