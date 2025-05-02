// Voice module for te-IN language
use serde::{Serialize, Deserialize};

pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.te-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.te-IN-Standard-C")]
            StandardC,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.te-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.te-IN-Standard-D")]
            StandardD,
        }

        #[amass::amass_telety(crate::twiml::voices::te_in::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::te_in::standard)]
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
            #[serde(rename = "Google.te-IN-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.te-IN-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.te-IN-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.te-IN-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.te-IN-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.te-IN-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.te-IN-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.te-IN-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::te_in::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::te_in::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::te_in)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
    Generative(generative::Voice),
}
