#![allow(non_upper_case_globals)]

use crate::twiml::{
    VoicePrice,
    voices::{NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
};

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fi-FI-Standard-B")]
            StandardB,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::FiFi(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
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

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fi-FI-Wavenet-B")]
            WavenetB,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::FiFi(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Suvi-Neural")]
            SuviNeural,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::FiFi(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
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
        Polly(polly::Voice),
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
    Standard(standard::Voice),
    Neural(neural::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<f32> {
        match self {
            Voice::Standard(_) => Some(STANDARD_VOICE_PRICE),
            Voice::Neural(_) => Some(NEURAL_VOICE_PRICE),
        }
    }
}

pub mod female {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardB: Female = Female::StandardB;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const WavenetB: Female = Female::WavenetB;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const SuviNeural: Female = Female::SuviNeural;
        }
    }
}
