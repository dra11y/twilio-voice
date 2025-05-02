use serde::{Serialize, Deserialize};

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
            #[serde(rename = "Google.es-ES-Standard-A")]
            StandardA,
            #[serde(rename = "Google.es-ES-Standard-F")]
            StandardF,
            #[serde(rename = "Google.es-ES-Standard-H")]
            StandardH,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EsEs(super::super::Voice::Standard(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.es-ES-Standard-E")]
            StandardE,
            #[serde(rename = "Google.es-ES-Standard-G")]
            StandardG,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EsEs(super::super::Voice::Standard(super::Voice::Google(
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
            #[serde(rename = "Google.es-ES-Neural2-H")]
            Neural2H,
            #[serde(rename = "Google.es-ES-Wavenet-F")]
            WavenetF,
            #[serde(rename = "Google.es-ES-Wavenet-H")]
            WavenetH,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EsEs(super::super::Voice::Neural(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.es-ES-Neural2-G")]
            Neural2G,
            #[serde(rename = "Google.es-ES-Wavenet-E")]
            WavenetE,
            #[serde(rename = "Google.es-ES-Wavenet-G")]
            WavenetG,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EsEs(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Google.es-ES-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.es-ES-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::EsEs(super::super::Voice::Generative(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
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


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::EsEs(super::super::Voice::Generative(super::Voice::Google(
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
    Standard(standard::Voice),
    Neural(neural::Voice),
    Generative(generative::Voice),
}
