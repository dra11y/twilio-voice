#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Cristiano")]
            Cristiano,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtPt(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Ines")]
            Ines,
            #[serde(rename = "Polly.Inês")]
            Inês,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtPt(super::super::Voice::Standard(super::Voice::Polly(
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
            #[serde(rename = "Google.pt-PT-Standard-E")]
            StandardE,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtPt(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pt-PT-Standard-F")]
            StandardF,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtPt(super::super::Voice::Standard(super::Voice::Google(
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

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pt-PT-Wavenet-F")]
            WavenetF,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PtPt(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.pt-PT-Wavenet-E")]
            WavenetE,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtPt(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Polly.Ines-Neural")]
            InesNeural,
            #[serde(rename = "Polly.Inês-Neural")]
            InêsNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PtPt(super::super::Voice::Neural(super::Voice::Polly(
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
    Neural(neural::Voice),
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardE: Female = Female::StandardE;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Ines: Female = Female::Ines;
            pub const Inês: Female = Female::Inês;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetE: Female = Female::WavenetE;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const InesNeural: Female = Female::InesNeural;
            pub const InêsNeural: Female = Female::InêsNeural;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardF: Male = Male::StandardF;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Cristiano: Male = Male::Cristiano;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetF: Male = Male::WavenetF;
        }
    }
}
