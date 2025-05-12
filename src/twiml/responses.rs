use std::{fmt::Display, str::FromStr};

use crate::errors::Error;

use super::{Gather, Play, Redirect, Say, VoicePrice};
use quick_xml::escape::{escape, unescape};
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use typed_builder::TypedBuilder;

static XML_DECL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<\?.+\?>").unwrap());

static SAY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<Say(.*?)>(.*?)</Say>").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseVerb {
    Say(Say),
    Gather(Gather),
    Pause(Pause),
    Play(Play),
    Redirect(Redirect),
    Hangup,
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "#content")]
    pub verbs: Vec<ResponseVerb>,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xml = serde_xml_rs::to_string(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{xml}")
    }
}

impl FromStr for Response {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let escaped = escape_say(s);
        serde_xml_rs::from_str(&escaped)
            .map_err(|e| Error::ResponseDeser(e.to_string(), s.to_string()))
    }
}

fn remove_xml_decl(xml: &str) -> String {
    XML_DECL_REGEX.replace(xml, "").to_string()
}

fn unescape_say(xml: &str) -> String {
    SAY_REGEX
        .replace_all(xml, |caps: &Captures| {
            let attrs = &caps[1];
            let content = unescape(&caps[2]).expect("Failed to unescape XML");
            format!("<Say{attrs}>{content}</Say>")
        })
        .to_string()
}

fn escape_say(xml: &str) -> String {
    SAY_REGEX
        .replace_all(xml, |caps: &Captures| {
            let attrs = &caps[1];
            let content = escape(&caps[2]);
            format!("<Say{attrs}>{content}</Say>")
        })
        .to_string()
}

impl Response {
    pub fn to_xml(&self) -> String {
        let xml = self.to_string();
        remove_xml_decl(&unescape_say(&xml))
    }

    pub fn to_xml_pretty(&self) -> String {
        self.to_xml()
        // let mut buffer = Vec::new();
        // let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);
        // writer.write_serializable("Response", self).unwrap();
        // let xml = String::from_utf8(buffer).unwrap();
        // unescape_say(&xml)
    }
}

impl VoicePrice for Response {
    fn price(&self) -> Option<f32> {
        self.verbs
            .iter()
            .filter_map(|v| match v {
                ResponseVerb::Say(say) => Some(say.price()),
                ResponseVerb::Gather(gather) => Some(gather.price()),
                _ => None,
            })
            .sum()
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        #[cfg(debug_assertions)]
        let xml = self.to_xml_pretty();

        #[cfg(not(debug_assertions))]
        let xml = self.to_xml();

        ([(axum::http::header::CONTENT_TYPE, "text/xml")], xml).into_response()
    }
}

impl ResponseBuilder<((),)> {
    pub fn say(self, say: Say) -> ResponseBuilder<((Vec<ResponseVerb>,),)> {
        self.verbs(vec![ResponseVerb::Say(say)])
    }

    pub fn gather(self, gather: Gather) -> ResponseBuilder<((Vec<ResponseVerb>,),)> {
        self.verbs(vec![ResponseVerb::Gather(gather)])
    }

    pub fn pause(self, pause: Pause) -> ResponseBuilder<((Vec<ResponseVerb>,),)> {
        self.verbs(vec![ResponseVerb::Pause(pause)])
    }

    pub fn play(self, play: Play) -> ResponseBuilder<((Vec<ResponseVerb>,),)> {
        self.verbs(vec![ResponseVerb::Play(play)])
    }

    pub fn redirect(self, redirect: Redirect) -> ResponseBuilder<((Vec<ResponseVerb>,),)> {
        self.verbs(vec![ResponseVerb::Redirect(redirect)])
    }

    pub fn hangup(self) -> ResponseBuilder<((Vec<ResponseVerb>,),)> {
        self.verbs(vec![ResponseVerb::Hangup])
    }
}

impl ResponseBuilder<((Vec<ResponseVerb>,),)> {
    fn add_verb(self, verb: ResponseVerb) -> Self {
        let ((mut verbs,),) = self.fields;
        verbs.push(verb);

        ResponseBuilder {
            fields: ((verbs,),),
            phantom: self.phantom,
        }
    }

    pub fn say(self, say: Say) -> Self {
        self.add_verb(ResponseVerb::Say(say))
    }

    pub fn gather(self, gather: Gather) -> Self {
        self.add_verb(ResponseVerb::Gather(gather))
    }

    pub fn pause(self, pause: Pause) -> Self {
        self.add_verb(ResponseVerb::Pause(pause))
    }

