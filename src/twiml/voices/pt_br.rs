#![allow(non_upper_case_globals)]

use crate::{GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE, VoicePrice};

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pt-BR-Standard-B")]
            StandardB,
            #[serde(rename = "Google.pt-BR-Standard-E")]
            StandardE,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtBr(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.pt-BR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.pt-BR-Standard-D")]
            StandardD,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtBr(super::super::Voice::Standard(super::Voice::Google(
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
                STANDARD_VOICE_PRICE
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Camila")]
            Camila,
            #[serde(rename = "Polly.Vitoria")]
            Vitoria,
            #[serde(rename = "Polly.Vitória")]
            Vitória,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtBr(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Ricardo")]
            Ricardo,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtBr(super::super::Voice::Standard(super::Voice::Polly(
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
        Google(google::Voice),
        Polly(polly::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> f32 {
            STANDARD_VOICE_PRICE
        }
    }
}

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Camila-Neural")]
            CamilaNeural,
            #[serde(rename = "Polly.Vitoria-Neural")]
            VitoriaNeural,
            #[serde(rename = "Polly.Vitória-Neural")]
            VitóriaNeural,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtBr(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Thiago-Neural")]
            ThiagoNeural,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtBr(super::super::Voice::Neural(super::Voice::Polly(
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

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.pt-BR-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.pt-BR-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.pt-BR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.pt-BR-Wavenet-D")]
            WavenetD,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtBr(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pt-BR-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.pt-BR-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.pt-BR-Wavenet-E")]
            WavenetE,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtBr(super::super::Voice::Neural(super::Voice::Google(
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
        Polly(polly::Voice),
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> f32 {
            NEURAL_VOICE_PRICE
        }
    }
}

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                GENERATIVE_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtBr(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                GENERATIVE_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtBr(super::super::Voice::Generative(super::Voice::Google(
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
        Google(google::Voice),
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
    Standard(standard::Voice),
    Neural(neural::Voice),
    Generative(generative::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> f32 {
        match self {
            Voice::Standard(_) => STANDARD_VOICE_PRICE,
            Voice::Neural(_) => NEURAL_VOICE_PRICE,
            Voice::Generative(_) => GENERATIVE_VOICE_PRICE,
        }
    }
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardC: Female = Female::StandardC;
            pub const StandardD: Female = Female::StandardD;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Camila: Female = Female::Camila;
            pub const Vitoria: Female = Female::Vitoria;
            pub const Vitória: Female = Female::Vitória;
        }
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const CamilaNeural: Female = Female::CamilaNeural;
            pub const VitoriaNeural: Female = Female::VitoriaNeural;
            pub const VitóriaNeural: Female = Female::VitóriaNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2A: Female = Female::Neural2A;
            pub const Neural2C: Female = Female::Neural2C;
            pub const WavenetC: Female = Female::WavenetC;
            pub const WavenetD: Female = Female::WavenetD;
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
    }
}

pub mod male {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardE: Male = Male::StandardE;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Ricardo: Male = Male::Ricardo;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2B: Male = Male::Neural2B;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetE: Male = Male::WavenetE;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const ThiagoNeural: Male = Male::ThiagoNeural;
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
}
