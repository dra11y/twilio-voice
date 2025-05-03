#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrCa(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrCa(super::super::Voice::Generative(super::Voice::Google(
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Liam-Neural")]
            LiamNeural,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrCa(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Gabrielle-Neural")]
            GabrielleNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrCa(super::super::Voice::Neural(super::Voice::Polly(
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

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fr-CA-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.fr-CA-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.fr-CA-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.fr-CA-Wavenet-D")]
            WavenetD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrCa(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fr-CA-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.fr-CA-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.fr-CA-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.fr-CA-Wavenet-C")]
            WavenetC,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrCa(super::super::Voice::Neural(super::Voice::Google(
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
        Polly(polly::Voice),
        Google(google::Voice),
    }
}

pub mod standard {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Chantal")]
            Chantal,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrCa(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fr-CA-Standard-A")]
            StandardA,
            #[serde(rename = "Google.fr-CA-Standard-C")]
            StandardC,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrCa(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fr-CA-Standard-B")]
            StandardB,
            #[serde(rename = "Google.fr-CA-Standard-D")]
            StandardD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrCa(super::super::Voice::Standard(super::Voice::Google(
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Neural(neural::Voice),
    Standard(standard::Voice),
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
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2A: Female = Female::Neural2A;
            pub const Neural2C: Female = Female::Neural2C;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetC: Female = Female::WavenetC;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const GabrielleNeural: Female = Female::GabrielleNeural;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardC: Female = Female::StandardC;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Chantal: Female = Female::Chantal;
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
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const LiamNeural: Male = Male::LiamNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2B: Male = Male::Neural2B;
            pub const Neural2D: Male = Male::Neural2D;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetD: Male = Male::WavenetD;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardD: Male = Male::StandardD;
        }
    }
}
