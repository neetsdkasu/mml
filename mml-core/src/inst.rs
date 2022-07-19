// mml-core
// author: Leonardone @ NEETSDKASU

use std::sync::Once;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instrument {
    // Piano
    AcousticGrandPiano = 1,
    BrightAcousticPiano,
    ElectricGrandPiano,
    HonkyTonkPiano,
    ElectricPiano1,
    ElectricPiano2,
    Harpsichord,
    Clavinet,

    // Chromatic Precussion
    Celesta,
    Glockenspiel,
    MusicBox,
    Vibraphone,
    Marimba,
    Xylophone,
    TubularBells,
    Dulcimer,

    // Organ
    DrawbarOrgan,
    PercussiveOrgan,
    RockOrgan,
    ChurchOrgan,
    ReedOrgan,
    Accordion,
    Harmonica,
    TangoAccordion,

    // Guitar
    AcousticGuitarNylon,
    AcousticGuitarSteel,
    ElectricGuitarJazz,
    ElectricGuitarClean,
    ElectricGuitarMuted,
    ElectricGuitarOverdriven,
    ElectricGuitarDistortion,
    ElectricGuitarHarmonics,

    // Bass
    AcousticBass,
    ElectricBassFinger,
    ElectricBassPicked,
    FretlessBass,
    SlapBass1,
    SlapBass2,
    SynthBass1,
    SynthBass2,

    // Strings
    Violin,
    Viola,
    Cello,
    Contrabass,
    TremoloStrings,
    PizzicatoStrings,
    OrchestralHarp,
    Timpani,

    // Ensemble
    StringEnsemble1,
    StringEnsemble2,
    SynthStrings1,
    SynthStrings2,
    ChoirAahs,
    VoiceOohs,
    SynthVoice,
    OrchestraHit,

    // Brass
    Trumpet,
    Trombone,
    Tuba,
    MutedTrumpet,
    FrenchHorn,
    BrassSection,
    SynthBrass1,
    SynthBrass2,

    // Reed
    SopranoSax,
    AltoSax,
    TenorSax,
    BaritoneSax,
    Oboe,
    EnglishHorn,
    Bassoon,
    Clarinet,

    // Pipe
    Piccolo,
    Flute,
    Recorder,
    PanFlute,
    BlownBottle,
    Shakuhachi,
    Whistle,
    Ocarina,

    // Synth Lead
    Lead1Square,
    Lead2Sawtooth,
    Lead3Calliope,
    Lead4Chiff,
    Lead5Charang,
    Lead6SpaceVoice,
    Lead7Fifths,
    Lead8BassAndLead,

    // Synth Pad
    Pad1NewAge,
    Pad2Warm,
    Pad3Polysynth,
    Pad4Choir,
    Pad5Bowed,
    Pad6Metallic,
    Pad7Halo,
    Pad8Sweep,

    // Synth Effects
    FX1Rain,
    FX2Soundtrack,
    FX3Crystal,
    FX4Atmosphere,
    FX5Brightness,
    FX6Goblins,
    FX7Echoes,
    FX8SciFi,

    // Ethnic
    Sitar,
    Banjo,
    Shamisen,
    Koto,
    Kalimba,
    BagPipe,
    Fiddle,
    Shanai,

    // Percussive
    TinkleBell,
    Agogo,
    SteelDrums,
    Woodblock,
    TaikoDrum,
    MelodicTom,
    SynthDrum,
    ReverseCymbal,

    // Sound Effects
    GuitarFretNoise,
    BreathNoise,
    Seashore,
    BirdTweet,
    TelephoneRing,
    Helicopter,
    Applause,
    Gunshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Piano = 1,
    ChromaticPrecussion,
    Organ,
    Guitar,
    Bass,
    Strings,
    Ensemble,
    Brass,
    Reed,
    Pipe,
    SynthLead,
    SynthPad,
    SynthEffects,
    Ethnic,
    Percussive,
    SoundEffects,
}

