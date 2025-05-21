use serde::{Deserialize, Deserializer, Serialize, de};
use typed_builder::TypedBuilder;

use super::{Language, Method, Pause, Play, Say, VoicePrice};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    strum::AsRefStr,
    strum::Display,
    Serialize,
    Deserialize,
)]
pub enum GatherDigit {
    #[strum(serialize = "0")]
    #[serde(rename = "0")]
    Zero,
    #[strum(serialize = "1")]
    #[serde(rename = "1")]
    One,
    #[strum(serialize = "2")]
    #[serde(rename = "2")]
    Two,
    #[strum(serialize = "3")]
    #[serde(rename = "3")]
    Three,
    #[strum(serialize = "4")]
    #[serde(rename = "4")]
    Four,
    #[strum(serialize = "5")]
    #[serde(rename = "5")]
    Five,
    #[strum(serialize = "6")]
    #[serde(rename = "6")]
    Six,
    #[strum(serialize = "7")]
    #[serde(rename = "7")]
    Seven,
    #[strum(serialize = "8")]
    #[serde(rename = "8")]
    Eight,
    #[strum(serialize = "9")]
    #[serde(rename = "9")]
    Nine,
    #[strum(serialize = "*")]
    #[serde(rename = "*")]
    Star,
    #[default]
    #[strum(serialize = "#")]
    #[serde(rename = "#")]
    Pound,
    #[strum(serialize = "")]
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Clone, Copy, strum::Display, strum::IntoStaticStr, PartialEq, Eq)]
#[strum(prefix = "$", serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SpeechHint {
    OovClassAlphanumericSequence,
    OovClassAlphaSequence,
    OovClassAmRadioFrequency,
    OovClassDigitSequence,
    OovClassFmRadioFrequency,
    OovClassFullphonenum,
    OovClassOrdinal,
    OovClassTemperature,
    OovClassTvChannel,
    Addressnum,
    Day,
    Fullphonenum,
    Money,
    Month,
    Operand,
    Percent,
    Postalcode,
    Street,
    Time,
    Year,
}

impl SpeechHint {
    pub fn as_str(&self) -> &str {
        self.into()
    }
}

