// Voice module for en-US language
use serde::{Serialize, Deserialize};

pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Danielle-Generative")]
            DanielleGenerative,
            #[serde(rename = "Polly.Joanna-Generative")]
            JoannaGenerative,
            #[serde(rename = "Polly.Ruth-Generative")]
            RuthGenerative,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Matthew-Generative")]
            MatthewGenerative,
            #[serde(rename = "Polly.Stephen-Generative")]
            StephenGenerative,
        }

        #[amass::amass_telety(crate::twiml::voices::en_us::generative::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.en-US-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.en-US-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.en-US-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.en-US-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.en-US-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.en-US-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.en-US-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.en-US-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::en_us::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::en_us::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

        #[amass::amass_telety(crate::twiml::voices::en_us::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Danielle-Neural")]
            DanielleNeural,
            #[serde(rename = "Polly.Ivy-Neural")]
            IvyNeural,
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Gregory-Neural")]
            GregoryNeural,
            #[serde(rename = "Polly.Joey-Neural")]
            JoeyNeural,
            #[serde(rename = "Polly.Justin-Neural")]
            JustinNeural,
            #[serde(rename = "Polly.Kevin-Neural")]
            KevinNeural,
            #[serde(rename = "Polly.Matthew-Neural")]
            MatthewNeural,
            #[serde(rename = "Polly.Stephen-Neural")]
            StephenNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::en_us::neural::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::en_us::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

        #[amass::amass_telety(crate::twiml::voices::en_us::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Joey")]
            Joey,
            #[serde(rename = "Polly.Justin")]
            Justin,
            #[serde(rename = "Polly.Kevin")]
            Kevin,
            #[serde(rename = "Polly.Matthew")]
            Matthew,
        }

        #[amass::amass_telety(crate::twiml::voices::en_us::standard::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::en_us::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::en_us)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Neural(neural::Voice),
    Standard(standard::Voice),
}
