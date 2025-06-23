use http::Method;
use serde::Deserialize;

use crate::twilio::CallStatus;

use super::{TwilioClient, TwilioClientResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundCall {
    pub from: String,
    pub to: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Calls {
    client: TwilioClient,
}

impl Calls {
    pub async fn create(&self, call: &OutboundCall) -> TwilioClientResult<Call> {
        let opts = &[
            ("To", &*call.to),
            ("From", &*call.from),
            ("Url", &*call.url),
        ];
        self.client.send_request(Method::POST, "Calls", opts).await
    }
}

#[derive(Debug, Deserialize)]
pub struct Call {
    pub from: String,
    pub to: String,
    pub sid: String,
    pub status: CallStatus,
}

impl TwilioClient {
    pub fn calls(&self) -> Calls {
        Calls {
            client: self.clone(),
        }
    }
}
