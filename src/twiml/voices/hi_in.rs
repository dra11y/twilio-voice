#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.hi-IN-Standard-D")]
            StandardD,
            #[serde(rename = "Google.hi-IN-Standard-E")]
            StandardE,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::HiIn(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.hi-IN-Standard-C")]
            StandardC,
            #[serde(rename = "Google.hi-IN-Standard-F")]
            StandardF,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::HiIn(super::super::Voice::Standard(super::Voice::Google(
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
        pub enum Female {
            #[serde(rename = "Polly.Aditi")]
            Aditi,
        }

        impl From<Female> for crate::Voice {
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

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::HiIn(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
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
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.hi-IN-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.hi-IN-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.hi-IN-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.hi-IN-Wavenet-F")]
            WavenetF,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::HiIn(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.hi-IN-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.hi-IN-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.hi-IN-Wavenet-D")]
            WavenetD,
            #[serde(rename = "Google.hi-IN-Wavenet-E")]
            WavenetE,
        }

        impl From<Female> for crate::Voice {
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
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Kajal-Neural")]
            KajalNeural,
        }

        impl From<Female> for crate::Voice {
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
    Standard(standard::Voice),
    Generative(generative::Voice),
    Neural(neural::Voice),
}

pub mod female {
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Aditi: Female = Female::Aditi;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardD: Female = Female::StandardD;
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
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const KajalNeural: Female = Female::KajalNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2A: Female = Female::Neural2A;
            pub const Neural2D: Female = Female::Neural2D;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetD: Female = Female::WavenetD;
            pub const WavenetE: Female = Female::WavenetE;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardC: Male = Male::StandardC;
            pub const StandardF: Male = Male::StandardF;
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
            pub const Neural2B: Male = Male::Neural2B;
            pub const Neural2C: Male = Male::Neural2C;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetC: Male = Male::WavenetC;
            pub const WavenetF: Male = Male::WavenetF;
        }
    }
}
