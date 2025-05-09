use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Gather, Redirect, Say, VoicePrice};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseVerb {
    Say(Say),
    Gather(Gather),
    Pause(Pause),
    Redirect(Redirect),
    Hangup,
}

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "$value")]
    pub verbs: Vec<ResponseVerb>,
}

impl Response {
    pub fn to_xml(&self) -> String {
        quick_xml::se::to_string(self).unwrap()
    }

    pub fn to_xml_pretty(&self) -> String {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);
        writer.write_serializable("Response", self).unwrap();
        String::from_utf8(buffer).unwrap()
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
        (
            [(axum::http::header::CONTENT_TYPE, "text/xml")],
            self.to_xml(),
        )
            .into_response()
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
    use crate::twiml::{
        GatherBuilderVerbs, GatherDigit, GatherVerb, InputType, Language, SpeechModel,
        SpeechTimeout,
        voices::{self, GENERATIVE_VOICE_PRICE, NEURAL_VOICE_PRICE, STANDARD_VOICE_PRICE},
    };

    use super::*;

    #[test]
    fn test_say() {
        let resp = Response::builder()
            .say(
                Say::builder()
                    .text("Hello".to_string())
                    .voice(voices::Voice::Woman)
                    .build(),
            )
            .build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert_eq!(
            xml,
            r#"<Response><Say voice="Woman" loop="1">Hello</Say></Response>"#
        );

        let resp = Response::builder()
            .say(
                Say::builder()
                    .text("Hello".to_string())
                    .voice(voices::en_us::generative::google::Male::Chirp3HdCharon.into())
                    .build(),
            )
            .build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert_eq!(
            xml,
            r#"<Response><Say voice="Google.en-US-Chirp3-HD-Charon" loop="1">Hello</Say></Response>"#
        );

        let resp = Response::builder()
            .say(
                Say::builder()
                    .text("Hello".to_string())
                    .language(Language::EnUs)
                    .voice(voices::en_us::generative::google::Male::Chirp3HdCharon.into())
                    .build(),
            )
            .build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert_eq!(
            xml,
            r#"<Response><Say language="en-US" voice="Google.en-US-Chirp3-HD-Charon" loop="1">Hello</Say></Response>"#
        );
    }

    #[test]
    fn test_deserialize_say() {
        let text = "Excepteur et labore in excepteur enim nisi tempor. Commodo ex eiusmod incididunt occaecat commodo dolor consequat. Consectetur laboris velit dolore tempor Lorem adipisicing anim occaecat.";
        let xml = format!(
            r#"<Response><Say language="en-US" voice="Google.en-US-Neural2-A" loop="1">{text}</Say></Response>"#
        );
        let response: Response = quick_xml::de::from_str(&xml).unwrap();
        let blocks = text.len() / 100;
        let expected_price = NEURAL_VOICE_PRICE * blocks as f32;
        assert_eq!(response.price(), Some(expected_price));
        let ResponseVerb::Say(say) = &response.verbs[0] else {
            panic!("Expected Say verb");
        };
        assert_eq!(say.text, text);
        assert_eq!(
            say.voice,
            Some(voices::en_us::neural::google::Male::Neural2A.into())
        );

        let xml = format!(
            r#"<Response><Say language="en-US" voice="Woman" loop="1">{text}</Say></Response>"#
        );
        let response: Response = quick_xml::de::from_str(&xml).unwrap();
        let ResponseVerb::Say(say) = &response.verbs[0] else {
            panic!("Expected Say verb");
        };
        assert_eq!(say.voice, Some(voices::Voice::Woman));
    }

    #[test]
    fn test_gather() {
        let say = Say::builder()
            .text("Press 1 for sales, 2 for support.".to_string())
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
        let xml = quick_xml::se::to_string(&resp).unwrap();
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
            let response: Response = quick_xml::de::from_str(&xml).unwrap();
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
            assert_eq!(say.text, text);
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
                    .text(welcome_text.to_string())
                    .voice(voices::en_us::standard::polly::Female::Joanna.into())
                    .build(),
            )
            .pause(Pause::builder().length(3).build())
            .say(
                Say::builder()
                    .text(selection_text.to_string())
                    .language(Language::EnUs)
                    .voice(voices::en_us::standard::polly::Female::Joanna.into())
                    .build(),
            )
            .build();

        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert_eq!(
            xml,
            r#"<Response><Say voice="Polly.Joanna" loop="1">Welcome to our service.</Say><Pause length="3"/><Say language="en-US" voice="Polly.Joanna" loop="1">Please make a selection.</Say></Response>"#
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
                    .text("Please enter your account number.".to_string())
                    .voice(voices::en_us::standard::polly::Male::Matthew.into())
                    .build(),
            )
            .build();
        let resp = Response::builder().gather(gather).build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
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
                    .text(account_text.to_string())
                    .voice(voices::en_us::neural::google::Male::Neural2D.into())
                    .build(),
            )
            .say(
                Say::builder()
                    .text(pound_text.to_string())
                    .voice(voices::en_us::neural::google::Male::Neural2D.into())
                    .build(),
            )
            .build();

        let resp = Response::builder().gather(gather).build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
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
                    .text("This message will repeat three times.".to_string())
                    .voice(voices::en_us::neural::polly::Female::RuthNeural.into())
                    .loop_count(3)
                    .build(),
            )
            .build();

        let xml = quick_xml::se::to_string(&resp).unwrap();
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
                    .text(speech_text.to_string())
                    .voice(voices::en_us::generative::google::Female::Chirp3HdAoede.into())
                    .build(),
            )
            .build();

        let resp = Response::builder().gather(gather).build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert!(xml.contains(r#"input="speech""#));
        assert!(xml.contains(r#"speechModel="phone_call""#));

        // Calculate correct pricing for generative voice
        let speech_blocks = (speech_text.len() / 100) as f32;
        let expected_price = GENERATIVE_VOICE_PRICE * speech_blocks;
        assert_eq!(resp.price(), Some(expected_price));
    }

    #[test]
    fn test_hangup() {
        let resp = Response::builder()
            .say(
                Say::builder()
                    .text("Thank you for calling. Goodbye!".to_string())
                    .voice(voices::en_us::neural::polly::Female::KendraNeural.into())
                    .build(),
            )
            .hangup()
            .build();

        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert!(xml.contains("Thank you for calling"));
        assert!(xml.contains("<Hangup/>"));
    }

    #[test]
    fn test_gather_with_finish_on_key() {
        let gather = Gather::builder()
            .action("/handle_input".to_string())
            .finish_on_key(GatherDigit::Pound)
            .say(
                Say::builder()
                    .text("Enter your pin followed by the pound key.".to_string())
                    .voice(voices::en_us::neural::polly::Male::JoeyNeural.into())
                    .build(),
            )
            .build();

        let resp = Response::builder().gather(gather).build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert!(xml.contains(r##"finishOnKey="#""##));
    }

    #[test]
    fn test_complex_ivr_flow() {
        let main_menu = Gather::builder()
            .action("/process_menu".to_string())
            .num_digits(1)
            .say(
                Say::builder()
                    .text("Welcome to Acme Corporation. Press 1 for sales, 2 for support, or 3 for billing.".to_string())
                    .voice(voices::en_us::neural::google::Female::Neural2F.into())
                    .build(),
            )
            .build();

        let fallback = Say::builder()
            .text("We didn't receive any input. Please call back later.".to_string())
            .voice(voices::en_us::neural::google::Female::Neural2F.into())
            .build();

        let resp = Response::builder().gather(main_menu).say(fallback).build();

        let xml = quick_xml::se::to_string(&resp).unwrap();
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
