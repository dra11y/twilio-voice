#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.is-IS-Standard-B")]
            StandardB,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::IsIs(super::super::Voice::Standard(super::Voice::Google(
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

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Dora")]
            Dora,
            #[serde(rename = "Polly.Dóra")]
            Dóra,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::IsIs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Karl")]
            Karl,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::IsIs(super::super::Voice::Standard(super::Voice::Polly(
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
    Standard(standard::Voice),
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Female = Female::StandardB;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Dora: Female = Female::Dora;
            pub const Dóra: Female = Female::Dóra;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Karl: Male = Male::Karl;
        }
    }
}
