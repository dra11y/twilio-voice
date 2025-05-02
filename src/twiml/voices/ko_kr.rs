// Voice module for ko-KR language
use serde::{Serialize, Deserialize};

pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.ko-KR-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.ko-KR-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.ko-KR-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.ko-KR-Wavenet-B")]
            WavenetB,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.ko-KR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.ko-KR-Wavenet-D")]
            WavenetD,
        }

        #[amass::amass_telety(crate::twiml::voices::ko_kr::neural::google)]
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
            #[serde(rename = "Polly.Seoyeon-Neural")]
            SeoyeonNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::ko_kr::neural::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ko_kr::neural)]
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
            #[serde(rename = "Google.ko-KR-Standard-A")]
            StandardA,
            #[serde(rename = "Google.ko-KR-Standard-B")]
            StandardB,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.ko-KR-Standard-D")]
            StandardD,
        }

        #[amass::amass_telety(crate::twiml::voices::ko_kr::standard::google)]
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
            #[serde(rename = "Polly.Seoyeon")]
            Seoyeon,
        }

        #[amass::amass_telety(crate::twiml::voices::ko_kr::standard::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ko_kr::standard)]
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
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.ko-KR-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::ko_kr::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ko_kr::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::ko_kr)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
    Generative(generative::Voice),
}
