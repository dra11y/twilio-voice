use serde::{Serialize, Deserialize};

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.lt-LT-Standard-B")]
            StandardB,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::LtLt(super::super::Voice::Standard(super::Voice::Google(
                                        Voice::Male(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Male(Male),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Standard(standard::Voice),
}
