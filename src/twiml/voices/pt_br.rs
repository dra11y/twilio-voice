// Voice module for pt-BR language
use serde::{Serialize, Deserialize};

pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.pt-BR-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::pt_br::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::pt_br::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Camila")]
            Camila,
            #[serde(rename = "Polly.Vitoria")]
            Vitoria,
            #[serde(rename = "Polly.Vitória")]
            Vitória,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Ricardo")]
            Ricardo,
        }

        #[amass::amass_telety(crate::twiml::voices::pt_br::standard::polly)]
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
            #[serde(rename = "Google.pt-BR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.pt-BR-Standard-D")]
            StandardD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.pt-BR-Standard-B")]
            StandardB,
            #[serde(rename = "Google.pt-BR-Standard-E")]
            StandardE,
        }

        #[amass::amass_telety(crate::twiml::voices::pt_br::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::pt_br::standard)]
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
            #[serde(rename = "Google.pt-BR-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.pt-BR-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.pt-BR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.pt-BR-Wavenet-D")]
            WavenetD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.pt-BR-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.pt-BR-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.pt-BR-Wavenet-E")]
            WavenetE,
        }

        #[amass::amass_telety(crate::twiml::voices::pt_br::neural::google)]
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
            #[serde(rename = "Polly.Camila-Neural")]
            CamilaNeural,
            #[serde(rename = "Polly.Vitoria-Neural")]
            VitoriaNeural,
            #[serde(rename = "Polly.Vitória-Neural")]
            VitóriaNeural,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Polly.Thiago-Neural")]
            ThiagoNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::pt_br::neural::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::pt_br::neural)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::pt_br)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Standard(standard::Voice),
    Neural(neural::Voice),
}