pub static INSTRUMENTS: [Instrument; 128] = {
    use Instrument::*;
    [
        // Piano
        AcousticGrandPiano,
        BrightAcousticPiano,
        ElectricGrandPiano,
        HonkyTonkPiano,
        ElectricPiano1,
        ElectricPiano2,
        Harpsichord,
        Clavinet,
        // Chromatic Precussion
        Celesta,
        Glockenspiel,
        MusicBox,
        Vibraphone,
        Marimba,
        Xylophone,
        TubularBells,
        Dulcimer,
        // Organ
        DrawbarOrgan,
        PercussiveOrgan,
        RockOrgan,
        ChurchOrgan,
        ReedOrgan,
        Accordion,
        Harmonica,
        TangoAccordion,
        // Guitar
        AcousticGuitarNylon,
        AcousticGuitarSteel,
        ElectricGuitarJazz,
        ElectricGuitarClean,
        ElectricGuitarMuted,
        ElectricGuitarOverdriven,
        ElectricGuitarDistortion,
        ElectricGuitarHarmonics,
        // Bass
        AcousticBass,
        ElectricBassFinger,
        ElectricBassPicked,
        FretlessBass,
        SlapBass1,
        SlapBass2,
        SynthBass1,
        SynthBass2,
        // Strings
        Violin,
        Viola,
        Cello,
        Contrabass,
        TremoloStrings,
        PizzicatoStrings,
        OrchestralHarp,
        Timpani,
        // Ensemble
        StringEnsemble1,
        StringEnsemble2,
        SynthStrings1,
        SynthStrings2,
        ChoirAahs,
        VoiceOohs,
        SynthVoice,
        OrchestraHit,
        // Brass
        Trumpet,
        Trombone,
        Tuba,
        MutedTrumpet,
        FrenchHorn,
        BrassSection,
        SynthBrass1,
        SynthBrass2,
        // Reed
        SopranoSax,
        AltoSax,
        TenorSax,
        BaritoneSax,
        Oboe,
        EnglishHorn,
        Bassoon,
        Clarinet,
        // Pipe
        Piccolo,
        Flute,
        Recorder,
        PanFlute,
        BlownBottle,
        Shakuhachi,
        Whistle,
        Ocarina,
        // Synth Lead
        Lead1Square,
        Lead2Sawtooth,
        Lead3Calliope,
        Lead4Chiff,
        Lead5Charang,
        Lead6SpaceVoice,
        Lead7Fifths,
        Lead8BassAndLead,
        // Synth Pad
        Pad1NewAge,
        Pad2Warm,
        Pad3Polysynth,
        Pad4Choir,
        Pad5Bowed,
        Pad6Metallic,
        Pad7Halo,
        Pad8Sweep,
        // Synth Effects
        FX1Rain,
        FX2Soundtrack,
        FX3Crystal,
        FX4Atmosphere,
        FX5Brightness,
        FX6Goblins,
        FX7Echoes,
        FX8SciFi,
        // Ethnic
        Sitar,
        Banjo,
        Shamisen,
        Koto,
        Kalimba,
        BagPipe,
        Fiddle,
        Shanai,
        // Percussive
        TinkleBell,
        Agogo,
        SteelDrums,
        Woodblock,
        TaikoDrum,
        MelodicTom,
        SynthDrum,
        ReverseCymbal,
        // Sound Effects
        GuitarFretNoise,
        BreathNoise,
        Seashore,
        BirdTweet,
        TelephoneRing,
        Helicopter,
        Applause,
        Gunshot,
    ]
};

pub static CATEGORIES: [Category; 16] = {
    use Category::*;
    [
        Piano,
        ChromaticPrecussion,
        Organ,
        Guitar,
        Bass,
        Strings,
        Ensemble,
        Brass,
        Reed,
        Pipe,
        SynthLead,
        SynthPad,
        SynthEffects,
        Ethnic,
        Percussive,
        SoundEffects,
    ]
};

impl Instrument {
    pub fn category(self) -> Category {
        let index = self as usize;
        assert!(0 < index && index <= 128);
        CATEGORIES[(index - 1) / 8]
    }
}

impl std::fmt::Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.name(), f)
    }
}

static mut INST_NAME: [&str; 128] = [""; 128];

static INST_NAME_INIT: Once = Once::new();

impl Instrument {
    pub fn name(self) -> &'static str {
        INST_NAME_INIT.call_once(|| {
            let src = include_str!("inst_name.in");
            for (i, line) in src.lines().enumerate() {
                assert!(i < 128);
                let (_, inst) = line.split_once(' ').unwrap();
                unsafe {
                    INST_NAME[i] = inst;
                }
            }
        });
        let index: usize = self as usize;
        assert!(0 < index && index <= 128);
        unsafe { INST_NAME[index - 1] }
    }
}

static mut INST_NAME_JA: [&str; 128] = [""; 128];

static INST_NAME_JA_INIT: Once = Once::new();

