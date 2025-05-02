// Voice module for kn-IN language
use serde::{Serialize, Deserialize};

pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.kn-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.kn-IN-Standard-C")]
            StandardC,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.kn-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.kn-IN-Standard-D")]
            StandardD,
        }

        #[amass::amass_telety(crate::twiml::voices::kn_in::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::kn_in::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.kn-IN-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::kn_in::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::kn_in::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.kn-IN-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.kn-IN-Wavenet-C")]
            WavenetC,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.kn-IN-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.kn-IN-Wavenet-D")]
            WavenetD,
        }

        #[amass::amass_telety(crate::twiml::voices::kn_in::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::kn_in::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::kn_in)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
    Generative(generative::Voice),
    Neural(neural::Voice),
}
