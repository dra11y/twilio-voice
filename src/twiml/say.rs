use std::{
    convert::Infallible,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{Language, Voice, VoicePrice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ssml(Vec<Tag>);

impl Default for Ssml {
    fn default() -> Self {
        Ssml(vec![Tag::Text("".to_string())])
    }
}

impl Serialize for Ssml {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[allow(clippy::unnecessary_filter_map)]
        let text = self
            .0
            .iter()
            .filter_map(|tag| match tag {
                Tag::Text(s) => Some(s.as_str()),
            })
            .collect::<String>();
        serializer.serialize_str(&text)
    }
}

impl<'de> Deserialize<'de> for Ssml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Capture the entire text content as a single string
        let text = String::deserialize(deserializer)?;
        Ok(Ssml(vec![Tag::Text(text.trim().to_string())]))
    }
}

impl From<String> for Ssml {
    fn from(value: String) -> Self {
        Ssml::from(value.trim())
    }
}

impl From<&str> for Ssml {
    fn from(value: &str) -> Self {
        Ssml::from_str(value.trim()).unwrap_or_default()
    }
}

impl FromStr for Ssml {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ssml(vec![Tag::Text(s.trim().to_string())]))
    }
}

impl Ssml {
    fn text(&self) -> String {
        #[allow(clippy::unnecessary_filter_map)]
        self.0
            .iter()
            .filter_map(|e| match e {
                Tag::Text(text) => Some(text),
            })
            .cloned()
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Deref for Ssml {
    type Target = Vec<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ssml {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Tag {
    #[serde(untagged)]
    Text(String),
}

/// TwiML Voice: <Say>
/// https://www.twilio.com/docs/voice/twiml/say
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
pub struct Say {
    #[serde(rename = "@language")]
    #[builder(default, setter(strip_option))]
    pub language: Option<Language>,
    #[serde(rename = "@voice")]
    #[builder(default, setter(strip_option))]
    pub voice: Option<Voice>,
    #[serde(rename = "@loop")]
    #[builder(default = 1)]
    pub loop_count: u32,
    #[serde(default, rename = "#text")]
    pub text: Ssml,
}

impl Say {
    pub fn text(&self) -> String {
        self.text.text()
    }
}

impl VoicePrice for Say {
    fn price(&self) -> Option<f32> {
        self.voice
            .and_then(|v| v.price())
            .map(|p| p * (self.text().len() / 100) as f32)
    }
}
