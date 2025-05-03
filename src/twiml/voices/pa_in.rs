#![allow(non_upper_case_globals)]

use crate::{NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE, VoicePrice};

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

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
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

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
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

        impl VoicePrice for Voice {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> f32 {
            NEURAL_VOICE_PRICE
        }
    }
}

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.pa-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.pa-IN-Standard-C")]
            StandardC,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::PaIn(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.pa-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.pa-IN-Standard-D")]
            StandardD,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::PaIn(super::super::Voice::Standard(super::Voice::Google(
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

        impl VoicePrice for Voice {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> f32 {
            STANDARD_VOICE_PRICE
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> f32 {
        match self {
            Voice::Neural(_) => NEURAL_VOICE_PRICE,
            Voice::Standard(_) => STANDARD_VOICE_PRICE,
        }
    }
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
