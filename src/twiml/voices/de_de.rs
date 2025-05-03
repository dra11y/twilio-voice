#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.de-DE-Standard-H")]
            StandardH,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DeDe(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.de-DE-Standard-G")]
            StandardG,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DeDe(super::super::Voice::Standard(super::Voice::Google(
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
            #[serde(rename = "Polly.Marlene")]
            Marlene,
            #[serde(rename = "Polly.Vicki")]
            Vicki,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DeDe(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Hans")]
            Hans,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DeDe(super::super::Voice::Standard(super::Voice::Polly(
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
        Polly(polly::Voice),
    }
}

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.de-DE-Neural2-H")]
            Neural2H,
            #[serde(rename = "Google.de-DE-Wavenet-H")]
            WavenetH,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DeDe(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.de-DE-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.de-DE-Wavenet-G")]
            WavenetG,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DeDe(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Polly.Daniel-Neural")]
            DanielNeural,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DeDe(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Vicki-Neural")]
            VickiNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DeDe(super::super::Voice::Neural(super::Voice::Polly(
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
            #[serde(rename = "Polly.Vicki-Generative")]
            VickiGenerative,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DeDe(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Daniel-Generative")]
            DanielGenerative,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DeDe(super::super::Voice::Generative(super::Voice::Polly(
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

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.de-DE-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.de-DE-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.de-DE-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.de-DE-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DeDe(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.de-DE-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.de-DE-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.de-DE-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.de-DE-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DeDe(super::super::Voice::Generative(super::Voice::Google(
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
    Standard(standard::Voice),
    Neural(neural::Voice),
    Generative(generative::Voice),
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardG: Female = Female::StandardG;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Marlene: Female = Female::Marlene;
            pub const Vicki: Female = Female::Vicki;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2G: Female = Female::Neural2G;
            pub const WavenetG: Female = Female::WavenetG;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const VickiNeural: Female = Female::VickiNeural;
        }
    }
    pub mod generative {
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const VickiGenerative: Female = Female::VickiGenerative;
        }
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
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Hans: Male = Male::Hans;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardH: Male = Male::StandardH;
        }
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const DanielNeural: Male = Male::DanielNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2H: Male = Male::Neural2H;
            pub const WavenetH: Male = Male::WavenetH;
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
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const DanielGenerative: Male = Male::DanielGenerative;
        }
    }
}
