use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Gather, Say, TwilioMethod, VoicePrice};

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

impl VoicePrice for Response {
    fn price(&self) -> f32 {
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
}

impl ResponseBuilder<((Vec<ResponseVerb>,),)> {
    pub fn say(self, say: Say) -> Self {
        let ((mut verbs,),) = self.fields;
        verbs.push(ResponseVerb::Say(say));

        ResponseBuilder {
            fields: ((verbs,),),
            phantom: self.phantom,
        }
    }

    pub fn gather(self, gather: Gather) -> Self {
        let ((mut verbs,),) = self.fields;
        verbs.push(ResponseVerb::Gather(gather));

        ResponseBuilder {
            fields: ((verbs,),),
            phantom: self.phantom,
        }
    }

    pub fn pause(self, pause: Pause) -> Self {
        let ((mut verbs,),) = self.fields;
        verbs.push(ResponseVerb::Pause(pause));

        ResponseBuilder {
            fields: ((verbs,),),
            phantom: self.phantom,
        }
    }
}

/// TwiML Voice: <Pause>
/// https://www.twilio.com/docs/voice/twiml/pause
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pause {
    #[serde(rename = "@length")]
    length: u8,
}

/// TwiML Voice: <Redirect>
/// https://www.twilio.com/docs/voice/twiml/redirect
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "@method")]
    method: TwilioMethod,
}
