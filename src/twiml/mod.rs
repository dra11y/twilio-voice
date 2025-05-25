mod redirect;
pub use redirect::*;

mod responses;
pub use responses::*;

mod gather;
pub use gather::*;

mod play;
pub use play::*;

mod say;
pub use say::*;

pub mod voices;
pub use voices::{Gender, Language, Voice, VoiceGender, VoicePrice};

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, strum::Display, Default, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Method {
    GET,
    #[default]
    POST,
}
