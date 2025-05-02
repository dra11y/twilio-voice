use serde::{Serialize, Deserialize};

pub mod generative {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-AU-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.en-AU-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.en-AU-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.en-AU-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EnAu(super::super::Voice::Generative(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-AU-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.en-AU-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.en-AU-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.en-AU-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EnAu(super::super::Voice::Generative(super::Voice::Google(
                                        Voice::Male(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}

pub mod neural {
    use super::*;

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
        }
    }

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-AU-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.en-AU-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.en-AU-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.en-AU-Wavenet-C")]
            WavenetC,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EnAu(super::super::Voice::Neural(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-AU-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.en-AU-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.en-AU-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.en-AU-Wavenet-D")]
            WavenetD,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EnAu(super::super::Voice::Neural(super::Voice::Google(
                                        Voice::Male(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Polly(polly::Voice),
        Google(google::Voice),
    }
}

pub mod standard {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-AU-Standard-A")]
            StandardA,
            #[serde(rename = "Google.en-AU-Standard-C")]
            StandardC,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EnAu(super::super::Voice::Standard(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-AU-Standard-B")]
            StandardB,
            #[serde(rename = "Google.en-AU-Standard-D")]
            StandardD,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EnAu(super::super::Voice::Standard(super::Voice::Google(
                                        Voice::Male(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
            Female(Female),
            Male(Male),
        }
    }

    pub mod polly {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Voice {
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Voice {
        Google(google::Voice),
        Polly(polly::Voice),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Neural(neural::Voice),
    Standard(standard::Voice),
}
