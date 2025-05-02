// Voice module for ru-RU language
use serde::{Serialize, Deserialize};

pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.ru-RU-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::ru_ru::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ru_ru::generative)]
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
            #[serde(rename = "Google.ru-RU-Standard-A")]
            StandardA,
            #[serde(rename = "Google.ru-RU-Standard-C")]
            StandardC,
            #[serde(rename = "Google.ru-RU-Standard-E")]
            StandardE,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ru-RU-Standard-B")]
            StandardB,
            #[serde(rename = "Google.ru-RU-Standard-D")]
            StandardD,
        }

        #[amass::amass_telety(crate::twiml::voices::ru_ru::standard::google)]
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
            #[serde(rename = "Polly.Tatyana")]
            Tatyana,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Maxim")]
            Maxim,
        }

        #[amass::amass_telety(crate::twiml::voices::ru_ru::standard::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ru_ru::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.ru-RU-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.ru-RU-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.ru-RU-Wavenet-E")]
            WavenetE,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.ru-RU-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.ru-RU-Wavenet-D")]
            WavenetD,
        }

        #[amass::amass_telety(crate::twiml::voices::ru_ru::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::ru_ru::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::ru_ru)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Standard(standard::Voice),
    Neural(neural::Voice),
}
