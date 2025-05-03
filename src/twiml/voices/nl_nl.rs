#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.nl-NL-Standard-F")]
            StandardF,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::NlNl(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.nl-NL-Standard-G")]
            StandardG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::NlNl(super::super::Voice::Standard(super::Voice::Google(
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
            #[serde(rename = "Polly.Ruben")]
            Ruben,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::NlNl(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Lotte")]
            Lotte,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::NlNl(super::super::Voice::Standard(super::Voice::Polly(
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

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Laura-Neural")]
            LauraNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::NlNl(super::super::Voice::Neural(super::Voice::Polly(
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
            #[serde(rename = "Google.nl-NL-Wavenet-F")]
            WavenetF,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::NlNl(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.nl-NL-Wavenet-G")]
            WavenetG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::NlNl(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::NlNl(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.nl-NL-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::NlNl(super::super::Voice::Generative(super::Voice::Google(
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
    Standard(standard::Voice),
    Neural(neural::Voice),
    Generative(generative::Voice),
}

pub mod female {
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Lotte: Female = Female::Lotte;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardF: Female = Female::StandardF;
        }
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const LauraNeural: Female = Female::LauraNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetF: Female = Female::WavenetF;
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
            pub const StandardG: Male = Male::StandardG;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Ruben: Male = Male::Ruben;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetG: Male = Male::WavenetG;
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
