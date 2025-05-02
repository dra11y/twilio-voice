use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::TwilioLanguage;

/// TwiML Voice: <Say>
/// https://www.twilio.com/docs/voice/twiml/say
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Say {
    #[serde(rename = "$value")]
    text: String,
    #[serde(rename = "@language")]
    #[builder(default)]
    language: TwilioLanguage,
    #[serde(rename = "@voice")]
    voice: super::voices::Voice,
    #[serde(rename = "@loop")]
    #[builder(default = 1)]
    loop_count: u32,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub enum StandardVoice {
//     #[serde(rename = "Google.en-US-Standard-C")]
//     FemaleGoogleC,
//     #[serde(rename = "Google.en-US-Standard-F")]
//     FemaleGoogleF,
//     #[serde(rename = "Google.en-US-Standard-G")]
//     FemaleGoogleG,
//     #[serde(rename = "Google.en-US-Standard-H")]
//     FemaleGoogleH,
//     #[serde(rename = "Polly.Ivy")]
//     FemalePollyIvy,
//     #[serde(rename = "Polly.Joanna")]
//     FemalePollyJoanna,
//     #[serde(rename = "Polly.Kendra")]
//     FemalePollyKendra,
//     #[serde(rename = "Polly.Kimberly")]
//     FemalePollyKimberly,
//     #[serde(rename = "Polly.Salli")]
//     FemalePollySalli,
//     #[serde(rename = "Google.en-US-Standard-A")]
//     MaleGoogleA,
//     #[serde(rename = "Google.en-US-Standard-B")]
//     MaleGoogleB,
//     #[serde(rename = "Google.en-US-Standard-D")]
//     MaleGoogleD,
//     #[serde(rename = "Google.en-US-Standard-I")]
//     MaleGoogleI,
//     #[serde(rename = "Google.en-US-Standard-J")]
//     MaleGoogleJ,
//     #[serde(rename = "Polly.Joey")]
//     MalePollyJoey,
//     #[serde(rename = "Polly.Justin")]
//     MalePollyJustin,
//     #[serde(rename = "Polly.Matthew")]
//     MalePollyMatthew,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub enum GenerativeVoice {
//     #[serde(rename = "Google.en-US-Chirp3-HD-Aoede")]
//     FemaleGoogleChirp3Aoede,
//     #[serde(rename = "Google.en-US-Chirp3-HD-Kore")]
//     FemaleGoogleChirp3Kore,
//     #[serde(rename = "Google.en-US-Chirp3-HD-Leda")]
//     FemaleGoogleChirp3Leda,
//     #[serde(rename = "Google.en-US-Chirp3-HD-Zephyr")]
//     FemaleGoogleChirp3Zephyr,
//     #[serde(rename = "Google.Danielle-Generative")]
//     FemalePollyDanielleGenerative,
//     #[serde(rename = "Polly.Joanna-Generative")]
//     FemalePollyJoannaGenerative,
//     #[serde(rename = "Polly.Ruth-Generative")]
//     FemalePollyRuthGenerative,
//     #[serde(rename = "Google.en-US-Chirp3-HD-Charon")]
//     MaleGoogleChirp3Charon,
//     #[serde(rename = "Google.en-US-Chirp3-HD-Fenrir")]
//     MaleGoogleChirp3Fenrir,
//     #[serde(rename = "Google.en-US-Chirp3-HD-Orus")]
//     MaleGoogleChirp3Orus,
//     #[serde(rename = "Google.en-US-Chirp3-HD-Puck")]
//     MaleGoogleChirp3Puck,
//     #[serde(rename = "Polly.Matthew-Generative")]
//     MalePollyMatthewGenerative,
//     #[serde(rename = "Polly.Stephen-Generative")]
//     MalePollyStephenGenerative,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// pub enum NeuralVoice {
//     #[serde(rename = "Google.en-US-Neural2-A")]
//     MaleGoogleNeural2A,
//     #[serde(rename = "Google.en-US-Neural2-D")]
//     MaleGoogleNeural2D,
//     #[serde(rename = "Google.en-US-Neural2-I")]
//     MaleGoogleNeural2I,
//     #[serde(rename = "Google.en-US-Neural2-J")]
//     MaleGoogleNeural2J,
//     #[serde(rename = "Google.en-US-Wavenet-A")]
//     MaleGoogleWavenetA,
//     #[serde(rename = "Google.en-US-Wavenet-B")]
//     MaleGoogleWavenetB,
//     #[serde(rename = "Google.en-US-Wavenet-D")]
//     MaleGoogleWavenetD,
//     #[serde(rename = "Google.en-US-Wavenet-I")]
//     MaleGoogleWavenetI,
//     #[serde(rename = "Google.en-US-Wavenet-J")]
//     MaleGoogleWavenetJ,
//     #[serde(rename = "Polly.Gregory-Neural")]
//     MalePollyGregory,
//     #[serde(rename = "Polly.Joey-Neural")]
//     MalePollyJoey,
//     #[serde(rename = "Polly.Matthew-Neural")]
//     MalePollyMatthew,
//     #[serde(rename = "Polly.Stephen-Neural")]
//     MalePollyStephen,
//     #[serde(rename = "Google.en-US-Neural2-C")]
//     FemaleGoogleNeural2C,
//     #[serde(rename = "Google.en-US-Neural2-E")]
//     FemaleGoogleNeural2E,
//     #[serde(rename = "Google.en-US-Neural2-F")]
//     FemaleGoogleNeural2F,
//     #[serde(rename = "Google.en-US-Neural2-G")]
//     FemaleGoogleNeural2G,
//     #[serde(rename = "Google.en-US-Neural2-H")]
//     FemaleGoogleNeural2H,
//     #[serde(rename = "Google.en-US-Wavenet-C")]
//     FemaleGoogleWavenetC,
//     #[serde(rename = "Google.en-US-Wavenet-E")]
//     FemaleGoogleWavenetE,
//     #[serde(rename = "Google.en-US-Wavenet-F")]
//     FemaleGoogleWavenetF,
//     #[serde(rename = "Google.en-US-Wavenet-G")]
//     FemaleGoogleWavenetG,
//     #[serde(rename = "Google.en-US-Wavenet-H")]
//     FemaleGoogleWavenetH,
//     #[serde(rename = "Polly.Danielle-Neural")]
//     FemalePollyDanielle,
//     #[serde(rename = "Polly.Joanna-Neural")]
//     FemalePollyJoanna,
//     #[serde(rename = "Polly.Kendra-Neural")]
//     FemalePollyKendra,
//     #[serde(rename = "Polly.Kimberly-Neural")]
//     FemalePollyKimberly,
//     #[serde(rename = "Polly.Ruth-Neural")]
//     FemalePollyRuth,
//     #[serde(rename = "Polly.Salli-Neural")]
//     FemalePollySalli,
// }
