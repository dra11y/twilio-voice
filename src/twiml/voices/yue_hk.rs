// Voice module for yue-HK language
use serde::{Serialize, Deserialize};

pub mod standard {
    use serde::{Serialize, Deserialize};

    pub mod google {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Female {
            #[serde(rename = "Google.yue-HK-Standard-A")]
            StandardA,
            #[serde(rename = "Google.yue-HK-Standard-C")]
            StandardC,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Male {
            #[serde(rename = "Google.yue-HK-Standard-B")]
            StandardB,
            #[serde(rename = "Google.yue-HK-Standard-D")]
            StandardD,
        }

        #[amass::amass_telety(crate::twiml::voices::yue_hk::standard::google)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }
    #[amass::amass_telety(crate::twiml::voices::yue_hk::standard)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}
#[amass::amass_telety(crate::twiml::voices::yue_hk)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
}
