// Voice module for cmn-CN language
use serde::{Serialize, Deserialize};

pub mod neural {
    use serde::{Serialize, Deserialize};

    pub mod polly {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Polly.Zhiyu-Neural")]
            ZhiyuNeural,
        }

        #[amass::amass_telety(crate::twiml::voices::cmn_cn::neural::polly)]
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
            #[serde(rename = "Google.cmn-CN-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.cmn-CN-Wavenet-D")]
            WavenetD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.cmn-CN-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.cmn-CN-Wavenet-C")]
            WavenetC,
        }

        #[amass::amass_telety(crate::twiml::voices::cmn_cn::neural::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::cmn_cn::neural)]
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
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.cmn-CN-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }

        #[amass::amass_telety(crate::twiml::voices::cmn_cn::generative::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::cmn_cn::generative)]
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
            #[serde(rename = "Polly.Zhiyu")]
            Zhiyu,
        }

        #[amass::amass_telety(crate::twiml::voices::cmn_cn::standard::polly)]
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
            #[serde(rename = "Google.cmn-CN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.cmn-CN-Standard-D")]
            StandardD,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.cmn-CN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.cmn-CN-Standard-C")]
            StandardC,
        }

        #[amass::amass_telety(crate::twiml::voices::cmn_cn::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::cmn_cn::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::cmn_cn)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Generative(generative::Voice),
    Standard(standard::Voice),
}
