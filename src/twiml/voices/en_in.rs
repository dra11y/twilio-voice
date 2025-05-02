// Voice module for en-IN language
use serde::{Serialize, Deserialize};

pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Kajal-Generative")]
            KajalGenerative,
        }

        #[amass::amass_telety(crate::twiml::voices::en_in::generative::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }
    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.en-IN-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.en-IN-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.en-IN-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.en-IN-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.en-IN-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.en-IN-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.en-IN-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.en-IN-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::en_in::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::en_in::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Kajal-Neural")]
            KajalNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::en_in::neural::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }
    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.en-IN-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.en-IN-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.en-IN-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.en-IN-Wavenet-D")]
            WavenetD,
            #[serde(rename = "Google.en-IN-Wavenet-E")]
            WavenetE,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.en-IN-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.en-IN-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.en-IN-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.en-IN-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.en-IN-Wavenet-F")]
            WavenetF,
        }

        #[amass::amass_telety(crate::twiml::voices::en_in::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::en_in::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.en-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.en-IN-Standard-D")]
            StandardD,
            #[serde(rename = "Google.en-IN-Standard-E")]
            StandardE,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.en-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.en-IN-Standard-C")]
            StandardC,
            #[serde(rename = "Google.en-IN-Standard-F")]
            StandardF,
        }

        #[amass::amass_telety(crate::twiml::voices::en_in::standard::google)]
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
            #[serde(rename = "Polly.Aditi")]
            Aditi,
            #[serde(rename = "Polly.Raveena")]
            Raveena,
        }

        #[amass::amass_telety(crate::twiml::voices::en_in::standard::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::en_in::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::en_in)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Neural(neural::Voice),
    Standard(standard::Voice),
}
