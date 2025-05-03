#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

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
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}

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
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Ricardo")]
            Ricardo,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtBr(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

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

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtBr(super::super::Voice::Standard(super::Voice::Polly(
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
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}

pub mod neural {
    use super::*;

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
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Thiago-Neural")]
            ThiagoNeural,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtBr(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

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

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtBr(super::super::Voice::Neural(super::Voice::Polly(
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
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Standard(standard::Voice),
    Neural(neural::Voice),
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
    }
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
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2A: Female = Female::Neural2A;
            pub const Neural2C: Female = Female::Neural2C;
            pub const WavenetC: Female = Female::WavenetC;
            pub const WavenetD: Female = Female::WavenetD;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const CamilaNeural: Female = Female::CamilaNeural;
            pub const VitoriaNeural: Female = Female::VitoriaNeural;
            pub const VitóriaNeural: Female = Female::VitóriaNeural;
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
    }
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
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const ThiagoNeural: Male = Male::ThiagoNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2B: Male = Male::Neural2B;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetE: Male = Male::WavenetE;
        }
    }
}
