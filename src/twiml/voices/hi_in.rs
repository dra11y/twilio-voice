#![allow(non_upper_case_globals)]

use crate::{
    PriceType,
    twiml::{
        Gender, VoiceGender, VoicePrice,
        voices::{GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
    },
};

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Kajal-Neural")]
            #[strum(to_string = "Polly.Kajal-Neural")]
            KajalNeural,
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
                Self::HiIn(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
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
                    Voice::Female(_) => Gender::Female,
                }
            }
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Neural2-B")]
            #[strum(to_string = "Google.hi-IN-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.hi-IN-Neural2-C")]
            #[strum(to_string = "Google.hi-IN-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.hi-IN-Wavenet-B")]
            #[strum(to_string = "Google.hi-IN-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.hi-IN-Wavenet-C")]
            #[strum(to_string = "Google.hi-IN-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.hi-IN-Wavenet-F")]
            #[strum(to_string = "Google.hi-IN-Wavenet-F")]
            WavenetF,
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
                Self::HiIn(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Neural2-A")]
            #[strum(to_string = "Google.hi-IN-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.hi-IN-Neural2-D")]
            #[strum(to_string = "Google.hi-IN-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.hi-IN-Wavenet-A")]
            #[strum(to_string = "Google.hi-IN-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.hi-IN-Wavenet-D")]
            #[strum(to_string = "Google.hi-IN-Wavenet-D")]
            WavenetD,
            #[serde(rename = "Google.hi-IN-Wavenet-E")]
            #[strum(to_string = "Google.hi-IN-Wavenet-E")]
            WavenetE,
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
                Self::HiIn(super::super::Voice::Neural(super::Voice::Google(
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
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
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

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Standard-B")]
            #[strum(to_string = "Google.hi-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.hi-IN-Standard-C")]
            #[strum(to_string = "Google.hi-IN-Standard-C")]
            StandardC,
            #[serde(rename = "Google.hi-IN-Standard-F")]
            #[strum(to_string = "Google.hi-IN-Standard-F")]
            StandardF,
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
                Self::HiIn(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Standard-A")]
            #[strum(to_string = "Google.hi-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.hi-IN-Standard-D")]
            #[strum(to_string = "Google.hi-IN-Standard-D")]
            StandardD,
            #[serde(rename = "Google.hi-IN-Standard-E")]
            #[strum(to_string = "Google.hi-IN-Standard-E")]
            StandardE,
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
                Self::HiIn(super::super::Voice::Standard(super::Voice::Google(
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
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
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

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Aditi")]
            #[strum(to_string = "Polly.Aditi")]
            Aditi,
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
                Self::HiIn(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
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

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Charon")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Fenrir")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Orus")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Puck")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Puck")]
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
                Self::HiIn(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Aoede")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Kore")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Leda")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Zephyr")]
            #[strum(to_string = "Google.hi-IN-Chirp3-HD-Zephyr")]
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
                Self::HiIn(super::super::Voice::Generative(super::Voice::Google(
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
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
                Voice::Google(voice) => voice.gender(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
    Generative(generative::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<PriceType> {
        match self {
            Voice::Neural(_) => crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE),
            Voice::Standard(_) => crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE),
            Voice::Generative(_) => crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE),
        }
    }
}

impl VoiceGender for Voice {
    fn gender(&self) -> Gender {
        match self {
            Voice::Neural(voice) => voice.gender(),
            Voice::Standard(voice) => voice.gender(),
            Voice::Generative(voice) => voice.gender(),
        }
    }
}
