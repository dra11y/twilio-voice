#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.da-DK-Standard-F")]
            StandardF,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DaDk(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.da-DK-Standard-G")]
            StandardG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DaDk(super::super::Voice::Standard(super::Voice::Google(
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
            #[serde(rename = "Polly.Naja")]
            Naja,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DaDk(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Mads")]
            Mads,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DaDk(super::super::Voice::Standard(super::Voice::Polly(
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

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Sofie-Neural")]
            SofieNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DaDk(super::super::Voice::Neural(super::Voice::Polly(
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
            #[serde(rename = "Google.da-DK-Wavenet-F")]
            WavenetF,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::DaDk(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.da-DK-Wavenet-G")]
            WavenetG,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::DaDk(super::super::Voice::Neural(super::Voice::Google(
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
    Standard(standard::Voice),
    Neural(neural::Voice),
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardF: Female = Female::StandardF;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Naja: Female = Female::Naja;
        }
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const SofieNeural: Female = Female::SofieNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetF: Female = Female::WavenetF;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Mads: Male = Male::Mads;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardG: Male = Male::StandardG;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetG: Male = Male::WavenetG;
        }
    }
}
