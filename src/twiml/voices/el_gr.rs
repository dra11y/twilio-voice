#![allow(non_upper_case_globals)]

use crate::{
    PriceType,
    twiml::{
        Gender, VoiceGender, VoicePrice,
        voices::{NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
    },
};

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.el-GR-Standard-B")]
            #[strum(to_string = "Google.el-GR-Standard-B")]
            StandardB,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::ElGr(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Female(_) => Gender::Female,
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<PriceType> {
            crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
        }
    }

    impl VoiceGender for Voice {
        fn gender(&self) -> Gender {
            match self {
                Voice::Google(voice) => voice.gender(),
            }
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
            #[serde(rename = "Google.el-GR-Wavenet-B")]
            #[strum(to_string = "Google.el-GR-Wavenet-B")]
            WavenetB,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Female {
            fn gender(&self) -> Gender {
                Gender::Female
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::ElGr(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Female(_) => Gender::Female,
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<PriceType> {
            crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
        }
    }

    impl VoiceGender for Voice {
        fn gender(&self) -> Gender {
            match self {
                Voice::Google(voice) => voice.gender(),
            }
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
    fn price(&self) -> Option<PriceType> {
        match self {
            Voice::Standard(_) => crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE),
            Voice::Neural(_) => crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE),
        }
    }
}

impl VoiceGender for Voice {
    fn gender(&self) -> Gender {
        match self {
            Voice::Standard(voice) => voice.gender(),
            Voice::Neural(voice) => voice.gender(),
        }
    }
}
