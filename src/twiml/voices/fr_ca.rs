// Voice module for fr-CA language
use serde::{Serialize, Deserialize};

pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.fr-CA-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.fr-CA-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.fr-CA-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.fr-CA-Wavenet-C")]
            WavenetC,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.fr-CA-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.fr-CA-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.fr-CA-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.fr-CA-Wavenet-D")]
            WavenetD,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_ca::neural::google)]
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
            #[serde(rename = "Polly.Gabrielle-Neural")]
            GabrielleNeural,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Liam-Neural")]
            LiamNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_ca::neural::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::fr_ca::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.fr-CA-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_ca::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::fr_ca::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.fr-CA-Standard-A")]
            StandardA,
            #[serde(rename = "Google.fr-CA-Standard-C")]
            StandardC,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.fr-CA-Standard-B")]
            StandardB,
            #[serde(rename = "Google.fr-CA-Standard-D")]
            StandardD,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_ca::standard::google)]
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
            #[serde(rename = "Polly.Chantal")]
            Chantal,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_ca::standard::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::fr_ca::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::fr_ca)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Generative(generative::Voice),
    Standard(standard::Voice),
}
