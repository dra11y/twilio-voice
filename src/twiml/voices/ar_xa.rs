// Voice module for ar-XA language
use serde::{Serialize, Deserialize};

pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.ar-XA-Standard-A")]
            StandardA,
            #[serde(rename = "Google.ar-XA-Standard-D")]
            StandardD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ar-XA-Standard-B")]
            StandardB,
            #[serde(rename = "Google.ar-XA-Standard-C")]
            StandardC,
        }

        #[amass::amass_telety(crate::twiml::voices::ar_xa::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ar_xa::standard)]
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
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.ar-XA-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::ar_xa::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ar_xa::generative)]
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
            #[serde(rename = "Google.ar-XA-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.ar-XA-Wavenet-D")]
            WavenetD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ar-XA-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.ar-XA-Wavenet-C")]
            WavenetC,
        }

        #[amass::amass_telety(crate::twiml::voices::ar_xa::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ar_xa::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::ar_xa)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
    Generative(generative::Voice),
    Neural(neural::Voice),
}
