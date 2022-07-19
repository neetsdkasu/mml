// mml-core
// author: Leonardone @ NEETSDKASU

mod inst;
mod midi;
mod mml;
mod tone_control;

pub use inst::*;
pub use mml::MMLError;

pub fn convert(src: &str, inst: Instrument) -> Result<Vec<u8>, MMLError> {
    let tseq = mml::parse(src)?;
    let inst: i8 = (inst as i32 - 1) as i8;
    assert!(0 <= inst, "inst {}", inst);
    midi::translate(&tseq, inst).map_err(MMLError::IoError)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUMM_SUMM_SUMM: &str = r#"
        {0 O5 L4 D C > B R }
        {1 O4 L8 A B < C > A G4 R4 }
        {2 $0 $1 }
        $2 [2 O4 L8 B < C D > B A B < C > A ] $2
    "#;

    const KUCKUCK_KUCKUCK_RUFTS_AUS_DEM_WALD: &str = r#"
        T150%96{0GFGF2.}O5[2C>AR]$0GGAB-2GAAB-<[3C2>A]$0
    "#;

    const MORGEN_KOMMT_DER_WEIHNACHTSMANN: &str = r#"
        T104
        {0
            L8
            FF<CC
            DDC4
            >B-B-AA
            G4FR
        }
        $0
        [2
            L8
            <CC>B-B-
            AAG4
        ]
        $0    
    "#;

    #[test]
    fn it_works() {
        let inst = Instrument::AcousticGrandPiano;

        let res1 = convert(SUMM_SUMM_SUMM, inst);
        assert!(res1.is_ok(), "{:?}", res1);

        let res2 = convert(KUCKUCK_KUCKUCK_RUFTS_AUS_DEM_WALD, inst);
        assert!(res2.is_ok(), "{:?}", res2);

        let res3 = convert(MORGEN_KOMMT_DER_WEIHNACHTSMANN, inst);
        assert!(res3.is_ok(), "{:?}", res3);

        let inst = Instrument::Gunshot;

        let res1 = convert(SUMM_SUMM_SUMM, inst);
        assert!(res1.is_ok(), "{:?}", res1);

        let res2 = convert(KUCKUCK_KUCKUCK_RUFTS_AUS_DEM_WALD, inst);
        assert!(res2.is_ok(), "{:?}", res2);

        let res3 = convert(MORGEN_KOMMT_DER_WEIHNACHTSMANN, inst);
        assert!(res3.is_ok(), "{:?}", res3);
    }
}
