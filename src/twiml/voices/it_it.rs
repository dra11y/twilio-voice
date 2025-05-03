#![allow(non_upper_case_globals)]

use crate::{GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE, VoicePrice};

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Bianca")]
            Bianca,
            #[serde(rename = "Polly.Carla")]
            Carla,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::ItIt(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Giorgio")]
            Giorgio,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::ItIt(super::super::Voice::Standard(super::Voice::Polly(
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
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.it-IT-Standard-A")]
            StandardA,
            #[serde(rename = "Google.it-IT-Standard-E")]
            StandardE,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::ItIt(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.it-IT-Standard-F")]
            StandardF,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::ItIt(super::super::Voice::Standard(super::Voice::Google(
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
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
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
        fn price(&self) -> f32 {
            STANDARD_VOICE_PRICE
        }
    }
}

pub mod generative {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Bianca-Generative")]
            BiancaGenerative,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                GENERATIVE_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::ItIt(super::super::Voice::Generative(super::Voice::Polly(
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
            fn price(&self) -> f32 {
                GENERATIVE_VOICE_PRICE
            }
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.it-IT-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                GENERATIVE_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::ItIt(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.it-IT-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                GENERATIVE_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::ItIt(super::super::Voice::Generative(super::Voice::Google(
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
            fn price(&self) -> f32 {
                GENERATIVE_VOICE_PRICE
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
        fn price(&self) -> f32 {
            GENERATIVE_VOICE_PRICE
        }
    }
}

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.it-IT-Neural2-F")]
            Neural2F,
            #[serde(rename = "Google.it-IT-Wavenet-F")]
            WavenetF,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::ItIt(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.it-IT-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.it-IT-Wavenet-E")]
            WavenetE,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::ItIt(super::super::Voice::Neural(super::Voice::Google(
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
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Bianca-Neural")]
            BiancaNeural,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::ItIt(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Adriano-Neural")]
            AdrianoNeural,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::ItIt(super::super::Voice::Neural(super::Voice::Polly(
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
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
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
        fn price(&self) -> f32 {
            NEURAL_VOICE_PRICE
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
    fn price(&self) -> f32 {
        match self {
            Voice::Standard(_) => STANDARD_VOICE_PRICE,
            Voice::Generative(_) => GENERATIVE_VOICE_PRICE,
            Voice::Neural(_) => NEURAL_VOICE_PRICE,
        }
    }
}

pub mod female {
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Bianca: Female = Female::Bianca;
            pub const Carla: Female = Female::Carla;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardE: Female = Female::StandardE;
        }
    }
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
            pub const BiancaGenerative: Female = Female::BiancaGenerative;
        }
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const BiancaNeural: Female = Female::BiancaNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetE: Female = Female::WavenetE;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardF: Male = Male::StandardF;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Giorgio: Male = Male::Giorgio;
        }
    }
    pub mod generative {
        pub mod google {
            use super::super::super::generative::google::*;
            pub const Chirp3HdCharon: Male = Male::Chirp3HdCharon;
            pub const Chirp3HdFenrir: Male = Male::Chirp3HdFenrir;
            pub const Chirp3HdOrus: Male = Male::Chirp3HdOrus;
            pub const Chirp3HdPuck: Male = Male::Chirp3HdPuck;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2F: Male = Male::Neural2F;
            pub const WavenetF: Male = Male::WavenetF;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const AdrianoNeural: Male = Male::AdrianoNeural;
        }
    }
}
