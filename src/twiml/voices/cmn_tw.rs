#![allow(non_upper_case_globals)]

use crate::{NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE, VoicePrice};

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.cmn-TW-Standard-A")]
            StandardA,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::CmnTw(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.cmn-TW-Standard-B")]
            StandardB,
            #[serde(rename = "Google.cmn-TW-Standard-C")]
            StandardC,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                STANDARD_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::CmnTw(super::super::Voice::Standard(super::Voice::Google(
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

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.cmn-TW-Wavenet-A")]
            WavenetA,
        }

        impl VoicePrice for Female {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Female> for crate::Voice {
            fn from(value: Female) -> Self {
                Self::CmnTw(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.cmn-TW-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.cmn-TW-Wavenet-C")]
            WavenetC,
        }

        impl VoicePrice for Male {
            fn price(&self) -> f32 {
                NEURAL_VOICE_PRICE
            }
        }

        impl From<Male> for crate::Voice {
            fn from(value: Male) -> Self {
                Self::CmnTw(super::super::Voice::Neural(super::Voice::Google(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
    Neural(neural::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> f32 {
        match self {
            Voice::Standard(_) => STANDARD_VOICE_PRICE,
            Voice::Neural(_) => NEURAL_VOICE_PRICE,
        }
    }
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Female = Female::StandardA;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetA: Female = Female::WavenetA;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardC: Male = Male::StandardC;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetC: Male = Male::WavenetC;
        }
    }
}
