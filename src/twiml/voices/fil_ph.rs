#![allow(non_upper_case_globals)]

use crate::{
    PriceType,
    twiml::{
        Gender, VoiceGender, VoicePrice,
        voices::{NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
    },
};

use serde::{Deserialize, Serialize};

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fil-PH-Wavenet-A")]
            #[strum(to_string = "Google.fil-PH-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.fil-PH-Wavenet-B")]
            #[strum(to_string = "Google.fil-PH-Wavenet-B")]
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
                Self::FilPh(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fil-PH-Wavenet-C")]
            #[strum(to_string = "Google.fil-PH-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.fil-PH-Wavenet-D")]
            #[strum(to_string = "Google.fil-PH-Wavenet-D")]
            WavenetD,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
            }
        }

        impl From<Male> for crate::twiml::Voice {
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

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Female(_) => Gender::Female,
                    Voice::Male(_) => Gender::Male,
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

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.fil-PH-Standard-C")]
            #[strum(to_string = "Google.fil-PH-Standard-C")]
            StandardC,
            #[serde(rename = "Google.fil-PH-Standard-D")]
            #[strum(to_string = "Google.fil-PH-Standard-D")]
            StandardD,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Male {
            fn gender(&self) -> Gender {
                Gender::Male
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
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.fil-PH-Standard-A")]
            #[strum(to_string = "Google.fil-PH-Standard-A")]
            StandardA,
            #[serde(rename = "Google.fil-PH-Standard-B")]
            #[strum(to_string = "Google.fil-PH-Standard-B")]
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
                Self::FilPh(super::super::Voice::Standard(super::Voice::Google(
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

        impl VoicePrice for Voice {
            fn price(&self) -> Option<PriceType> {
                crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE)
            }
        }

        impl VoiceGender for Voice {
            fn gender(&self) -> Gender {
                match self {
                    Voice::Male(_) => Gender::Male,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<PriceType> {
        match self {
            Voice::Neural(_) => crate::price_type_from_f64_ok(NEURAL_VOICE_PRICE),
            Voice::Standard(_) => crate::price_type_from_f64_ok(STANDARD_VOICE_PRICE),
        }
    }
}

impl VoiceGender for Voice {
    fn gender(&self) -> Gender {
        match self {
            Voice::Neural(voice) => voice.gender(),
            Voice::Standard(voice) => voice.gender(),
        }
    }
}
