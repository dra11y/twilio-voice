use serde::{Serialize, Deserialize};

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
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


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::TrTr(super::super::Voice::Generative(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
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


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::TrTr(super::super::Voice::Generative(super::Voice::Google(
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
            #[serde(rename = "Google.tr-TR-Standard-A")]
            StandardA,
            #[serde(rename = "Google.tr-TR-Standard-C")]
            StandardC,
            #[serde(rename = "Google.tr-TR-Standard-D")]
            StandardD,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::TrTr(super::super::Voice::Standard(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Standard-B")]
            StandardB,
            #[serde(rename = "Google.tr-TR-Standard-E")]
            StandardE,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::TrTr(super::super::Voice::Standard(super::Voice::Google(
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

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.tr-TR-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.tr-TR-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.tr-TR-Wavenet-D")]
            WavenetD,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::TrTr(super::super::Voice::Neural(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.tr-TR-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.tr-TR-Wavenet-E")]
            WavenetE,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::TrTr(super::super::Voice::Neural(super::Voice::Google(
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
    Standard(standard::Voice),
    Neural(neural::Voice),
}
