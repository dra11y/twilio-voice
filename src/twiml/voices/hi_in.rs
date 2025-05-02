use serde::{Serialize, Deserialize};

pub mod neural {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Neural2-A")]
            Neural2A,
            #[serde(rename = "Google.hi-IN-Neural2-D")]
            Neural2D,
            #[serde(rename = "Google.hi-IN-Wavenet-A")]
            WavenetA,
            #[serde(rename = "Google.hi-IN-Wavenet-D")]
            WavenetD,
            #[serde(rename = "Google.hi-IN-Wavenet-E")]
            WavenetE,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::HiIn(super::super::Voice::Neural(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Neural2-B")]
            Neural2B,
            #[serde(rename = "Google.hi-IN-Neural2-C")]
            Neural2C,
            #[serde(rename = "Google.hi-IN-Wavenet-B")]
            WavenetB,
            #[serde(rename = "Google.hi-IN-Wavenet-C")]
            WavenetC,
            #[serde(rename = "Google.hi-IN-Wavenet-F")]
            WavenetF,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::HiIn(super::super::Voice::Neural(super::Voice::Google(
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
            #[serde(rename = "Google.hi-IN-Standard-A")]
            StandardA,
            #[serde(rename = "Google.hi-IN-Standard-D")]
            StandardD,
            #[serde(rename = "Google.hi-IN-Standard-E")]
            StandardE,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::HiIn(super::super::Voice::Standard(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Standard-B")]
            StandardB,
            #[serde(rename = "Google.hi-IN-Standard-C")]
            StandardC,
            #[serde(rename = "Google.hi-IN-Standard-F")]
            StandardF,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::HiIn(super::super::Voice::Standard(super::Voice::Google(
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

pub mod generative {
    use super::*;

    pub mod google {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Female {
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Aoede")]
            Chirp3HDAoede,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Kore")]
            Chirp3HDKore,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Leda")]
            Chirp3HDLeda,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Zephyr")]
            Chirp3HDZephyr,
        }


                            impl From<Female> for crate::Voice {
                                fn from(value: Female) -> Self {
                                    Self::HiIn(super::super::Voice::Generative(super::Voice::Google(
                                        Voice::Female(value),
                                    )))
                                }
                            }
                        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[non_exhaustive]
        pub enum Male {
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Charon")]
            Chirp3HDCharon,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Fenrir")]
            Chirp3HDFenrir,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Orus")]
            Chirp3HDOrus,
            #[serde(rename = "Google.hi-IN-Chirp3-HD-Puck")]
            Chirp3HDPuck,
        }


                            impl From<Male> for crate::Voice {
                                fn from(value: Male) -> Self {
                                    Self::HiIn(super::super::Voice::Generative(super::Voice::Google(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Voice {
    Neural(neural::Voice),
    Standard(standard::Voice),
    Generative(generative::Voice),
}
