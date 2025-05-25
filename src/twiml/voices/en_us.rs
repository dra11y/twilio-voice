#![allow(non_upper_case_globals)]

use crate::{
    PriceType,
    twiml::{
        Gender, VoiceGender, VoicePrice,
        voices::{GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
    },
};

use serde::{Deserialize, Serialize};

pub mod generative {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Matthew-Generative")]
            #[strum(to_string = "Polly.Matthew-Generative")]
            MatthewGenerative,
            #[serde(rename = "Polly.Stephen-Generative")]
            #[strum(to_string = "Polly.Stephen-Generative")]
            StephenGenerative,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Danielle-Generative")]
            #[strum(to_string = "Polly.Danielle-Generative")]
            DanielleGenerative,
            #[serde(rename = "Polly.Joanna-Generative")]
            #[strum(to_string = "Polly.Joanna-Generative")]
            JoannaGenerative,
            #[serde(rename = "Polly.Ruth-Generative")]
            #[strum(to_string = "Polly.Ruth-Generative")]
            RuthGenerative,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Male(_) => Gender::Male,
                    Voice::Female(_) => Gender::Female,
                }
            }
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-US-Chirp3-HD-Aoede")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.en-US-Chirp3-HD-Kore")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.en-US-Chirp3-HD-Leda")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.en-US-Chirp3-HD-Zephyr")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Chirp3-HD-Charon")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.en-US-Chirp3-HD-Fenrir")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.en-US-Chirp3-HD-Orus")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.en-US-Chirp3-HD-Puck")]
            #[strum(to_string = "Google.en-US-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Female(_) => Gender::Female,
                    Voice::Male(_) => Gender::Male,
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<PriceType> {
            crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE)
        }
    }

    impl VoiceGender for Voice {
        fn gender(&self) -> Gender {
            match self {
                Voice::Polly(voice) => voice.gender(),
                Voice::Google(voice) => voice.gender(),
            }
        }
    }
}

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-US-Standard-C")]
            #[strum(to_string = "Google.en-US-Standard-C")]
            StandardC,
            #[serde(rename = "Google.en-US-Standard-E")]
            #[strum(to_string = "Google.en-US-Standard-E")]
            StandardE,
            #[serde(rename = "Google.en-US-Standard-F")]
            #[strum(to_string = "Google.en-US-Standard-F")]
            StandardF,
            #[serde(rename = "Google.en-US-Standard-G")]
            #[strum(to_string = "Google.en-US-Standard-G")]
            StandardG,
            #[serde(rename = "Google.en-US-Standard-H")]
            #[strum(to_string = "Google.en-US-Standard-H")]
            StandardH,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Standard-A")]
            #[strum(to_string = "Google.en-US-Standard-A")]
            StandardA,
            #[serde(rename = "Google.en-US-Standard-B")]
            #[strum(to_string = "Google.en-US-Standard-B")]
            StandardB,
            #[serde(rename = "Google.en-US-Standard-D")]
            #[strum(to_string = "Google.en-US-Standard-D")]
            StandardD,
            #[serde(rename = "Google.en-US-Standard-I")]
            #[strum(to_string = "Google.en-US-Standard-I")]
            StandardI,
            #[serde(rename = "Google.en-US-Standard-J")]
            #[strum(to_string = "Google.en-US-Standard-J")]
            StandardJ,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Female(_) => Gender::Female,
                    Voice::Male(_) => Gender::Male,
                }
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Joey")]
            #[strum(to_string = "Polly.Joey")]
            Joey,
            #[serde(rename = "Polly.Justin")]
            #[strum(to_string = "Polly.Justin")]
            Justin,
            #[serde(rename = "Polly.Matthew")]
            #[strum(to_string = "Polly.Matthew")]
            Matthew,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum MaleChild {
            #[serde(rename = "Polly.Kevin")]
            #[strum(to_string = "Polly.Kevin")]
            Kevin,
        }

        impl VoicePrice for MaleChild {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for MaleChild {
            fn gender(&self) -> Gender {
                Gender::MaleChild
            }
        }

        impl From<MaleChild> for crate::twiml::Voice {
            fn from(value: MaleChild) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::MaleChild(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Ivy")]
            #[strum(to_string = "Polly.Ivy")]
            Ivy,
            #[serde(rename = "Polly.Joanna")]
            #[strum(to_string = "Polly.Joanna")]
            Joanna,
            #[serde(rename = "Polly.Kendra")]
            #[strum(to_string = "Polly.Kendra")]
            Kendra,
            #[serde(rename = "Polly.Kimberly")]
            #[strum(to_string = "Polly.Kimberly")]
            Kimberly,
            #[serde(rename = "Polly.Salli")]
            #[strum(to_string = "Polly.Salli")]
            Salli,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            MaleChild(MaleChild),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Male(_) => Gender::Male,
                    Voice::MaleChild(_) => Gender::MaleChild,
                    Voice::Female(_) => Gender::Female,
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<PriceType> {
            crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
        }
    }

    impl VoiceGender for Voice {
        fn gender(&self) -> Gender {
            match self {
                Voice::Google(voice) => voice.gender(),
                Voice::Polly(voice) => voice.gender(),
            }
        }
    }
}

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum MaleChild {
            #[serde(rename = "Polly.Justin-Neural")]
            #[strum(to_string = "Polly.Justin-Neural")]
            JustinNeural,
            #[serde(rename = "Polly.Kevin-Neural")]
            #[strum(to_string = "Polly.Kevin-Neural")]
            KevinNeural,
        }

        impl VoicePrice for MaleChild {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for MaleChild {
            fn gender(&self) -> Gender {
                Gender::MaleChild
            }
        }

        impl From<MaleChild> for crate::twiml::Voice {
            fn from(value: MaleChild) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::MaleChild(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum FemaleChild {
            #[serde(rename = "Polly.Ivy-Neural")]
            #[strum(to_string = "Polly.Ivy-Neural")]
            IvyNeural,
        }

        impl VoicePrice for FemaleChild {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for FemaleChild {
            fn gender(&self) -> Gender {
                Gender::FemaleChild
            }
        }

        impl From<FemaleChild> for crate::twiml::Voice {
            fn from(value: FemaleChild) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::FemaleChild(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Gregory-Neural")]
            #[strum(to_string = "Polly.Gregory-Neural")]
            GregoryNeural,
            #[serde(rename = "Polly.Joey-Neural")]
            #[strum(to_string = "Polly.Joey-Neural")]
            JoeyNeural,
            #[serde(rename = "Polly.Matthew-Neural")]
            #[strum(to_string = "Polly.Matthew-Neural")]
            MatthewNeural,
            #[serde(rename = "Polly.Stephen-Neural")]
            #[strum(to_string = "Polly.Stephen-Neural")]
            StephenNeural,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Danielle-Neural")]
            #[strum(to_string = "Polly.Danielle-Neural")]
            DanielleNeural,
            #[serde(rename = "Polly.Joanna-Neural")]
            #[strum(to_string = "Polly.Joanna-Neural")]
            JoannaNeural,
            #[serde(rename = "Polly.Kendra-Neural")]
            #[strum(to_string = "Polly.Kendra-Neural")]
            KendraNeural,
            #[serde(rename = "Polly.Kimberly-Neural")]
            #[strum(to_string = "Polly.Kimberly-Neural")]
            KimberlyNeural,
            #[serde(rename = "Polly.Ruth-Neural")]
            #[strum(to_string = "Polly.Ruth-Neural")]
            RuthNeural,
            #[serde(rename = "Polly.Salli-Neural")]
            #[strum(to_string = "Polly.Salli-Neural")]
            SalliNeural,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            MaleChild(MaleChild),
            FemaleChild(FemaleChild),
            Male(Male),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::MaleChild(_) => Gender::MaleChild,
                    Voice::FemaleChild(_) => Gender::FemaleChild,
                    Voice::Male(_) => Gender::Male,
                    Voice::Female(_) => Gender::Female,
                }
            }
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-US-Neural2-C")]
            #[strum(to_string = "Google.en-US-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.en-US-Neural2-E")]
            #[strum(to_string = "Google.en-US-Neural2-E")]
            Neural2E,
            #[serde(rename = "Google.en-US-Neural2-F")]
            #[strum(to_string = "Google.en-US-Neural2-F")]
            Neural2F,
            #[serde(rename = "Google.en-US-Neural2-G")]
            #[strum(to_string = "Google.en-US-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.en-US-Neural2-H")]
            #[strum(to_string = "Google.en-US-Neural2-H")]
            Neural2H,
            #[serde(rename = "Google.en-US-Wavenet-C")]
            #[strum(to_string = "Google.en-US-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.en-US-Wavenet-E")]
            #[strum(to_string = "Google.en-US-Wavenet-E")]
            WavenetE,
            #[serde(rename = "Google.en-US-Wavenet-F")]
            #[strum(to_string = "Google.en-US-Wavenet-F")]
            WavenetF,
            #[serde(rename = "Google.en-US-Wavenet-G")]
            #[strum(to_string = "Google.en-US-Wavenet-G")]
            WavenetG,
            #[serde(rename = "Google.en-US-Wavenet-H")]
            #[strum(to_string = "Google.en-US-Wavenet-H")]
            WavenetH,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Neural2-A")]
            #[strum(to_string = "Google.en-US-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.en-US-Neural2-D")]
            #[strum(to_string = "Google.en-US-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.en-US-Neural2-I")]
            #[strum(to_string = "Google.en-US-Neural2-I")]
            Neural2I,
            #[serde(rename = "Google.en-US-Neural2-J")]
            #[strum(to_string = "Google.en-US-Neural2-J")]
            Neural2J,
            #[serde(rename = "Google.en-US-Wavenet-A")]
            #[strum(to_string = "Google.en-US-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.en-US-Wavenet-B")]
            #[strum(to_string = "Google.en-US-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.en-US-Wavenet-D")]
            #[strum(to_string = "Google.en-US-Wavenet-D")]
            WavenetD,
            #[serde(rename = "Google.en-US-Wavenet-I")]
            #[strum(to_string = "Google.en-US-Wavenet-I")]
            WavenetI,
            #[serde(rename = "Google.en-US-Wavenet-J")]
            #[strum(to_string = "Google.en-US-Wavenet-J")]
            WavenetJ,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Female(_) => Gender::Female,
                    Voice::Male(_) => Gender::Male,
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<PriceType> {
            crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
        }
    }

    impl VoiceGender for Voice {
        fn gender(&self) -> Gender {
            match self {
                Voice::Polly(voice) => voice.gender(),
                Voice::Google(voice) => voice.gender(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Standard(standard::Voice),
    Neural(neural::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<PriceType> {
        match self {
            Voice::Generative(_) => crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE),
            Voice::Standard(_) => crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE),
            Voice::Neural(_) => crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE),
        }
    }
}

impl VoiceGender for Voice {
    fn gender(&self) -> Gender {
        match self {
            Voice::Generative(voice) => voice.gender(),
            Voice::Standard(voice) => voice.gender(),
            Voice::Neural(voice) => voice.gender(),
        }
    }
}
