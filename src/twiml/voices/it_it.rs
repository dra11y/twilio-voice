// Voice module for it-IT language
use serde::{Serialize, Deserialize};

pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Bianca-Generative")]
            BiancaGenerative,
        }

        #[amass::amass_telety(crate::twiml::voices::it_it::generative::polly)]
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
            #[serde(rename = "Google.it-IT-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.it-IT-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.it-IT-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::it_it::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::it_it::generative)]
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
            #[serde(rename = "Polly.Bianca-Neural")]
            BiancaNeural,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Adriano-Neural")]
            AdrianoNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::it_it::neural::polly)]
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
            #[serde(rename = "Google.it-IT-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.it-IT-Wavenet-E")]
            WavenetE,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.it-IT-Neural2-F")]
            Neural2F,
            #[serde(rename = "Google.it-IT-Wavenet-F")]
            WavenetF,
        }

        #[amass::amass_telety(crate::twiml::voices::it_it::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::it_it::neural)]
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
            #[serde(rename = "Google.it-IT-Standard-A")]
            StandardA,
            #[serde(rename = "Google.it-IT-Standard-E")]
            StandardE,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.it-IT-Standard-F")]
            StandardF,
        }

        #[amass::amass_telety(crate::twiml::voices::it_it::standard::google)]
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
            #[serde(rename = "Polly.Bianca")]
            Bianca,
            #[serde(rename = "Polly.Carla")]
            Carla,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Giorgio")]
            Giorgio,
        }

        #[amass::amass_telety(crate::twiml::voices::it_it::standard::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::it_it::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::it_it)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Neural(neural::Voice),
    Standard(standard::Voice),
}
