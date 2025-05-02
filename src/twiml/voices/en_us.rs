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
            #[serde(rename = "Google.en-US-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.en-US-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.en-US-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.en-US-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EnUs(super::super::Voice::Generative(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.en-US-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.en-US-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.en-US-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EnUs(super::super::Voice::Generative(super::Voice::Google(
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

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.en-US-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.en-US-Neural2-E")]
            Neural2E,
            #[serde(rename = "Google.en-US-Neural2-F")]
            Neural2F,
            #[serde(rename = "Google.en-US-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.en-US-Neural2-H")]
            Neural2H,
            #[serde(rename = "Google.en-US-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.en-US-Wavenet-E")]
            WavenetE,
            #[serde(rename = "Google.en-US-Wavenet-F")]
            WavenetF,
            #[serde(rename = "Google.en-US-Wavenet-G")]
            WavenetG,
            #[serde(rename = "Google.en-US-Wavenet-H")]
            WavenetH,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EnUs(super::super::Voice::Neural(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.en-US-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.en-US-Neural2-I")]
            Neural2I,
            #[serde(rename = "Google.en-US-Neural2-J")]
            Neural2J,
            #[serde(rename = "Google.en-US-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.en-US-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.en-US-Wavenet-D")]
            WavenetD,
            #[serde(rename = "Google.en-US-Wavenet-I")]
            WavenetI,
            #[serde(rename = "Google.en-US-Wavenet-J")]
            WavenetJ,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EnUs(super::super::Voice::Neural(super::Voice::Google(
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

pub mod standard {
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
            #[serde(rename = "Google.en-US-Standard-C")]
            StandardC,
            #[serde(rename = "Google.en-US-Standard-E")]
            StandardE,
            #[serde(rename = "Google.en-US-Standard-F")]
            StandardF,
            #[serde(rename = "Google.en-US-Standard-G")]
            StandardG,
            #[serde(rename = "Google.en-US-Standard-H")]
            StandardH,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EnUs(super::super::Voice::Standard(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.en-US-Standard-A")]
            StandardA,
            #[serde(rename = "Google.en-US-Standard-B")]
            StandardB,
            #[serde(rename = "Google.en-US-Standard-D")]
            StandardD,
            #[serde(rename = "Google.en-US-Standard-I")]
            StandardI,
            #[serde(rename = "Google.en-US-Standard-J")]
            StandardJ,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EnUs(super::super::Voice::Standard(super::Voice::Google(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Generative(generative::Voice),
    Neural(neural::Voice),
    Standard(standard::Voice),
}
