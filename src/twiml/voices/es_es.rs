#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.es-ES-Neural2-H")]
            Neural2H,
            #[serde(rename = "Google.es-ES-Wavenet-F")]
            WavenetF,
            #[serde(rename = "Google.es-ES-Wavenet-H")]
            WavenetH,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EsEs(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.es-ES-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.es-ES-Wavenet-E")]
            WavenetE,
            #[serde(rename = "Google.es-ES-Wavenet-G")]
            WavenetG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EsEs(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Polly.Sergio-Neural")]
            SergioNeural,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EsEs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Lucia-Neural")]
            LuciaNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EsEs(super::super::Voice::Neural(super::Voice::Polly(
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

pub mod standard {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Conchita")]
            Conchita,
            #[serde(rename = "Polly.Lucia")]
            Lucia,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EsEs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Enrique")]
            Enrique,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EsEs(super::super::Voice::Standard(super::Voice::Polly(
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
            #[serde(rename = "Google.es-ES-Standard-E")]
            StandardE,
            #[serde(rename = "Google.es-ES-Standard-G")]
            StandardG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EsEs(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.es-ES-Standard-A")]
            StandardA,
            #[serde(rename = "Google.es-ES-Standard-F")]
            StandardF,
            #[serde(rename = "Google.es-ES-Standard-H")]
            StandardH,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EsEs(super::super::Voice::Standard(super::Voice::Google(
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
        pub enum Male {
            #[serde(rename = "Google.es-ES-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EsEs(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.es-ES-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EsEs(super::super::Voice::Generative(super::Voice::Google(
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
            #[serde(rename = "Polly.Sergio-Generative")]
            SergioGenerative,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::EsEs(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Lucia-Generative")]
            LuciaGenerative,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EsEs(super::super::Voice::Generative(super::Voice::Polly(
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
    Standard(standard::Voice),
    Generative(generative::Voice),
}

pub mod female {
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const LuciaNeural: Female = Female::LuciaNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2H: Female = Female::Neural2H;
            pub const WavenetF: Female = Female::WavenetF;
            pub const WavenetH: Female = Female::WavenetH;
        }
    }
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Conchita: Female = Female::Conchita;
            pub const Lucia: Female = Female::Lucia;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardF: Female = Female::StandardF;
            pub const StandardH: Female = Female::StandardH;
        }
    }
    pub mod generative {
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const LuciaGenerative: Female = Female::LuciaGenerative;
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
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2G: Male = Male::Neural2G;
            pub const WavenetE: Male = Male::WavenetE;
            pub const WavenetG: Male = Male::WavenetG;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const SergioNeural: Male = Male::SergioNeural;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardE: Male = Male::StandardE;
            pub const StandardG: Male = Male::StandardG;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Enrique: Male = Male::Enrique;
        }
    }
    pub mod generative {
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const SergioGenerative: Male = Male::SergioGenerative;
        }
        pub mod google {
            use super::super::super::generative::google::*;
            pub const Chirp3HdCharon: Male = Male::Chirp3HdCharon;
            pub const Chirp3HdFenrir: Male = Male::Chirp3HdFenrir;
            pub const Chirp3HdOrus: Male = Male::Chirp3HdOrus;
            pub const Chirp3HdPuck: Male = Male::Chirp3HdPuck;
        }
    }
}
