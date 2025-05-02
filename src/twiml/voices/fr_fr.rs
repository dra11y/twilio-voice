// Voice module for fr-FR language
use serde::{Serialize, Deserialize};

pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Lea-Generative")]
            LeaGenerative,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Rémi-Generative")]
            RémiGenerative,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_fr::generative::polly)]
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
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.fr-FR-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_fr::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::fr_fr::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Celine")]
            Celine,
            #[serde(rename = "Polly.Céline")]
            Céline,
            #[serde(rename = "Polly.Lea")]
            Lea,
            #[serde(rename = "Polly.Léa")]
            Léa,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Mathieu")]
            Mathieu,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_fr::standard::polly)]
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
            #[serde(rename = "Google.fr-FR-Standard-F")]
            StandardF,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.fr-FR-Standard-G")]
            StandardG,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_fr::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::fr_fr::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.fr-FR-Neural2-F")]
            Neural2F,
            #[serde(rename = "Google.fr-FR-Wavenet-F")]
            WavenetF,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.fr-FR-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.fr-FR-Wavenet-G")]
            WavenetG,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_fr::neural::google)]
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
            #[serde(rename = "Polly.Lea-Neural")]
            LeaNeural,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Remi-Neural")]
            RemiNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::fr_fr::neural::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::fr_fr::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::fr_fr)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Standard(standard::Voice),
    Neural(neural::Voice),
}
