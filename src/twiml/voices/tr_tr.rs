// Voice module for tr-TR language
use serde::{Serialize, Deserialize};

pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.tr-TR-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.tr-TR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.tr-TR-Wavenet-D")]
            WavenetD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.tr-TR-Wavenet-E")]
            WavenetE,
        }

        #[amass::amass_telety(crate::twiml::voices::tr_tr::neural::google)]
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
            #[serde(rename = "Polly.Burcu-Neural")]
            BurcuNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::tr_tr::neural::polly)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::tr_tr::neural)]
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
            #[serde(rename = "Polly.Filiz")]
            Filiz,
        }

        #[amass::amass_telety(crate::twiml::voices::tr_tr::standard::polly)]
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
            #[serde(rename = "Google.tr-TR-Standard-A")]
            StandardA,
            #[serde(rename = "Google.tr-TR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.tr-TR-Standard-D")]
            StandardD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Standard-B")]
            StandardB,
            #[serde(rename = "Google.tr-TR-Standard-E")]
            StandardE,
        }

        #[amass::amass_telety(crate::twiml::voices::tr_tr::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::tr_tr::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
pub mod generative {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.tr-TR-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::tr_tr::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::tr_tr::generative)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::tr_tr)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
    Generative(generative::Voice),
}
