#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fr-FR-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.fr-FR-Wavenet-G")]
            WavenetG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrFr(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fr-FR-Neural2-F")]
            Neural2F,
            #[serde(rename = "Google.fr-FR-Wavenet-F")]
            WavenetF,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrFr(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Polly.Lea-Neural")]
            LeaNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrFr(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Remi-Neural")]
            RemiNeural,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrFr(super::super::Voice::Neural(super::Voice::Polly(
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

pub mod standard {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Mathieu")]
            Mathieu,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrFr(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Celine")]
            Celine,
            #[serde(rename = "Polly.Céline")]
            Céline,
            #[serde(rename = "Polly.Lea")]
            Lea,
            #[serde(rename = "Polly.Léa")]
            Léa,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrFr(super::super::Voice::Standard(super::Voice::Polly(
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
        pub enum Female {
            #[serde(rename = "Google.fr-FR-Standard-F")]
            StandardF,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrFr(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fr-FR-Standard-G")]
            StandardG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrFr(super::super::Voice::Standard(super::Voice::Google(
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
        pub enum Male {
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrFr(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrFr(super::super::Voice::Generative(super::Voice::Google(
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
            #[serde(rename = "Polly.Lea-Generative")]
            LeaGenerative,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FrFr(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Rémi-Generative")]
            RémiGenerative,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FrFr(super::super::Voice::Generative(super::Voice::Polly(
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
            pub const Neural2F: Female = Female::Neural2F;
            pub const WavenetF: Female = Female::WavenetF;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const LeaNeural: Female = Female::LeaNeural;
        }
    }
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Celine: Female = Female::Celine;
            pub const Céline: Female = Female::Céline;
            pub const Lea: Female = Female::Lea;
            pub const Léa: Female = Female::Léa;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardF: Female = Female::StandardF;
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
            pub const LeaGenerative: Female = Female::LeaGenerative;
        }
    }
}

pub mod male {
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const RemiNeural: Male = Male::RemiNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2G: Male = Male::Neural2G;
            pub const WavenetG: Male = Male::WavenetG;
        }
    }
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Mathieu: Male = Male::Mathieu;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardG: Male = Male::StandardG;
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
            pub const RémiGenerative: Male = Male::RémiGenerative;
        }
    }
}