    pub fn play(self, play: Play) -> Self {
        self.add_verb(ResponseVerb::Play(play))
    }

    pub fn redirect(self, redirect: Redirect) -> Self {
        self.add_verb(ResponseVerb::Redirect(redirect))
    }

    pub fn hangup(self) -> Self {
        self.add_verb(ResponseVerb::Hangup)
    }
}

/// TwiML Voice: <Pause>
/// https://www.twilio.com/docs/voice/twiml/pause
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pause {
    #[serde(rename = "@length")]
    pub length: u8,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::twiml::{
        self, GatherBuilderVerbs, GatherDigit, GatherVerb, InputType, Language, SpeechModel,
        SpeechTimeout, Voice,
        voices::{self, GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
    };

    use super::*;

    #[test]
    fn test_say() {
        let resp = Response::builder()
            .say(
                Say::builder()
                    .ssml("Hello, World! You’re #1!".into())
                    .voice(voices::Voice::Woman)
                    .build(),
            )
            .build();

        let xml = resp.to_xml();
        assert_eq!(
            xml,
            r#"<Response><Say voice="Woman" loop="1">Hello, World! You’re #1!</Say></Response>"#
        );

        println!("XML:\n{xml}");
        let deser: Response = xml.parse().unwrap();
        assert_eq!(deser, resp);

        let resp = Response::builder()
            .say(
                Say::builder()
                    .ssml("Hello".into())
                    .voice(voices::en_us::generative::google::Male::Chirp3HdCharon.into())
                    .build(),
            )
            .build();
        let xml = resp.to_xml();
        assert_eq!(
            xml,
            r#"<Response><Say voice="Google.en-US-Chirp3-HD-Charon" loop="1">Hello</Say></Response>"#
        );

        let deser: Response = xml.parse().unwrap();
        assert_eq!(deser, resp);

        let resp = Response::builder()
            .say(
                Say::builder()
                    .ssml("Hello".into())
                    .language(Language::EnUs)
                    .voice(voices::en_us::generative::google::Male::Chirp3HdCharon.into())
                    .build(),
            )
            .build();
        let xml = resp.to_xml();
        assert_eq!(
            xml,
            r#"<Response><Say language="en-US" voice="Google.en-US-Chirp3-HD-Charon" loop="1">Hello</Say></Response>"#
        );

        let deser: Response = xml.parse().unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_ssml() {
        let ssml = r#"Hello, World! <prosody volume="x-loud" pitch="+5%">You’re #1!</prosody>"#;
        let resp = Response::builder()
            .say(
                Say::builder()
                    .ssml(ssml.into())
                    .voice(voices::Voice::Woman)
                    .build(),
            )
            .build();

        let xml = resp.to_xml();
        assert_eq!(
            xml,
            format!(r#"<Response><Say voice="Woman" loop="1">{ssml}</Say></Response>"#)
        );

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_ssml_with_mixed_content() {
        let ssml = r#"<speak>Hello <emphasis>world</emphasis> &amp; goodbye!</speak>"#;
        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder().ssml(ssml.into()).voice(Voice::Woman).build(),
            )])
            .build();

        let xml = resp.to_xml();
        let expected = r#"<Response><Say voice="Woman" loop="1"><speak>Hello <emphasis>world</emphasis> &amp; goodbye!</speak></Say></Response>"#;
        assert_eq!(xml, expected);

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_ssml_with_cdata() {
        let ssml = r#"<![CDATA[<message>Hello & Welcome!</message>]]>"#;
        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder().ssml(ssml.into()).build(),
            )])
            .build();

        let xml = resp.to_xml();
        let expected = r#"<Response><Say loop="1"><![CDATA[<message>Hello & Welcome!</message>]]></Say></Response>"#;
        assert_eq!(xml, expected);

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_ssml_with_nested_tags() {
        let ssml = r#"
            <speak>
                <prosody rate="fast">Hello <emphasis>world</emphasis></prosody>
            </speak>
        "#
        .trim();
        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder()
                    .ssml(ssml.into())
                    .language(Language::EnUs)
                    .build(),
            )])
            .build();

        let xml = resp.to_xml();
        let expected =
            format!(r#"<Response><Say language="en-US" loop="1">{ssml}</Say></Response>"#);
        assert_eq!(xml, expected);

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_multiple_say_verbs() {
        let resp = Response::builder()
            .verbs(vec![
                ResponseVerb::Say(
                    Say::builder()
                        .ssml("First message".into())
                        .voice(Voice::Woman)
                        .build(),
                ),
                ResponseVerb::Say(
                    Say::builder()
                        .ssml("<speak>Second message</speak>".into())
                        .loop_count(3)
                        .build(),
                ),
            ])
            .build();

        let xml = resp.to_xml();
        let expected = r#"<Response><Say voice="Woman" loop="1">First message</Say><Say loop="3"><speak>Second message</speak></Say></Response>"#;
        assert_eq!(xml, expected);

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_special_characters_roundtrip() {
        let ssml = r#"1 < 2 &amp; 3 > 0"#;
        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder().ssml(ssml.into()).build(),
            )])
            .build();

        let xml = resp.to_xml();
        assert_eq!(
            xml,
            r#"<Response><Say loop="1">1 < 2 &amp; 3 > 0</Say></Response>"#
        );

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_complex_ssml_structure() {
        let ssml = r#"
            <speak version="1.0" xml:lang="en-US">
                <voice name="en-US-Wavenet-A">
                    <prosody rate="fast">Hello</prosody>
                    <break time="500ms" />
                    <emphasis level="strong">world</emphasis>!
                </voice>
            </speak>
        "#
        .trim()
        .replace("\n", "");

        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder().ssml(ssml.into()).build(),
            )])
            .build();

        let xml = resp.to_xml();
        assert!(xml.contains("<prosody rate=\"fast\">Hello</prosody>"));
        assert!(xml.contains("<break time=\"500ms\" />"));

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_whitespace_preservation() {
        let ssml = r#"
            <speak>
                Preserve   whitespace
                and newlines
            </speak>
        "#
        .trim();
        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder().ssml(ssml.into()).build(),
            )])
            .build();

        let xml = resp.to_xml();
        assert!(xml.contains("Preserve   whitespace"));
        assert!(xml.contains("and newlines"));

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_empty_ssml() {
        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder().ssml("".into()).build(),
            )])
            .build();

        let xml = resp.to_xml();
        assert_eq!(xml, r#"<Response><Say loop="1"></Say></Response>"#);

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);

        let resp = Response::builder()
            .verbs(vec![ResponseVerb::Say(
                Say::builder().ssml(" a".into()).build(),
            )])
            .build();

        let xml = resp.to_xml();
        assert_eq!(xml, r#"<Response><Say loop="1">a</Say></Response>"#);

        let deser = Response::from_str(&xml).unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_voice_strings() {
        let voice_id = "Polly.Joanna-Neural";
        let voice = twiml::Voice::from_str(voice_id).unwrap();
        assert_eq!(
            voice,
            voices::en_us::neural::polly::Female::JoannaNeural.into()
        );
        let voice_id = "Google.en-US-Neural2-D";
        let voice = twiml::Voice::from_str(voice_id).unwrap();
        assert_eq!(voice, voices::en_us::neural::google::Male::Neural2D.into());
        assert_eq!(voice.to_string(), voice_id);

        // test invalid id
        let voice_err = twiml::Voice::from_str("invalid-voice-id");
        assert!(voice_err.is_err());
    }

    #[test]
    fn test_deserialize_say() {
        let text = "Excepteur et labore in excepteur enim nisi tempor. Commodo ex eiusmod incididunt occaecat commodo dolor consequat. Consectetur laboris velit dolore tempor Lorem adipisicing anim occaecat.";
        let xml = format!(
            r#"<Response><Say language="en-US" voice="Google.en-US-Neural2-A" loop="1">{text}</Say></Response>"#
        );
        let response: Response = xml.parse().unwrap();
        let blocks = text.len() / 100;
        let expected_price = NEURAL_VOICE_PRICE * blocks as f32;
        assert_eq!(response.price(), Some(expected_price));
        let ResponseVerb::Say(say) = &response.verbs[0] else {
            panic!("Expected Say verb");
        };
        assert_eq!(say.text(), text);
        assert_eq!(
            say.voice,
            Some(voices::en_us::neural::google::Male::Neural2A.into())
        );

        let xml = format!(
            r#"<Response><Say language="en-US" voice="Woman" loop="1">{text}</Say></Response>"#
        );
        let response: Response = xml.parse().unwrap();
        let ResponseVerb::Say(say) = &response.verbs[0] else {
            panic!("Expected Say verb");
        };
        assert_eq!(say.voice, Some(voices::Voice::Woman));
    }

    #[test]
    fn test_gather() {
        let say = Say::builder()
            .ssml("Press 1 for sales, 2 for support.".into())
            .voice(voices::en_us::neural::google::Female::Neural2C.into())
            .build();
        let resp = Response::builder()
            .gather(
                Gather::builder()
                    .action("".to_string())
                    .num_digits(1)
                    .say(say)
                    .build(),
            )
            .build();
        let xml = resp.to_xml();
        assert_eq!(
            xml,
            r#"<Response><Gather action="" actionOnEmptyResult="false" input="dtmf" language="en-US" method="POST" numDigits="1"><Say voice="Google.en-US-Neural2-C" loop="1">Press 1 for sales, 2 for support.</Say></Gather></Response>"#
        )
    }

    #[test]
    fn test_deserialize_gather_with_speech_timeout() {
        let text = "Excepteur et labore in excepteur enim nisi tempor. Commodo ex eiusmod incididunt occaecat commodo dolor consequat. Consectetur laboris velit dolore tempor Lorem adipisicing anim occaecat.";
        for speech_timeout in [
            (r#"speechTimeout="auto""#, Some(SpeechTimeout::Auto)),
            (r#"speechTimeout="30""#, Some(SpeechTimeout::Seconds(30))),
            ("", None),
        ] {
            let xml = format!(
                r#"
                    <Response>
                        <Gather action="" actionOnEmptyResult="false" input="dtmf" language="en-US" method="POST" numDigits="10" {speech_timeout}>
                            <Say voice="Google.en-US-Wavenet-C" loop="1">{text}</Say>
                        </Gather>
                    </Response>
                "#,
                speech_timeout = speech_timeout.0,
            );
            let response: Response = xml.parse().unwrap();
            let blocks = text.len() / 100;
            let expected_price = NEURAL_VOICE_PRICE * blocks as f32;
            assert_eq!(response.price(), Some(expected_price));
            let ResponseVerb::Gather(gather) = &response.verbs[0] else {
                panic!("Expected Gather verb");
            };
            assert_eq!(gather.speech_timeout, speech_timeout.1);
            let GatherVerb::Say(say) = &gather.verbs[0] else {
                panic!("Expected Say verb inside Gather");
            };
            assert_eq!(say.text(), text);
            assert_eq!(
                say.voice,
                Some(voices::en_us::neural::google::Female::WavenetC.into())
            );
        }
    }

    #[test]
    fn test_multiple_verbs() {
        let welcome_text = "Welcome to our service.";
        let selection_text = "Please make a selection.";

        let resp = Response::builder()
            .say(
                Say::builder()
                    .ssml(welcome_text.into())
                    .voice(voices::en_us::standard::polly::Female::Joanna.into())
                    .build(),
            )
            .pause(Pause::builder().length(3).build())
            .say(
                Say::builder()
                    .ssml(selection_text.into())
                    .language(Language::EnUs)
                    .voice(voices::en_us::standard::polly::Female::Joanna.into())
                    .build(),
            )
            .build();

        let xml = resp.to_xml();
        assert_eq!(
            xml,
            r#"<Response><Say voice="Polly.Joanna" loop="1">Welcome to our service.</Say><Pause length="3" /><Say language="en-US" voice="Polly.Joanna" loop="1">Please make a selection.</Say></Response>"#
        );

        // Calculate correct pricing based on text length
        let welcome_blocks = (welcome_text.len() / 100) as f32;
        let selection_blocks = (selection_text.len() / 100) as f32;
        let expected_price = STANDARD_VOICE_PRICE * (welcome_blocks + selection_blocks);
        assert_eq!(resp.price(), Some(expected_price));
    }

    #[test]
    fn test_gather_with_auto_timeout() {
        let gather = Gather::builder()
            .action("/process_input".to_string())
            .num_digits(1)
            .timeout(15)
            .say(
                Say::builder()
                    .ssml("Please enter your account number.".into())
                    .voice(voices::en_us::standard::polly::Male::Matthew.into())
                    .build(),
            )
            .build();
        let resp = Response::builder().gather(gather).build();
        let xml = resp.to_xml();
        assert!(xml.contains("Please enter your account number"));
        assert!(xml.contains(r#"timeout="15""#));
    }

    #[test]
    fn test_gather_with_multiple_verbs() {
        let account_text = "Please enter your account number.";
        let pound_text = "Followed by the pound sign.";

        let gather = Gather::builder()
            .action("/process_input".to_string())
            .num_digits(2)
            .timeout(20)
            .say(
                Say::builder()
                    .ssml(account_text.into())
                    .voice(voices::en_us::neural::google::Male::Neural2D.into())
                    .build(),
            )
            .say(
                Say::builder()
                    .ssml(pound_text.into())
                    .voice(voices::en_us::neural::google::Male::Neural2D.into())
                    .build(),
            )
            .build();

        let resp = Response::builder().gather(gather).build();
        let xml = resp.to_xml();
        assert!(xml.contains("Please enter your account number"));
        assert!(xml.contains("Followed by the pound sign"));
        assert!(xml.contains(r#"timeout="20""#));
        assert!(xml.contains(r#"numDigits="2""#));

        // Calculate correct pricing based on text length
        let account_blocks = (account_text.len() / 100) as f32;
        let pound_blocks = (pound_text.len() / 100) as f32;
        let expected_price = NEURAL_VOICE_PRICE * (account_blocks + pound_blocks);
        assert_eq!(resp.price(), Some(expected_price));
    }

    #[test]
    fn test_say_with_loop_count() {
        let resp = Response::builder()
            .say(
                Say::builder()
                    .ssml("This message will repeat three times.".into())
                    .voice(voices::en_us::neural::polly::Female::RuthNeural.into())
                    .loop_count(3)
                    .build(),
            )
            .build();

        let xml = resp.to_xml();
        assert_eq!(
            xml,
            r#"<Response><Say voice="Polly.Ruth-Neural" loop="3">This message will repeat three times.</Say></Response>"#
        );
    }

    #[test]
    fn test_gather_with_speech_input() {
        let speech_text = "Please state your destination city.";

        let gather = Gather::builder()
            .action("/process_speech".to_string())
            .input(InputType::Speech)
            .speech_model(SpeechModel::PhoneCall)
            .language(Language::EnUs)
            .say(
                Say::builder()
                    .ssml(speech_text.into())
                    .voice(voices::en_us::generative::google::Female::Chirp3HdAoede.into())
                    .build(),
            )
            .build();

        let resp = Response::builder().gather(gather).build();
        let xml = resp.to_xml();
        assert!(xml.contains(r#"input="speech""#));
        assert!(xml.contains(r#"speechModel="phone_call""#));

        // Calculate correct pricing for generative voice
        let speech_blocks = (speech_text.len() / 100) as f32;
        let expected_price = GENERATIVE_VOICE_PRICE * speech_blocks;
        assert_eq!(resp.price(), Some(expected_price));
    }

    #[test]
    fn test_hangup() {
        let resp = Response::builder().hangup().build();

        let xml = resp.to_xml();
        println!("hangup xml: {xml}");
        assert!(xml.contains("<Hangup />"));

        let deser: Response = xml.parse().unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_play() {
        let url = "https://example.com/hello.mp3";
        let resp = Response::builder().play(url.into()).build();

        let xml = resp.to_xml();
        println!("play xml: {xml}");
        assert_eq!(
            xml,
            format!(r#"<Response><Play loop="1">{url}</Play></Response>"#)
        );

        let deser: Response = xml.parse().unwrap();
        assert_eq!(deser, resp);
    }

    #[test]
    fn test_gather_with_finish_on_key() {
        let gather = Gather::builder()
            .action("/handle_input".to_string())
            .finish_on_key(GatherDigit::Pound)
            .say(
                Say::builder()
                    .ssml("Enter your pin followed by the pound key.".into())
                    .voice(voices::en_us::neural::polly::Male::JoeyNeural.into())
                    .build(),
            )
            .build();

        let resp = Response::builder().gather(gather).build();
        let xml = resp.to_xml();
        assert!(xml.contains(r##"finishOnKey="#""##));
    }

    #[test]
    fn test_complex_ivr_flow() {
        let main_menu = Gather::builder()
            .action("/process_menu".to_string())
            .num_digits(1)
            .say(
                Say::builder()
                    .ssml("Welcome to Acme Corporation. Press 1 for sales, 2 for support, or 3 for billing.".into())
                    .voice(voices::en_us::neural::google::Female::Neural2F.into())
                    .build(),
            )
            .build();

        let fallback = Say::builder()
            .ssml("We didn't receive any input. Please call back later.".into())
            .voice(voices::en_us::neural::google::Female::Neural2F.into())
            .build();

        let resp = Response::builder().gather(main_menu).say(fallback).build();

        let xml = resp.to_xml();
        assert!(xml.contains("Welcome to Acme Corporation"));
        assert!(xml.contains("We didn't receive any input"));

        // Calculate expected pricing
        let menu_text_blocks =
            "Welcome to Acme Corporation. Press 1 for sales, 2 for support, or 3 for billing."
                .len()
                / 100;
        let fallback_text_blocks =
            "We didn't receive any input. Please call back later.".len() / 100;
        let expected_price = NEURAL_VOICE_PRICE * (menu_text_blocks + fallback_text_blocks) as f32;
        assert_eq!(resp.price(), Some(expected_price));
    }
}
