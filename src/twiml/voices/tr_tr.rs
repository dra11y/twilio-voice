#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Burcu-Neural")]
            BurcuNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::TrTr(super::super::Voice::Neural(super::Voice::Polly(
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
            #[serde(rename = "Google.tr-TR-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.tr-TR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.tr-TR-Wavenet-D")]
            WavenetD,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::TrTr(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.tr-TR-Wavenet-E")]
            WavenetE,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::TrTr(super::super::Voice::Neural(super::Voice::Google(
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

pub mod standard {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Filiz")]
            Filiz,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::TrTr(super::super::Voice::Standard(super::Voice::Polly(
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
            #[serde(rename = "Google.tr-TR-Standard-A")]
            StandardA,
            #[serde(rename = "Google.tr-TR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.tr-TR-Standard-D")]
            StandardD,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::TrTr(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Standard-B")]
            StandardB,
            #[serde(rename = "Google.tr-TR-Standard-E")]
            StandardE,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::TrTr(super::super::Voice::Standard(super::Voice::Google(
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

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::TrTr(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::TrTr(super::super::Voice::Generative(super::Voice::Google(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
    Generative(generative::Voice),
}

pub mod female {
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetC: Female = Female::WavenetC;
            pub const WavenetD: Female = Female::WavenetD;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const BurcuNeural: Female = Female::BurcuNeural;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardC: Female = Female::StandardC;
            pub const StandardD: Female = Female::StandardD;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Filiz: Female = Female::Filiz;
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
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetE: Male = Male::WavenetE;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardE: Male = Male::StandardE;
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
