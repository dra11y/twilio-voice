#![allow(non_upper_case_globals)]

use crate::{
    PriceType,
    twiml::{
        Gender, VoiceGender, VoicePrice,
        voices::{GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
    },
};

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.ko-KR-Standard-A")]
            #[strum(to_string = "Google.ko-KR-Standard-A")]
            StandardA,
            #[serde(rename = "Google.ko-KR-Standard-B")]
            #[strum(to_string = "Google.ko-KR-Standard-B")]
            StandardB,
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
                Self::KoKr(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Standard-C")]
            #[strum(to_string = "Google.ko-KR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.ko-KR-Standard-D")]
            #[strum(to_string = "Google.ko-KR-Standard-D")]
            StandardD,
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
                Self::KoKr(super::super::Voice::Standard(super::Voice::Google(
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
        pub enum Female {
            #[serde(rename = "Polly.Seoyeon")]
            #[strum(to_string = "Polly.Seoyeon")]
            Seoyeon,
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
                Self::KoKr(super::super::Voice::Standard(super::Voice::Polly(
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
        pub enum Female {
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Aoede")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Kore")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Leda")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Zephyr")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Zephyr")]
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
                Self::KoKr(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Charon")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Fenrir")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Orus")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Puck")]
            #[strum(to_string = "Google.ko-KR-Chirp3-HD-Puck")]
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
                Self::KoKr(super::super::Voice::Generative(super::Voice::Google(
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

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Neural2-C")]
            #[strum(to_string = "Google.ko-KR-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.ko-KR-Wavenet-C")]
            #[strum(to_string = "Google.ko-KR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.ko-KR-Wavenet-D")]
            #[strum(to_string = "Google.ko-KR-Wavenet-D")]
            WavenetD,
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
                Self::KoKr(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.ko-KR-Neural2-A")]
            #[strum(to_string = "Google.ko-KR-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.ko-KR-Neural2-B")]
            #[strum(to_string = "Google.ko-KR-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.ko-KR-Wavenet-A")]
            #[strum(to_string = "Google.ko-KR-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.ko-KR-Wavenet-B")]
            #[strum(to_string = "Google.ko-KR-Wavenet-B")]
            WavenetB,
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
                Self::KoKr(super::super::Voice::Neural(super::Voice::Google(
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

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Seoyeon-Neural")]
            #[strum(to_string = "Polly.Seoyeon-Neural")]
            SeoyeonNeural,
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
                Self::KoKr(super::super::Voice::Neural(super::Voice::Polly(
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<PriceType> {
            crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
    Generative(generative::Voice),
    Neural(neural::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<PriceType> {
        match self {
            Voice::Standard(_) => crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE),
            Voice::Generative(_) => crate::price_type_from_f64_ok(GENERATIVE_VOICE_PRICE),
            Voice::Neural(_) => crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE),
        }
    }
}

impl VoiceGender for Voice {
    fn gender(&self) -> Gender {
        match self {
            Voice::Standard(voice) => voice.gender(),
            Voice::Generative(voice) => voice.gender(),
            Voice::Neural(voice) => voice.gender(),
        }
    }
}
