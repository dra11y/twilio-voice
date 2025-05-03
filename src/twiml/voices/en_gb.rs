#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-GB-Neural2-O")]
            Neural2O,
            #[serde(rename = "Google.en-GB-Wavenet-O")]
            WavenetO,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EnGb(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-GB-Neural2-N")]
            Neural2N,
            #[serde(rename = "Google.en-GB-Wavenet-N")]
            WavenetN,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EnGb(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Polly.Arthur-Neural")]
            ArthurNeural,
            #[serde(rename = "Polly.Brian-Neural")]
            BrianNeural,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EnGb(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Amy-Neural")]
            AmyNeural,
            #[serde(rename = "Polly.Emma-Neural")]
            EmmaNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EnGb(super::super::Voice::Neural(super::Voice::Polly(
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

pub mod generative {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Amy-Generative")]
            AmyGenerative,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EnGb(super::super::Voice::Generative(super::Voice::Polly(
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
            #[serde(rename = "Google.en-GB-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.en-GB-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.en-GB-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.en-GB-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EnGb(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-GB-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.en-GB-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.en-GB-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.en-GB-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EnGb(super::super::Voice::Generative(super::Voice::Google(
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

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-GB-Standard-O")]
            StandardO,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EnGb(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-GB-Standard-N")]
            StandardN,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EnGb(super::super::Voice::Standard(super::Voice::Google(
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
            #[serde(rename = "Polly.Brian")]
            Brian,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EnGb(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Amy")]
            Amy,
            #[serde(rename = "Polly.Emma")]
            Emma,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EnGb(super::super::Voice::Standard(super::Voice::Polly(
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
    Neural(neural::Voice),
    Generative(generative::Voice),
    Standard(standard::Voice),
}

pub mod female {
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const AmyNeural: Female = Female::AmyNeural;
            pub const EmmaNeural: Female = Female::EmmaNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2N: Female = Female::Neural2N;
            pub const WavenetN: Female = Female::WavenetN;
        }
    }
    pub mod generative {
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const AmyGenerative: Female = Female::AmyGenerative;
        }
        pub mod google {
            use super::super::super::generative::google::*;
            pub const Chirp3HdAoede: Female = Female::Chirp3HdAoede;
            pub const Chirp3HdKore: Female = Female::Chirp3HdKore;
            pub const Chirp3HdLeda: Female = Female::Chirp3HdLeda;
            pub const Chirp3HdZephyr: Female = Female::Chirp3HdZephyr;
        }
    }
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Amy: Female = Female::Amy;
            pub const Emma: Female = Female::Emma;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardN: Female = Female::StandardN;
        }
    }
}

pub mod male {
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const ArthurNeural: Male = Male::ArthurNeural;
            pub const BrianNeural: Male = Male::BrianNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2O: Male = Male::Neural2O;
            pub const WavenetO: Male = Male::WavenetO;
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
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardO: Male = Male::StandardO;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Brian: Male = Male::Brian;
        }
    }
}
