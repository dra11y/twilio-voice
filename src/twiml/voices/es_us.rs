#![allow(non_upper_case_globals)]

use crate::twiml::{
    VoicePrice,
    voices::{GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
};

use serde::{Deserialize, Serialize};

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.es-US-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.es-US-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.es-US-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.es-US-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EsUs(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.es-US-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.es-US-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.es-US-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.es-US-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EsUs(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Lupe-Generative")]
            LupeGenerative,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EsUs(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Pedro-Generative")]
            PedroGenerative,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EsUs(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }
    }

    #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<f32> {
            Some(GENERATIVE_VOICE_PRICE)
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
            #[serde(rename = "Google.es-US-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.es-US-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.es-US-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.es-US-Wavenet-C")]
            WavenetC,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EsUs(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.es-US-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.es-US-Wavenet-A")]
            WavenetA,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EsUs(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Lupe-Neural")]
            LupeNeural,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EsUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Pedro-Neural")]
            PedroNeural,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EsUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }
    }

    #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<f32> {
            Some(NEURAL_VOICE_PRICE)
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
            #[serde(rename = "Google.es-US-Standard-B")]
            StandardB,
            #[serde(rename = "Google.es-US-Standard-C")]
            StandardC,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EsUs(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.es-US-Standard-A")]
            StandardA,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EsUs(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Miguel")]
            Miguel,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EsUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Lupe")]
            Lupe,
            #[serde(rename = "Polly.Penelope")]
            Penelope,
            #[serde(rename = "Polly.Penélope")]
            Penélope,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EsUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }
    }

    #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<f32> {
            Some(STANDARD_VOICE_PRICE)
        }
    }
}

#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Neural(neural::Voice),
    Standard(standard::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<f32> {
        match self {
            Voice::Generative(_) => Some(GENERATIVE_VOICE_PRICE),
            Voice::Neural(_) => Some(NEURAL_VOICE_PRICE),
            Voice::Standard(_) => Some(STANDARD_VOICE_PRICE),
        }
    }
}

pub mod female {
    pub mod generative {
        pub mod google {
            use super::super::super::generative::google::*;
            pub const Chirp3HdAoede: Female = Female::Chirp3HdAoede;
            pub const Chirp3HdKore: Female = Female::Chirp3HdKore;
            pub const Chirp3HdLeda: Female = Female::Chirp3HdLeda;
            pub const Chirp3HdZephyr: Female = Female::Chirp3HdZephyr;
        }
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const LupeGenerative: Female = Female::LupeGenerative;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2A: Female = Female::Neural2A;
            pub const WavenetA: Female = Female::WavenetA;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const LupeNeural: Female = Female::LupeNeural;
        }
    }
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Lupe: Female = Female::Lupe;
            pub const Penelope: Female = Female::Penelope;
            pub const Penélope: Female = Female::Penélope;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
        }
    }
}

pub mod male {
    pub mod generative {
        pub mod google {
            use super::super::super::generative::google::*;
            pub const Chirp3HdCharon: Male = Male::Chirp3HdCharon;
            pub const Chirp3HdFenrir: Male = Male::Chirp3HdFenrir;
            pub const Chirp3HdOrus: Male = Male::Chirp3HdOrus;
            pub const Chirp3HdPuck: Male = Male::Chirp3HdPuck;
        }
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const PedroGenerative: Male = Male::PedroGenerative;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2B: Male = Male::Neural2B;
            pub const Neural2C: Male = Male::Neural2C;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetC: Male = Male::WavenetC;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const PedroNeural: Male = Male::PedroNeural;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardC: Male = Male::StandardC;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Miguel: Male = Male::Miguel;
        }
    }
}
