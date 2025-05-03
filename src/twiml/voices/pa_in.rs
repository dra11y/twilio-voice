#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.pa-IN-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.pa-IN-Wavenet-C")]
            WavenetC,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PaIn(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pa-IN-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.pa-IN-Wavenet-D")]
            WavenetD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PaIn(super::super::Voice::Neural(super::Voice::Google(
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

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pa-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.pa-IN-Standard-D")]
            StandardD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PaIn(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.pa-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.pa-IN-Standard-C")]
            StandardC,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PaIn(super::super::Voice::Standard(super::Voice::Google(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
}

pub mod female {
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetC: Female = Female::WavenetC;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardC: Female = Female::StandardC;
        }
    }
}

pub mod male {
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetD: Male = Male::WavenetD;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardD: Male = Male::StandardD;
        }
    }
}
