#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Niamh-Neural")]
            NiamhNeural,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::EnIe(super::super::Voice::Neural(super::Voice::Polly(
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
        Polly(polly::Voice),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
}

pub mod female {
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const NiamhNeural: Female = Female::NiamhNeural;
        }
    }
}