impl Instrument {
    pub fn name_ja(self) -> &'static str {
        INST_NAME_JA_INIT.call_once(|| {
            let src = include_str!("inst_name_ja.in");
            for (i, line) in src.lines().enumerate() {
                assert!(i < 128);
                let (_, inst) = line.split_once('.').unwrap();
                unsafe {
                    INST_NAME_JA[i] = inst;
                }
            }
        });
        let index: usize = self as usize;
        assert!(0 < index && index <= 128);
        unsafe { INST_NAME_JA[index - 1] }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.name(), f)
    }
}

static mut CATEGORY_NAME: [&str; 16] = [""; 16];

static CATEGORY_NAME_INIT: Once = Once::new();

impl Category {
    pub fn name(self) -> &'static str {
        CATEGORY_NAME_INIT.call_once(|| {
            let src = include_str!("inst_category_name.in");
            for (i, name) in src.lines().enumerate() {
                assert!(i < 16);
                unsafe {
                    CATEGORY_NAME[i] = name;
                }
            }
        });
        let index: usize = self as usize;
        assert!(0 < index && index <= 16);
        unsafe { CATEGORY_NAME[index - 1] }
    }
}

static mut CATEGORY_NAME_JA: [&str; 16] = [""; 16];

static CATEGORY_NAME_JA_INIT: Once = Once::new();

impl Category {
    pub fn name_ja(self) -> &'static str {
        CATEGORY_NAME_JA_INIT.call_once(|| {
            let src = include_str!("inst_category_name_ja.in");
            for (i, name) in src.lines().enumerate() {
                assert!(i < 16);
                unsafe {
                    CATEGORY_NAME_JA[i] = name;
                }
            }
        });
        let index: usize = self as usize;
        assert!(0 < index && index <= 16);
        unsafe { CATEGORY_NAME_JA[index - 1] }
    }
}

impl Category {
    pub fn instruments(self) -> &'static [Instrument] {
        let index = self as usize;
        assert!(0 < index && index <= 16);
        let start = (index - 1) * 8;
        let end = start + 8;
        &INSTRUMENTS[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Instrument::AcousticGrandPiano as u32, 1);
        assert_eq!(Instrument::OrchestraHit as u32, 56);
        assert_eq!(Instrument::TinkleBell as u32, 113);
        assert_eq!(Instrument::Gunshot as u32, 128);

        assert_eq!(
            Instrument::AcousticGrandPiano.name(),
            "Acoustic Grand Piano"
        );
        assert_eq!(Instrument::OrchestraHit.name(), "Orchestra Hit");
        assert_eq!(Instrument::TinkleBell.name(), "Tinkle Bell");
        assert_eq!(Instrument::Gunshot.name(), "Gunshot");

        assert_eq!(
            Instrument::AcousticGrandPiano.name_ja(),
            "アコースティック・グランド・ピアノ"
        );
        assert_eq!(Instrument::OrchestraHit.name_ja(), "オーケストラ・ヒット");
        assert_eq!(Instrument::TinkleBell.name_ja(), "ティンカ・ベル");
        assert_eq!(Instrument::Gunshot.name_ja(), "ガン・ショット");

        assert_eq!(Instrument::AcousticGrandPiano.category(), Category::Piano);
        assert_eq!(Instrument::OrchestraHit.category(), Category::Ensemble);
        assert_eq!(Instrument::TinkleBell.category(), Category::Percussive);
        assert_eq!(Instrument::Gunshot.category(), Category::SoundEffects);

        assert_eq!(Category::Piano.name(), "Piano");
        assert_eq!(Category::SoundEffects.name(), "Sound Effects");

        assert_eq!(Category::Piano.name_ja(), "ピアノ");
        assert_eq!(Category::SoundEffects.name_ja(), "サウンド・エフェクト");

        assert_eq!(Category::Piano.instruments(), {
            use Instrument::*;
            [
                AcousticGrandPiano,
                BrightAcousticPiano,
                ElectricGrandPiano,
                HonkyTonkPiano,
                ElectricPiano1,
                ElectricPiano2,
                Harpsichord,
                Clavinet,
            ]
        });
        assert_eq!(Category::SoundEffects.instruments(), {
            use Instrument::*;
            [
                GuitarFretNoise,
                BreathNoise,
                Seashore,
                BirdTweet,
                TelephoneRing,
                Helicopter,
                Applause,
                Gunshot,
            ]
        });
    }
}
