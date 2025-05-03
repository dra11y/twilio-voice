#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fil-PH-Standard-A")]
            StandardA,
            #[serde(rename = "Google.fil-PH-Standard-B")]
            StandardB,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FilPh(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fil-PH-Standard-C")]
            StandardC,
            #[serde(rename = "Google.fil-PH-Standard-D")]
            StandardD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FilPh(super::super::Voice::Standard(super::Voice::Google(
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

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fil-PH-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.fil-PH-Wavenet-B")]
            WavenetB,
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::FilPh(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fil-PH-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.fil-PH-Wavenet-D")]
            WavenetD,
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::FilPh(super::super::Voice::Neural(super::Voice::Google(
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
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardB: Female = Female::StandardB;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetB: Female = Female::WavenetB;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardC: Male = Male::StandardC;
            pub const StandardD: Male = Male::StandardD;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetC: Male = Male::WavenetC;
            pub const WavenetD: Male = Male::WavenetD;
        }
    }
}
