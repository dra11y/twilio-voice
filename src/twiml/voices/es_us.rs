// Voice module for es-US language
use serde::{Serialize, Deserialize};

pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Lupe-Neural")]
            LupeNeural,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Pedro-Neural")]
            PedroNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::es_us::neural::polly)]
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
            #[serde(rename = "Google.es-US-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.es-US-Wavenet-A")]
            WavenetA,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.es-US-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.es-US-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.es-US-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.es-US-Wavenet-C")]
            WavenetC,
        }

        #[amass::amass_telety(crate::twiml::voices::es_us::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::es_us::neural)]
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
            #[serde(rename = "Google.es-US-Standard-A")]
            StandardA,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.es-US-Standard-B")]
            StandardB,
            #[serde(rename = "Google.es-US-Standard-C")]
            StandardC,
        }

        #[amass::amass_telety(crate::twiml::voices::es_us::standard::google)]
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
            #[serde(rename = "Polly.Lupe")]
            Lupe,
            #[serde(rename = "Polly.Penelope")]
            Penelope,
            #[serde(rename = "Polly.Penélope")]
            Penélope,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Miguel")]
            Miguel,
        }

        #[amass::amass_telety(crate::twiml::voices::es_us::standard::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::es_us::standard)]
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
            #[serde(rename = "Google.es-US-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.es-US-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.es-US-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.es-US-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.es-US-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.es-US-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.es-US-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.es-US-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::es_us::generative::google)]
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
            #[serde(rename = "Polly.Lupe-Generative")]
            LupeGenerative,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Pedro-Generative")]
            PedroGenerative,
        }

        #[amass::amass_telety(crate::twiml::voices::es_us::generative::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::es_us::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::es_us)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
    Generative(generative::Voice),
}
