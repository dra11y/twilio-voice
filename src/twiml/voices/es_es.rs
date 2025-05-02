// Voice module for es-ES language
use serde::{Serialize, Deserialize};

pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.es-ES-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.es-ES-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::es_es::generative::google)]
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
            #[serde(rename = "Polly.Lucia-Generative")]
            LuciaGenerative,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Sergio-Generative")]
            SergioGenerative,
        }

        #[amass::amass_telety(crate::twiml::voices::es_es::generative::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::es_es::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Conchita")]
            Conchita,
            #[serde(rename = "Polly.Lucia")]
            Lucia,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Enrique")]
            Enrique,
        }

        #[amass::amass_telety(crate::twiml::voices::es_es::standard::polly)]
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
            #[serde(rename = "Google.es-ES-Standard-A")]
            StandardA,
            #[serde(rename = "Google.es-ES-Standard-F")]
            StandardF,
            #[serde(rename = "Google.es-ES-Standard-H")]
            StandardH,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.es-ES-Standard-E")]
            StandardE,
            #[serde(rename = "Google.es-ES-Standard-G")]
            StandardG,
        }

        #[amass::amass_telety(crate::twiml::voices::es_es::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::es_es::standard)]
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
            #[serde(rename = "Polly.Lucia-Neural")]
            LuciaNeural,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Sergio-Neural")]
            SergioNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::es_es::neural::polly)]
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
            #[serde(rename = "Google.es-ES-Neural2-H")]
            Neural2H,
            #[serde(rename = "Google.es-ES-Wavenet-F")]
            WavenetF,
            #[serde(rename = "Google.es-ES-Wavenet-H")]
            WavenetH,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.es-ES-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.es-ES-Wavenet-E")]
            WavenetE,
            #[serde(rename = "Google.es-ES-Wavenet-G")]
            WavenetG,
        }

        #[amass::amass_telety(crate::twiml::voices::es_es::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::es_es::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::es_es)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Standard(standard::Voice),
    Neural(neural::Voice),
}
