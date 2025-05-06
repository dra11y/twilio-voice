#![allow(non_upper_case_globals)]

use crate::twiml::{
    VoicePrice,
    voices::{GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
};

use serde::{Deserialize, Serialize};

pub mod standard {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Joey")]
            Joey,
            #[serde(rename = "Polly.Justin")]
            Justin,
            #[serde(rename = "Polly.Matthew")]
            Matthew,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum MaleChild {
            #[serde(rename = "Polly.Kevin")]
            Kevin,
        }

        impl VoicePrice for MaleChild {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<MaleChild> for crate::twiml::Voice {
            fn from(value: MaleChild) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::MaleChild(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Ivy")]
            Ivy,
            #[serde(rename = "Polly.Joanna")]
            Joanna,
            #[serde(rename = "Polly.Kendra")]
            Kendra,
            #[serde(rename = "Polly.Kimberly")]
            Kimberly,
            #[serde(rename = "Polly.Salli")]
            Salli,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
            MaleChild(MaleChild),
            Female(Female),
        }

        impl VoicePrice for Voice {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-US-Standard-C")]
            StandardC,
            #[serde(rename = "Google.en-US-Standard-E")]
            StandardE,
            #[serde(rename = "Google.en-US-Standard-F")]
            StandardF,
            #[serde(rename = "Google.en-US-Standard-G")]
            StandardG,
            #[serde(rename = "Google.en-US-Standard-H")]
            StandardH,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Google(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Standard-A")]
            StandardA,
            #[serde(rename = "Google.en-US-Standard-B")]
            StandardB,
            #[serde(rename = "Google.en-US-Standard-D")]
            StandardD,
            #[serde(rename = "Google.en-US-Standard-I")]
            StandardI,
            #[serde(rename = "Google.en-US-Standard-J")]
            StandardJ,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(STANDARD_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Standard(super::Voice::Google(
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
        Polly(polly::Voice),
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
        pub enum Male {
            #[serde(rename = "Google.en-US-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.en-US-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.en-US-Neural2-I")]
            Neural2I,
            #[serde(rename = "Google.en-US-Neural2-J")]
            Neural2J,
            #[serde(rename = "Google.en-US-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.en-US-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.en-US-Wavenet-D")]
            WavenetD,
            #[serde(rename = "Google.en-US-Wavenet-I")]
            WavenetI,
            #[serde(rename = "Google.en-US-Wavenet-J")]
            WavenetJ,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-US-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.en-US-Neural2-E")]
            Neural2E,
            #[serde(rename = "Google.en-US-Neural2-F")]
            Neural2F,
            #[serde(rename = "Google.en-US-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.en-US-Neural2-H")]
            Neural2H,
            #[serde(rename = "Google.en-US-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.en-US-Wavenet-E")]
            WavenetE,
            #[serde(rename = "Google.en-US-Wavenet-F")]
            WavenetF,
            #[serde(rename = "Google.en-US-Wavenet-G")]
            WavenetG,
            #[serde(rename = "Google.en-US-Wavenet-H")]
            WavenetH,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Google(
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

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum FemaleChild {
            #[serde(rename = "Polly.Ivy-Neural")]
            IvyNeural,
        }

        impl VoicePrice for FemaleChild {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<FemaleChild> for crate::twiml::Voice {
            fn from(value: FemaleChild) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::FemaleChild(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum MaleChild {
            #[serde(rename = "Polly.Justin-Neural")]
            JustinNeural,
            #[serde(rename = "Polly.Kevin-Neural")]
            KevinNeural,
        }

        impl VoicePrice for MaleChild {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<MaleChild> for crate::twiml::Voice {
            fn from(value: MaleChild) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::MaleChild(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Gregory-Neural")]
            GregoryNeural,
            #[serde(rename = "Polly.Joey-Neural")]
            JoeyNeural,
            #[serde(rename = "Polly.Matthew-Neural")]
            MatthewNeural,
            #[serde(rename = "Polly.Stephen-Neural")]
            StephenNeural,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Danielle-Neural")]
            DanielleNeural,
            #[serde(rename = "Polly.Joanna-Neural")]
            JoannaNeural,
            #[serde(rename = "Polly.Kendra-Neural")]
            KendraNeural,
            #[serde(rename = "Polly.Kimberly-Neural")]
            KimberlyNeural,
            #[serde(rename = "Polly.Ruth-Neural")]
            RuthNeural,
            #[serde(rename = "Polly.Salli-Neural")]
            SalliNeural,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(NEURAL_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Neural(super::Voice::Polly(
                    Voice::Female(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            FemaleChild(FemaleChild),
            MaleChild(MaleChild),
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
        Polly(polly::Voice),
    }

    impl VoicePrice for Voice {
        fn price(&self) -> Option<f32> {
            Some(NEURAL_VOICE_PRICE)
        }
    }
}

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Chirp3-HD-Charon")]
            Chirp3HdCharon,
            #[serde(rename = "Google.en-US-Chirp3-HD-Fenrir")]
            Chirp3HdFenrir,
            #[serde(rename = "Google.en-US-Chirp3-HD-Orus")]
            Chirp3HdOrus,
            #[serde(rename = "Google.en-US-Chirp3-HD-Puck")]
            Chirp3HdPuck,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Google(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-US-Chirp3-HD-Aoede")]
            Chirp3HdAoede,
            #[serde(rename = "Google.en-US-Chirp3-HD-Kore")]
            Chirp3HdKore,
            #[serde(rename = "Google.en-US-Chirp3-HD-Leda")]
            Chirp3HdLeda,
            #[serde(rename = "Google.en-US-Chirp3-HD-Zephyr")]
            Chirp3HdZephyr,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Google(
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
                Some(GENERATIVE_VOICE_PRICE)
            }
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Polly.Matthew-Generative")]
            MatthewGenerative,
            #[serde(rename = "Polly.Stephen-Generative")]
            StephenGenerative,
        }

        impl VoicePrice for Male {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Male> for crate::twiml::Voice {
            fn from(value: Male) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Polly(
                    Voice::Male(value),
                )))
            }
        }

        #[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Polly.Danielle-Generative")]
            DanielleGenerative,
            #[serde(rename = "Polly.Joanna-Generative")]
            JoannaGenerative,
            #[serde(rename = "Polly.Ruth-Generative")]
            RuthGenerative,
        }

        impl VoicePrice for Female {
            fn price(&self) -> Option<f32> {
                Some(GENERATIVE_VOICE_PRICE)
            }
        }

        impl From<Female> for crate::twiml::Voice {
            fn from(value: Female) -> Self {
                Self::EnUs(super::super::Voice::Generative(super::Voice::Polly(
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
                Some(GENERATIVE_VOICE_PRICE)
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
            Some(GENERATIVE_VOICE_PRICE)
        }
    }
}

#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
    Neural(neural::Voice),
    Generative(generative::Voice),
}
impl VoicePrice for Voice {
    fn price(&self) -> Option<f32> {
        match self {
            Voice::Standard(_) => Some(STANDARD_VOICE_PRICE),
            Voice::Neural(_) => Some(NEURAL_VOICE_PRICE),
            Voice::Generative(_) => Some(GENERATIVE_VOICE_PRICE),
        }
    }
}

pub mod female {
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Ivy: Female = Female::Ivy;
            pub const Joanna: Female = Female::Joanna;
            pub const Kendra: Female = Female::Kendra;
            pub const Kimberly: Female = Female::Kimberly;
            pub const Salli: Female = Female::Salli;
        }
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardC: Female = Female::StandardC;
            pub const StandardE: Female = Female::StandardE;
            pub const StandardF: Female = Female::StandardF;
            pub const StandardG: Female = Female::StandardG;
            pub const StandardH: Female = Female::StandardH;
        }
    }
    pub mod neural {
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2C: Female = Female::Neural2C;
            pub const Neural2E: Female = Female::Neural2E;
            pub const Neural2F: Female = Female::Neural2F;
            pub const Neural2G: Female = Female::Neural2G;
            pub const Neural2H: Female = Female::Neural2H;
            pub const WavenetC: Female = Female::WavenetC;
            pub const WavenetE: Female = Female::WavenetE;
            pub const WavenetF: Female = Female::WavenetF;
            pub const WavenetG: Female = Female::WavenetG;
            pub const WavenetH: Female = Female::WavenetH;
        }
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const DanielleNeural: Female = Female::DanielleNeural;
            pub const JoannaNeural: Female = Female::JoannaNeural;
            pub const KendraNeural: Female = Female::KendraNeural;
            pub const KimberlyNeural: Female = Female::KimberlyNeural;
            pub const RuthNeural: Female = Female::RuthNeural;
            pub const SalliNeural: Female = Female::SalliNeural;
        }
    }
    pub mod generative {
        pub mod google {
            use super::super::super::generative::google::*;
            pub const Chirp3HdAoede: Female = Female::Chirp3HdAoede;
            pub const Chirp3HdKore: Female = Female::Chirp3HdKore;
            pub const Chirp3HdLeda: Female = Female::Chirp3HdLeda;
            pub const Chirp3HdZephyr: Female = Female::Chirp3HdZephyr;
        }
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const DanielleGenerative: Female = Female::DanielleGenerative;
            pub const JoannaGenerative: Female = Female::JoannaGenerative;
            pub const RuthGenerative: Female = Female::RuthGenerative;
        }
    }
}

pub mod female_child {
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const IvyNeural: FemaleChild = FemaleChild::IvyNeural;
        }
    }
}

pub mod male {
    pub mod standard {
        pub mod google {
            use super::super::super::standard::google::*;
            pub const StandardA: Male = Male::StandardA;
            pub const StandardB: Male = Male::StandardB;
            pub const StandardD: Male = Male::StandardD;
            pub const StandardI: Male = Male::StandardI;
            pub const StandardJ: Male = Male::StandardJ;
        }
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Joey: Male = Male::Joey;
            pub const Justin: Male = Male::Justin;
            pub const Matthew: Male = Male::Matthew;
        }
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const GregoryNeural: Male = Male::GregoryNeural;
            pub const JoeyNeural: Male = Male::JoeyNeural;
            pub const MatthewNeural: Male = Male::MatthewNeural;
            pub const StephenNeural: Male = Male::StephenNeural;
        }
        pub mod google {
            use super::super::super::neural::google::*;
            pub const Neural2A: Male = Male::Neural2A;
            pub const Neural2D: Male = Male::Neural2D;
            pub const Neural2I: Male = Male::Neural2I;
            pub const Neural2J: Male = Male::Neural2J;
            pub const WavenetA: Male = Male::WavenetA;
            pub const WavenetB: Male = Male::WavenetB;
            pub const WavenetD: Male = Male::WavenetD;
            pub const WavenetI: Male = Male::WavenetI;
            pub const WavenetJ: Male = Male::WavenetJ;
        }
    }
    pub mod generative {
        pub mod google {
            use super::super::super::generative::google::*;
            pub const Chirp3HdCharon: Male = Male::Chirp3HdCharon;
            pub const Chirp3HdFenrir: Male = Male::Chirp3HdFenrir;
            pub const Chirp3HdOrus: Male = Male::Chirp3HdOrus;
            pub const Chirp3HdPuck: Male = Male::Chirp3HdPuck;
        }
        pub mod polly {
            use super::super::super::generative::polly::*;
            pub const MatthewGenerative: Male = Male::MatthewGenerative;
            pub const StephenGenerative: Male = Male::StephenGenerative;
        }
    }
}

pub mod male_child {
    pub mod standard {
        pub mod polly {
            use super::super::super::standard::polly::*;
            pub const Kevin: MaleChild = MaleChild::Kevin;
        }
    }
    pub mod neural {
        pub mod polly {
            use super::super::super::neural::polly::*;
            pub const JustinNeural: MaleChild = MaleChild::JustinNeural;
            pub const KevinNeural: MaleChild = MaleChild::KevinNeural;
        }
    }
}
