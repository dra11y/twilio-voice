#![allow(non_upper_case_globals)]

use crate::twiml::{
    VoicePrice,
    voices::{NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
};

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fil-PH-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.fil-PH-Wavenet-D")]
            WavenetD,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::FilPh(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fil-PH-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.fil-PH-Wavenet-B")]
            WavenetB,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::FilPh(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }
    }

    #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<f32> {
            Some(NEURAL_VOICE_PRICE)
        }
    }
}

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fil-PH-Standard-A")]
            StandardA,
            #[serde(rename = "Google.fil-PH-Standard-B")]
            StandardB,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::FilPh(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fil-PH-Standard-C")]
            StandardC,
            #[serde(rename = "Google.fil-PH-Standard-D")]
            StandardD,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::FilPh(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }
    }

    #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<f32> {
            Some(STANDARD_VOICE_PRICE)
        }
    }
}

#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<f32> {
        match self {
            Voice::Neural(_) => Some(NEURAL_VOICE_PRICE),
            Voice::Standard(_) => Some(STANDARD_VOICE_PRICE),
        }
    }
}

pub mod female {
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetA: Female = Female::WavenetA;
            pub const WavenetB: Female = Female::WavenetB;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
            pub const StandardB: Female = Female::StandardB;
        }
    }
}

pub mod male {
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetC: Male = Male::WavenetC;
            pub const WavenetD: Male = Male::WavenetD;
        }
    }
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardC: Male = Male::StandardC;
            pub const StandardD: Male = Male::StandardD;
        }
    }
}
