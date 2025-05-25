use std::collections::HashMap;

use crate::PriceType;

use super::{CallStatus, Direction};
use super::{deserialize_opt_price_type, deserialize_opt_string, deserialize_opt_usize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnsweredBy {
    Human,
    Machine,
}
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
// Twilio API JSON keys are snake_case, and webhook params are camelCase.
#[serde(rename_all = "snake_case")]
pub struct CallResource {
    /// The unique string that we created to identify this Call resource.
    /// Pattern: `^CA[0-9a-fA-F]{32}$`
    /// Min length: `34`
    /// Max length: `34`
    pub sid: String,

    /// The date and time in UTC that this resource was created specified in [RFC 2822(link takes you to an external page)](https://www.ietf.org/rfc/rfc2822.txt "RFC 2822") format.
    #[serde(default)]
    pub date_created: Option<String>,

    /// The date and time in UTC that this resource was last updated, specified in [RFC 2822(link takes you to an external page)](https://www.ietf.org/rfc/rfc2822.txt "RFC 2822") format.
    #[serde(default)]
    pub date_updated: Option<String>,

    /// The SID that identifies the call that created this leg.
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub parent_call_sid: Option<String>,

    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account "Account") that created this Call resource.
    pub account_sid: String,

    /// The phone number, SIP address, Client identifier or SIM SID that received this call. Phone numbers are in E.164 format (e.g., +16175551212). SIP addresses are formatted as name@company.com. Client identifiers are formatted client:name. SIM SIDs are formatted as sim:sid.
    pub to: String,

    /// The phone number, SIP address or Client identifier that received this call. Formatted for display. Non-North American phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164 "E.164") format (e.g., +442071838750).
    pub to_formatted: String,

    /// The phone number, SIP address, Client identifier or SIM SID that made this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164 "E.164") format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`. SIM SIDs are formatted as `sim:sid`.
    pub from: String,

    /// The calling phone number, SIP address, or Client identifier formatted for display. Non-North American phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164 "E.164") format (e.g., +442071838750).
    pub from_formatted: String,

    /// If the call was inbound, this is the SID of the IncomingPhoneNumber resource that received the call. If the call was outbound, it is the SID of the OutgoingCallerId resource from which the call was placed.
    /// Pattern: `^PN[0-9a-fA-F]{32}$`
    /// Min length: `34`
    /// Max length: `34`
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub phone_number_sid: Option<String>,

    /// The status of this call. Can be: `queued`, `ringing`, `in-progress`, `canceled`, `completed`, `failed`, `busy` or `no-answer`. See [Call Status Values](https://www.twilio.com/docs/voice/api/call-resource#call-status-values "Call Status Values") below for more information.
    /// Possible values:
    /// `queued``ringing``in-progress``completed``busy``failed``no-answer``canceled`
    pub status: CallStatus,

    /// The start time of the call, given as UTC in [RFC 2822(link takes you to an external page)](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822 "RFC 2822") format. Empty if the call has not yet been dialed.
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub start_time: Option<String>,

    /// The time the call ended, given as UTC in [RFC 2822(link takes you to an external page)](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822 "RFC 2822") format. Empty if the call did not complete successfully.
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub end_time: Option<String>,

    /// The length of the call in seconds. This value is empty for busy, failed, unanswered, or ongoing calls.
    #[serde(default, deserialize_with = "deserialize_opt_usize")]
    pub duration: Option<usize>,

    /// The charge for this call, in the currency associated with the account. Populated after the call is completed. May not be immediately available. The price associated with a call only reflects the charge for connectivity. Charges for other call-related features such as Answering Machine Detection, Text-To-Speech, and SIP REFER are not included in this value.
    #[serde(default, deserialize_with = "deserialize_opt_price_type")]
    pub price: Option<PriceType>,

    /// The currency in which `Price` is measured, in [ISO 4127(link takes you to an external page)](https://www.iso.org/iso/home/standards/currency_codes.htm "ISO 4127") format (e.g., `USD`, `EUR`, `JPY`). Always capitalized for calls.
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub price_unit: Option<String>,

    /// A string describing the direction of the call. Can be: `inbound` for inbound calls, `outbound-api` for calls initiated via the REST API or `outbound-dial` for calls initiated by a `<Dial>` verb. Using [Elastic SIP Trunking](https://www.twilio.com/docs/sip-trunking "Elastic SIP Trunking"), the values can be [`trunking-terminating`](https://www.twilio.com/docs/sip-trunking#termination) for outgoing calls from your communications infrastructure to the PSTN or [`trunking-originating`](https://www.twilio.com/docs/sip-trunking#origination) for incoming calls to your communications infrastructure from the PSTN.
    pub direction: Direction,

    /// Either human or machine if this call was initiated with answering machine detection. Empty otherwise.
    #[serde(default)]
    pub answered_by: Option<AnsweredBy>,

    /// The API version used to create the call.
    pub api_version: String,

    /// The forwarding phone number if this call was an incoming call forwarded from another number (depends on carrier supporting forwarding). Otherwise, empty.
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub forwarded_from: Option<String>,

    /// The Group SID associated with this call. If no Group is associated with the call, the field is empty.
    /// Pattern: `^GP[0-9a-fA-F]{32}$`
    /// Min length: `34`
    /// Max length: `34`
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub group_sid: Option<String>,

    /// The caller's name if this call was an incoming call to a phone number with caller ID Lookup enabled. Otherwise, empty.
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub caller_name: Option<String>,

    /// The wait time in milliseconds before the call is placed.
    #[serde(default, deserialize_with = "deserialize_opt_usize")]
    pub queue_time: Option<usize>,

    /// The unique identifier of the trunk resource that was used for this call. The field is empty if the call was not made using a SIP trunk or if the call is not terminated.
    /// Pattern: ^TK[0-9a-fA-F]{32}$
    /// Min length: 34
    /// Max length: 34
    #[serde(default, deserialize_with = "deserialize_opt_string")]
    pub trunk_sid: Option<String>,

    /// The URI of this resource, relative to https://api.twilio.com.
    pub uri: String,

    /// A list of subresources available to this call, identified by their URIs relative to `https://api.twilio.com`.
    pub subresource_uris: HashMap<String, String>,
}