/// TwiML Voice: <Gather>
/// https://www.twilio.com/docs/voice/twiml/gather
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gather {
    /// The `action` attribute specifies one URL that Twilio sends your request using HTTP.
    ///
    /// Necessity: Recommended
    ///
    /// Accepted values: Relative or absolute URL
    ///
    /// Default value: current document URL
    ///
    /// After the caller finishes entering digits or reaches the timeout, Twilio sends a POST HTTP request to the specified URL. If you omit this attribute, Twilio calls the TwiML document making the request. This might lead to unwanted looping behavior.
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#action
    #[serde(rename = "@action")]
    #[builder(default)]
    pub action: String,

    /// The `actionOnEmptyResult` attribute specifies that `<Gather>` must send a webhook to the action URL with or without DTMF input. By default, if `<Gather>` times out while waiting for DTMF input, it continues to the next TwiML instruction.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: `true`, `false`
    ///
    /// Default value: `false`
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#actiononemptyresult
    #[serde(rename = "@actionOnEmptyResult")]
    #[builder(default)]
    pub action_on_empty_result: bool,

    /// The `finishOnKey` attribute specifies the value that your caller presses to submit their digits.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: `#`, `*`, single digits `0`-`9`, an empty string (`''`)
    ///
    /// Default value: `#`, the hash or pound sign
    ///
    /// If you set this attribute to an empty string, `<Gather>` captures all caller input. After the call reaches its timeout, Twilio submits the caller's digits to the action URL.
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#finishonkey
    #[serde(rename = "@finishOnKey")]
    #[builder(default, setter(strip_option))]
    pub finish_on_key: Option<GatherDigit>,

    /// The `hints` attribute specifies a list of words or phrases that Twilio should expect during recognition. Adding `hints` to your `<Gather>` improves Twilio's recognition.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: Comma-separated list of up to 500 entries
    ///
    /// Default value: None
    ///
    /// Entries contain either single words or phrases. Each entry can up to 100 characters in length. Separate each word in a phrase with a space.
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#hints
    #[serde(rename = "@hints")]
    #[builder(default, setter(strip_option))]
    pub hints: Option<String>,

    /// The `input` attribute specifies which types of input Twilio accepts. The types include dual-tone multi-frequency ([DTMF](https://www.twilio.com/docs/glossary/what-is-dtmf "DTMF")), speech, or both.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: `dtmf`, `speech`, `dtmf speech`
    ///
    /// Default value: `dtmf`
    ///
    /// #### Considerations
    /// - When this attribute value includes `speech`, Twilio gathers speech from the caller for a _maximum_ duration of 60 seconds. `<Gather>` doesn't recognize speaking individual alphanumeric characters like "ABC123".
    /// - When you set this attribute value to `dtmf speech`, Twilio gives precedence to the first input it detects. If Twilio detects `speech` first, it ignores the `finishOnKey` attribute.
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#input
    #[serde(rename = "@input")]
    #[builder(default)]
    pub input: GatherInput,

    /// The `language` attribute specifies the language Twilio should recognize from your caller.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: Any value the STT provider supports
    ///
    /// Default value: `en-US`
    ///
    /// #### Considerations
    /// - This attribute passes through unchanged to the provider.
    /// - This attribute value maps to specific languages in the related [speechModel](https://www.twilio.com/docs/voice/twiml/gather#speechmodel "speechModel").
    ///   - Google STT V2 models map to the [languages specified in its Speech-to-Text V2 supported languages guide(link takes you to an external page)](https://cloud.google.com/speech-to-text/v2/docs/speech-to-text-supported-languages "languages specified in its Speech-to-Text V2 supported languages guide")
    ///   - Deepgram models map to the [languages specified in its Models & Languages Overview guide(link takes you to an external page)](https://developers.deepgram.com/docs/models-languages-overview "languages specified in its Models & Languages Overview guide")
    ///   - Google STT V1 models map to [any language Twilio supports](https://www.twilio.com/docs/voice/twiml/gather#languagetags "any language Twilio supports"). _Twilio has deprecated Google STT V1 as a language model._
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#language
    #[serde(rename = "@language")]
    #[builder(default = Language::EnUs)]
    pub language: Language,

    /// The `method` attribute specifies the HTTP verb Twilio should use to request your action URL.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: `GET`, `POST`
    ///
    /// Default value: `POST`
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#method
    #[serde(rename = "@method")]
    #[builder(default)]
    pub method: Method,

    /// The `numDigits` attribute specifies how many digits you require from callers for this `<Gather>` instance for [DTMF](https://www.twilio.com/docs/glossary/what-is-dtmf "DTMF") input.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: any positive integer
    ///
    /// Default value: None
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#numdigits
    #[serde(rename = "@numDigits")]
    #[builder(default, setter(strip_option))]
    pub num_digits: Option<u8>,

    /// The `partialResultCallback` attribute specifies a URL to which Twilio sends requests as it recognizes speech _in real time_. These requests contain a parameter labeled **`UnstableSpeechResult`** which contains partial transcriptions. These transcriptions may change as the speech recognition progresses.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: comma-separated list of up to 500 entries
    ///
    /// Default value: None
    ///
    /// Twilio makes asynchronous [webhooks](https://www.twilio.com/docs/glossary/what-is-a-webhook "webhooks") to your `partialResultCallback` URL. They don't accept any TwiML documents in response. To take more actions based on this partial result, [use the REST API to modify the call](https://www.twilio.com/docs/voice/api/call-resource#update-a-call-resource "use the REST API to modify the call").
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#partialresultcallback
    #[serde(rename = "@partialResultCallback")]
    #[builder(default, setter(strip_option))]
    pub partial_result_callback: Option<String>,

    /// The `partialResultCallbackMethod` attribute specifies the HTTP verb for partialResultCallback requests.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: `GET`, `POST`
    ///
    /// Default value: `POST`
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#partialresultcallbackmethod
    #[serde(rename = "@partialResultCallbackMethod")]
    #[builder(default, setter(strip_option))]
    pub partial_result_callback_method: Option<Method>,

    /// The `profanityFilter` attribute specifies whether Twilio should filter profanities from transcription.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: `true`, `false`
    ///
    /// Default value: `true`
    ///
    /// When set to `true`, Twilio replaces all but the initial character in each filtered profane word with asterisks like `f***`.
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#profanityfilter
    #[serde(rename = "@profanityFilter")]
    #[builder(default, setter(strip_option))]
    pub profanity_filter: Option<bool>,

    /// The `speechModel` attribute specifies which language model to apply to your `<Gather>` request.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: `default`, `numbers_and_commands`, `phone_call`, `experimental_conversations`, `experimental_utterances`, `googlev2_long`, `googlev2_short`, `googlev2_telephony`, `googlev2_telephony_short`, `deepgram_nova-2`
    ///
    /// Default value: `default`
    ///
    /// #### Considerations
    /// - This attribute requires you to set [`speechTimeout`](https://www.twilio.com/docs/voice/twiml/gather#speechtimeout) to a positive integer value. Don't use `auto`.
    /// - If Twilio selects your language model, it can handle failover to another provider. In practice, if Google experiences a failure, Twilio switches STT to Deepgram without any action on your part.
    /// - If you want to select the model, you can choose either a [generic model](https://www.twilio.com/docs/voice/twiml/gather#generic-speech-to-text-models "generic model") or a [specific STT model](https://www.twilio.com/docs/voice/twiml/gather#specific-speech-to-text-models "specific STT model").
    ///
    /// #### Generic speech-to-text models
    /// Generic models include the following values (see documentation for supported languages):
    /// `default`, `phone_call`, `numbers_and_commands`, `experimental_conversations`, `experimental_utterances`
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#speechmodel
    #[serde(rename = "@speechModel")]
    #[builder(default, setter(strip_option))]
    pub speech_model: Option<SpeechModel>,

    /// The `speechTimeout` attribute specifies how long to wait after speech pauses before stopping recognition.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: Positive integer or `auto`
    ///
    /// Default value: `timeout` value
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#speechtimeout
    #[serde(rename = "@speechTimeout")]
    #[builder(default, setter(strip_option))]
    pub speech_timeout: Option<SpeechTimeout>,

    /// The `timeout` attribute specifies how long Twilio should wait for caller input.
    ///
    /// Necessity: Optional
    ///
    /// Accepted values: Positive integer
    ///
    /// Default value: `5`
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#timeout
    #[serde(rename = "@timeout")]
    #[builder(default, setter(strip_option))]
    pub timeout: Option<u8>,

    /// #### Nest other verbs
    ///
    /// You can nest the following verbs within `<Gather>`:
    /// - [`<Pause>`](https://www.twilio.com/docs/voice/twiml/pause)
    /// - [`<Play>`](https://www.twilio.com/docs/voice/twiml/play)
    /// - [`<Say>`](https://www.twilio.com/docs/voice/twiml/say)
    ///
    /// When a `<Gather>` contains nested `<Say>` or `<Play>` verbs, the `timeout` begins either after the audio completes or when the caller presses their first key. If `<Gather>` contains multiple `<Play>` verbs, Twilio retrieves the contents of all files before the `<Play>` begins.
    ///
    /// Documentation: https://www.twilio.com/docs/voice/twiml/gather#nest-other-verbs
    #[builder(default)]
    #[serde(rename = "#content")]
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
pub enum GatherInput {
    #[default]
    #[serde(rename = "dtmf")]
    Dtmf,
    #[serde(rename = "speech")]
    Speech,
    #[serde(rename = "dtmf speech")]
    DtmfSpeech,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechTimeout {
    #[serde(rename = "auto")]
    Auto,
    #[serde(untagged)]
    Seconds(#[serde(deserialize_with = "deserialize_seconds")] u8),
}

impl From<u8> for SpeechTimeout {
    fn from(seconds: u8) -> Self {
        Self::Seconds(seconds)
    }
}

impl From<u16> for SpeechTimeout {
    fn from(seconds: u16) -> Self {
        Self::from(seconds as usize)
    }
}

impl From<u32> for SpeechTimeout {
    fn from(seconds: u32) -> Self {
        Self::from(seconds as usize)
    }
}

impl From<u64> for SpeechTimeout {
    fn from(seconds: u64) -> Self {
        Self::from(seconds as usize)
    }
}

impl From<usize> for SpeechTimeout {
    fn from(seconds: usize) -> Self {
        Self::Seconds((seconds % u8::MAX as usize) as u8)
    }
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

pub trait GatherBuilderVerbs {
    type Output;

    /// Convenience method for [`GatherBuilder::verbs`] but supports multiple calls.
    fn pause(self, pause: Pause) -> Self::Output;

    /// Convenience method for [`GatherBuilder::verbs`] but supports multiple calls.
    fn play(self, play: Play) -> Self::Output;

    /// Convenience method for [`GatherBuilder::verbs`] but supports multiple calls.
    fn say(self, say: Say) -> Self::Output;
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> GatherBuilderVerbs
    for GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, ())>
{
    type Output = GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, (Vec<GatherVerb>,))>;

    fn pause(self, pause: Pause) -> Self::Output {
        self.verbs(vec![GatherVerb::Pause(pause)])
    }

    fn play(self, play: Play) -> Self::Output {
        self.verbs(vec![GatherVerb::Play(play)])
    }

    fn say(self, say: Say) -> Self::Output {
        self.verbs(vec![GatherVerb::Say(say)])
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> GatherBuilderVerbs
    for GatherBuilder<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, (Vec<GatherVerb>,))>
{
    type Output = Self;

    fn pause(mut self, pause: Pause) -> Self::Output {
        self.fields.14.0.push(GatherVerb::Pause(pause));
        self
    }

    fn play(mut self, play: Play) -> Self::Output {
        self.fields.14.0.push(GatherVerb::Play(play));
        self
    }

    fn say(mut self, say: Say) -> Self::Output {
        self.fields.14.0.push(GatherVerb::Say(say));
        self
    }
}
