mod responses;
pub use responses::*;

mod gather;
pub use gather::*;

mod say;
pub use say::*;

mod voices;
pub use voices::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TwilioLanguage {
    #[default]
    #[serde(rename = "en-US")]
    EnUs,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TwilioMethod {
    GET,
    #[default]
    POST,
}

// TwiML Voice: Voices List:
// https://www.twilio.com/docs/voice/twiml/say/text-speech#available-voices-and-languages

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeypadDigit {
    #[serde(rename = "0")]
    Zero,
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "6")]
    Six,
    #[serde(rename = "7")]
    Seven,
    #[serde(rename = "8")]
    Eight,
    #[serde(rename = "9")]
    Nine,
    #[serde(rename = "*")]
    Star,
    #[serde(rename = "#")]
    Pound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say() {
        let resp = Response::builder()
            .say(
                Say::builder()
                    .text("Hello".to_string())
                    .voice(voices::en_us::male::generative::google::Chirp3HdCharon.into())
                    .build(),
            )
            .build();
        let xml = quick_xml::se::to_string(&resp).unwrap();
        assert_eq!(
            xml,
            r#"<Response><Say language="en-US" voice="Google.en-US-Chirp3-HD-Charon" loop="1">Hello</Say></Response>"#
        )
    }

    #[test]
    fn test_deserialize_say() {
        let text = "Excepteur et labore in excepteur enim nisi tempor. Commodo ex eiusmod incididunt occaecat commodo dolor consequat. Consectetur laboris velit dolore tempor Lorem adipisicing anim occaecat.";
        let xml = format!(
            r#"<Response><Say language="en-US" voice="Google.en-US-Chirp3-HD-Charon" loop="1">{text}</Say></Response>"#
        );
        let response: Response = quick_xml::de::from_str(&xml).unwrap();
        // println!("RESPONSE:\n{response:#?}");
        // println!("PRICE: {}", response.price());
        assert_eq!(response.price(), 0.013);
        let say: Say = match &response.verbs[0] {
            ResponseVerb::Say(say) => say.clone(),
            _ => unreachable!(),
        };
        assert_eq!(say.text, text);
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
        println!("\n\n\n{xml}\n\n\n");
        assert_eq!(
            xml,
            r#"<Response><Gather action="" actionOnEmptyResult="false" input="dtmf" language="en-US" method="POST" numDigits="1"><Say language="en-US" voice="Google.en-US-Neural2-C" loop="1">Press 1 for sales, 2 for support.</Say></Gather></Response>"#
        )
    }
}
