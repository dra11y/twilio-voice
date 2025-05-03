use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Gather, Method, Say, VoicePrice};

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

    pub fn hangup(self) -> Self {
        self.add_verb(ResponseVerb::Hangup)
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
    method: Method,
}
