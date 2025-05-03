#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Seoyeon-Neural")]
            SeoyeonNeural,
        }

        impl From<Female> for crate::Voice {
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
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.ko-KR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.ko-KR-Wavenet-D")]
            WavenetD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::KoKr(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.ko-KR-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.ko-KR-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.ko-KR-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.ko-KR-Wavenet-B")]
            WavenetB,
        }

        impl From<Female> for crate::Voice {
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
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::KoKr(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
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
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
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
            #[serde(rename = "Polly.Seoyeon")]
            Seoyeon,
        }

        impl From<Female> for crate::Voice {
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
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.ko-KR-Standard-D")]
            StandardD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::KoKr(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.ko-KR-Standard-A")]
            StandardA,
            #[serde(rename = "Google.ko-KR-Standard-B")]
            StandardB,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::KoKr(super::super::Voice::Standard(super::Voice::Google(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Generative(generative::Voice),
    Standard(standard::Voice),
}

pub mod female {
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2A: Female = Female::Neural2A;
            pub const Neural2B: Female = Female::Neural2B;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetB: Female = Female::WavenetB;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const SeoyeonNeural: Female = Female::SeoyeonNeural;
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
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Seoyeon: Female = Female::Seoyeon;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardB: Female = Female::StandardB;
        }
    }
}

pub mod male {
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2C: Male = Male::Neural2C;
            pub const WavenetC: Male = Male::WavenetC;
            pub const WavenetD: Male = Male::WavenetD;
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
            pub const StandardC: Male = Male::StandardC;
            pub const StandardD: Male = Male::StandardD;
        }
    }
}
