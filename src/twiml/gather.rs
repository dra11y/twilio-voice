use serde::{Deserialize, Deserializer, Serialize, de};
use typed_builder::TypedBuilder;

use super::{Digit, Language, Method, Pause, Play, Say, VoicePrice};

/// TwiML Voice: <Gather>
/// https://www.twilio.com/docs/voice/twiml/gather
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gather {
    #[serde(rename = "@action")]
    #[builder(default)]
    pub action: String,
    #[serde(rename = "@actionOnEmptyResult")]
    #[builder(default)]
    pub action_on_empty_result: bool,
    #[serde(rename = "@finishOnKey")]
    #[builder(default, setter(strip_option))]
    pub finish_on_key: Option<Digit>,
    #[serde(rename = "@hints")]
    #[builder(default, setter(strip_option))]
    pub hints: Option<String>,
    #[serde(rename = "@input")]
    #[builder(default)]
    pub input: InputType,
    #[serde(rename = "@language")]
    #[builder(default = Language::EnUs)]
    pub language: Language,
    #[serde(rename = "@method")]
    #[builder(default)]
    pub method: Method,
    #[serde(rename = "@numDigits")]
    #[builder(default, setter(strip_option))]
    pub num_digits: Option<u8>,
    #[serde(rename = "@partialResultCallback")]
    #[builder(default, setter(strip_option))]
    pub partial_result_callback: Option<String>,
    #[serde(rename = "@partialResultCallbackMethod")]
    #[builder(default, setter(strip_option))]
    pub partial_result_callback_method: Option<Method>,
    #[serde(rename = "@profanityFilter")]
    #[builder(default, setter(strip_option))]
    pub profanity_filter: Option<bool>,
    #[serde(rename = "@speechModel")]
    #[builder(default, setter(strip_option))]
    pub speech_model: Option<SpeechModel>,
    #[serde(rename = "@speechTimeout")]
    #[builder(default, setter(strip_option))]
    pub speech_timeout: Option<Timeout>,
    #[serde(rename = "@timeout")]
    #[builder(default, setter(strip_option))]
    pub timeout: Option<Timeout>,

    #[builder(default)]
    #[serde(default, rename = "$value")]
    pub verbs: Vec<GatherVerb>,
}

impl VoicePrice for Gather {
    fn price(&self) -> Option<f32> {
        let says = self
            .verbs
            .iter()
            .filter_map(|v| match v {
                GatherVerb::Say(say) => Some(say),
                _ => None,
            })
            .collect::<Vec<_>>();
        let prices = says.iter().filter_map(|s| s.price()).collect::<Vec<_>>();
        if prices.is_empty() || prices.len() != says.len() {
            None
        } else {
            Some(prices.iter().sum())
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    #[default]
    Dtmf,
    Speech,
    DtmfSpeech,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Timeout {
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    Seconds(#[serde(deserialize_with = "deserialize_seconds")] u8),
}

fn deserialize_seconds<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GatherVerb {
    Pause(Pause),
    Play(Play),
    Say(Say),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechModel {
    #[default]
    Default,
    NumbersAndCommands,
    PhoneCall,
    ExperimentalConversations,
    ExperimentalUtterances,
    Googlev2Long,
    Googlev2Short,
    Googlev2Telephony,
    Googlev2TelephonyShort,
    #[serde(rename = "deepgram_nova-2")]
    DeepgramNova2,
}

#[allow(clippy::type_complexity)]
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N>
    GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, ())>
{
    pub fn say(
        self,
        say: Say,
    ) -> GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, (Vec<GatherVerb>,))> {
        self.verbs(vec![GatherVerb::Say(say)])
    }

    pub fn pause(
        self,
        pause: Pause,
    ) -> GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, (Vec<GatherVerb>,))> {
        self.verbs(vec![GatherVerb::Pause(pause)])
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N>
    GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, (Vec<GatherVerb>,))>
{
    pub fn pause(mut self, pause: Pause) -> Self {
        self.fields.14.0.push(GatherVerb::Pause(pause));
        self
    }

    pub fn play(mut self, play: Play) -> Self {
        self.fields.14.0.push(GatherVerb::Play(play));
        self
    }

    pub fn say(mut self, say: Say) -> Self {
        self.fields.14.0.push(GatherVerb::Say(say));
        self
    }
}
