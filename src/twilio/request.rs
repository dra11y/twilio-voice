use super::deserialize_opt_usize;
use crate::Digits;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use struct_field_names_as_array::FieldNamesAsSlice;

#[derive(
    Debug, Default, Clone, strum::Display, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum CallStatus {
    /// The call is ready and waiting in line before going out.
    Queued,

    /// The call is currently ringing.
    Ringing,

    /// The call was answered and is actively in progress.
    #[default]
    InProgress,

    /// The call was answered and has ended normally.
    Completed,

    /// The caller received a busy signal.
    Busy,

    /// The call could not be completed as dialed, most likely because the phone number was non-existent.
    Failed,

    /// The call ended without being answered.
    NoAnswer,

    /// The call was canceled via the REST API while queued or ringing.
    Canceled,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    /// inbound calls
    #[default]
    Inbound,

    /// calls initiated via the REST API
    OutboundApi,

    /// calls initiated by a <Dial> verb
    OutboundDial,

    /// if using Elastic SIP Trunking, outgoing calls from your communications infrastructure to the PSTN
    TrunkingTerminating,

    /// if using Elastic SIP Trunking, incoming calls to your communications infrastructure from the PSTN
    TrunkingOriginating,
}

fn deserialize_status_callback<'de, D>(deserializer: D) -> Result<Option<StatusCallback>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let Some(obj) = value.as_object() else {
        return Ok(None);
    };

    let has_any_field = obj
        .keys()
        .any(|k| StatusCallback::FIELD_NAMES_AS_SLICE.contains(&k.as_str()));

    if !has_any_field {
        return Ok(None);
    }

    StatusCallback::deserialize(value)
        .map(Some)
        .map_err(serde::de::Error::custom)
}

fn deserialize_extra<'de, D>(deserializer: D) -> Result<HashMap<String, Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut value = Value::deserialize(deserializer)?;
    let Some(obj) = value.as_object_mut() else {
        return Ok(HashMap::new());
    };

    for field in StatusCallback::FIELD_NAMES_AS_SLICE {
        obj.remove(*field);
    }

    let map = obj
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<HashMap<String, Value>>();

    Ok(map)
}

/// StatusCallback Parameters
/// https://www.twilio.com/docs/voice/api/call-resource#statuscallback
#[derive(Debug, Default, Clone, FieldNamesAsSlice, PartialEq, Serialize, Deserialize)]
#[field_names_as_slice(rename_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub struct StatusCallback {
    /// The duration in minutes of the just-completed call; calls are billed by the minute. Only present in the `completed` event.
    #[serde(default, deserialize_with = "deserialize_opt_usize")]
    duration: Option<usize>,

    /// The duration in seconds of the just-completed call. Only present in the `completed` event.
    #[serde(default, deserialize_with = "deserialize_opt_usize")]
    call_duration: Option<usize>,

    /// The final SIP code for the call. For example, a number that was unreachable will return 404. If the Timeout value was reached before the call connected, this code will be 487.
    #[serde(default, deserialize_with = "deserialize_opt_usize")]
    sip_response_code: Option<usize>,

    /// The URL of the phone call's recorded audio. This parameter is included only if `Record=true` is set on the REST API request and does not include recordings initiated in other ways. `RecordingUrl` is only present in the `completed` event. The recording file may not yet be accessible when the Status Callback is sent.
    /// _**Note:**_ _Use RecordingStatusCallback for reliable notification on when the recording is available for access._
    #[serde(default)]
    recording_url: Option<String>,

    /// The unique ID of the [Recording](https://www.twilio.com/docs/voice/api/recording "Recording") from this call. `RecordingSid` is only present with the `completed` event.
    #[serde(default)]
    recording_sid: Option<String>,

    /// The duration of the recorded audio (in seconds). `RecordingDuration` is only present in the `completed` event. To get a final accurate recording duration after any trimming of silence, use `RecordingStatusCallback`.
    #[serde(default, deserialize_with = "deserialize_opt_usize")]
    recording_duration: Option<usize>,

    /// The timestamp when the event fired, given as UTC in [RFC 2822(link takes you to an external page)](https://php.net/manual/en/class.datetime.php#datetime.constants.rfc2822 "RFC 2822") format.
    #[serde(default)]
    timestamp: Option<String>,

    /// A string that describes the source of the webhook. This is provided to help disambiguate why the webhook was made. On Status Callbacks, this value is always `call-progress-events`.
    #[serde(default)]
    callback_source: Option<String>,

    /// The order in which the events were fired, starting from `0`. Although events are fired in order, they are made as separate HTTP requests, and there is no guarantee they will arrive in the same order.
    #[serde(default, deserialize_with = "deserialize_opt_usize")]
    sequence_number: Option<usize>,
}

/// Trusted Calling with SHAKEN/STIR
///
/// To understand the possible values for the `StirVerstat` parameter/`X-Twilio-VerStat` header, you will first need to understand the three different **attestation levels**:
/// - `A`: the highest attestation given by the originating service provider to indicate that the caller is known and has the right to use the phone number as the caller ID
/// - `B`: the customer is known, it is unknown if they have the right to use the caller ID being used
/// - `C`: it doesn't meet the requirements of A or B including international calls.
///
/// https://www.twilio.com/docs/voice/trusted-calling-with-shakenstir
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StirVerstat {
    // Twilio received the SIP INVITE, with a SHAKEN PASSporT, and was able to fetch the public certificate of the originating service provider from the Certificate Authority that signed the call to verify that no one tampered with the SIP INVITE during transit.
    // Attestation level `A`
    #[serde(rename = "TN-Validation-Passed-A")]
    TnValidationPassedA,

    /// Twilio received the SIP INVITE, with a SHAKEN PASSporT, and was able to fetch the public certificate of the originating service provider from the Certificate Authority that signed the call to verify that no one tampered with the SIP INVITE during transit.
    /// Attestation level `B`
    #[serde(rename = "TN-Validation-Passed-B")]
    TnValidationPassedB,

    /// Twilio received the SIP INVITE, with a SHAKEN PASSporT, and was able to fetch the public certificate of the originating service provider from the Certificate Authority that signed the call to verify that no one tampered with the SIP INVITE during transit.
    /// Attestation level `C`
    #[serde(rename = "TN-Validation-Passed-C")]
    TnValidationPassedC,

    /// Twilio was unable to verify the contents of the SHAKEN PASSporT.
    /// This could mean tampering or that Twilio could not retrieve the public certificate of the originating service provider from the Certificate Authority.
    /// Attestation level `A`
    #[serde(rename = "TN-Validation-Failed-A")]
    TnValidationFailedA,

    /// Twilio was unable to verify the contents of the SHAKEN PASSporT.
    /// This could mean tampering or that Twilio could not retrieve the public certificate of the originating service provider from the Certificate Authority.
    /// Attestation level `B`
    #[serde(rename = "TN-Validation-Failed-B")]
    TnValidationFailedB,

    /// Twilio was unable to verify the contents of the SHAKEN PASSporT.
    /// This could mean tampering or that Twilio could not retrieve the public certificate of the originating service provider from the Certificate Authority.
    /// Attestation level `C`
    #[serde(rename = "TN-Validation-Failed-C")]
    TnValidationFailedC,

    /// Possible causes:
    /// - A malformed [E.164](https://www.twilio.com/docs/glossary/what-e164 "E.164") phone number.
    /// - SHAKEN PASSporT format is invalid. It should consist of a header, payload, signature, and parameters.
    /// - SHAKEN PASSporT does not have required fields like `ppt` headers or `info` parameter.
    /// - SHAKEN PASSporT `orig` field doesn't match with actual `callerid` in the SIP messages (`P-Asserted-Identity`, `Remote-Party-Identity`, or `From` header).
    /// - SHAKEN PASSporT `dest` field doesn't match with the actual destination of the call in the SIP Request-URI.
    /// - SHAKEN PASSporT `iat` field is too old - more than 1 minutes from current timestamp or the SIP Date header value.
    #[serde(rename = "No-TN-Validation")]
    NoTnValidation,

    /// Twilio was unable to verify the contents of the SHAKEN PASSporT.
    /// This could mean tampering or that Twilio could not retrieve the public certificate of the originating service provider from the Certificate Authority.
    /// No attestation level determined.    #[serde(rename = "TN-Validation-Failed")]
    TnValidationFailed,

    /// Twilio received the SIP INVITE, with a SHAKEN PASSporT, and was able to fetch the public certificate of the Diverting service provider from the Certificate Authority that signed the call.
    /// This verifies that no one tampered with the SIP INVITE during transit.
    /// Attestation level `A`
    #[serde(rename = "TN-Validation-Passed-A-Diverted")]
    TnValidationPassedADiverted,

    /// Twilio received the SIP INVITE, with a SHAKEN PASSporT, and was able to fetch the public certificate of the Diverting service provider from the Certificate Authority that signed the call.
    /// This verifies that no one tampered with the SIP INVITE during transit.
    /// Attestation level `B`
    #[serde(rename = "TN-Validation-Passed-B-Diverted")]
    TnValidationPassedBDiverted,

    /// Twilio received the SIP INVITE, with a SHAKEN PASSporT, and was able to fetch the public certificate of the Diverting service provider from the Certificate Authority that signed the call.
    /// This verifies that no one tampered with the SIP INVITE during transit.
    /// Attestation level `C`
    #[serde(rename = "TN-Validation-Passed-C-Diverted")]
    TnValidationPassedCDiverted,

    /// Twilio was unable to verify the contents of the SHAKEN PASSporT.
    /// This could mean tampering or that Twilio could not retrieve the public certificate of the Diverting service provider from the Certificate Authority.
    /// Attestation level `A`
    #[serde(rename = "TN-Validation-Failed-A-Diverted")]
    TnValidationFailedADiverted,

    /// Twilio was unable to verify the contents of the SHAKEN PASSporT.
    /// This could mean tampering or that Twilio could not retrieve the public certificate of the Diverting service provider from the Certificate Authority.
    /// Attestation level `B`
    #[serde(rename = "TN-Validation-Failed-B-Diverted")]
    TnValidationFailedBDiverted,

    /// Twilio was unable to verify the contents of the SHAKEN PASSporT.
    /// This could mean tampering or that Twilio could not retrieve the public certificate of the Diverting service provider from the Certificate Authority.
    /// Attestation level `C`
    #[serde(rename = "TN-Validation-Failed-C-Diverted")]
    TnValidationFailedCDiverted,
}

/// Twilio Voice Webhook Request Parameters
/// https://www.twilio.com/docs/voice/twiml#request-parameters
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    /// A unique identifier for this call, generated by Twilio.
    pub call_sid: String,

    /// Your Twilio account ID. It is 34 characters long, and always starts with the letters AC.
    pub account_sid: String,

    /// The phone number or client identifier of the party that initiated the call. Phone numbers are formatted with a '+' and country code, e.g., +16175551212 (E.164 format). Client identifiers begin with the client: URI scheme; for example, on a call from a client named 'charlie', the From parameter will be client:charlie. If a caller ID is withheld or otherwise unavailable, you may receive a string that contains anonymous, unknown, or other descriptions.
    pub from: String,

    /// The phone number or client identifier of the called party. Phone numbers are formatted with a '+' and country code, e.g., +16175551212(E.164 format). Client identifiers begin with the client: URI scheme; for example, for a call to a client named 'joey', the To parameter will be client:joey.
    pub to: String,

    /// The version of the Twilio API used to handle this call. For incoming calls, this is determined by the API version set on the called number. For outgoing calls, this is the version used by the REST API request from the outgoing call.
    pub api_version: String,

    /// A string describing the direction of the call: inbound for inbound calls outbound-api for calls initiated via the REST API outbound-dial for calls initiated by a <Dial> verb.
    pub direction: Direction,

    /// This parameter is set only when Twilio receives a forwarded call, but its value depends on the caller's carrier including information when forwarding. Not all carriers support passing this information.
    #[serde(default)]
    pub forwarded_from: Option<String>,

    /// This parameter is set when the IncomingPhoneNumber that received the call has had its VoiceCallerIdLookup value set to true.
    #[serde(default)]
    pub caller_name: Option<String>,

    /// A unique identifier for the call that created this leg. This parameter is not passed if this is the first leg of a call.
    #[serde(default)]
    pub parent_call_sid: Option<String>,

    /// A token string needed to invoke a forwarded call.
    #[serde(default)]
    pub call_token: Option<String>,

    /// The city of the caller
    #[serde(default)]
    pub from_city: Option<String>,

    /// The state or province of the caller
    #[serde(default)]
    pub from_state: Option<String>,

    /// The postal code of the caller
    #[serde(default)]
    pub from_zip: Option<String>,

    /// The country of the caller
    #[serde(default)]
    pub from_country: Option<String>,

    /// The city of the called party
    #[serde(default)]
    pub to_city: Option<String>,

    /// The state or province of the called party
    #[serde(default)]
    pub to_state: Option<String>,

    /// The postal code of the called party
    #[serde(default)]
    pub to_zip: Option<String>,

    /// The country of the called party
    #[serde(default)]
    pub to_country: Option<String>,

    /// A descriptive status for the call. See [`CallStatus`].
    pub call_status: CallStatus,

    /// Trusted Calling with SHAKEN/STIR
    ///
    /// To understand the possible values for the `StirVerstat` parameter/`X-Twilio-VerStat` header, you will first need to understand the three different **attestation levels**:
    /// - `A`: the highest attestation given by the originating service provider to indicate that the caller is known and has the right to use the phone number as the caller ID
    /// - `B`: the customer is known, it is unknown if they have the right to use the caller ID being used
    /// - `C`: it doesn't meet the requirements of A or B including international calls.
    ///
    /// A `None` value means:
    ///   - SHAKEN/STIR is not enabled in the account, or
    ///   - Twilio was unable to verify the contents of the SHAKEN PASSporT.
    ///     This could mean tampering or that Twilio could not retrieve the public certificate of the originating service provider from the Certificate Authority.
    ///     No attestation level determined.
    ///
    /// https://www.twilio.com/docs/voice/trusted-calling-with-shakenstir
    #[serde(default)]
    pub stir_verstat: Option<StirVerstat>,

    /// If `<Gather input>` included `speech`, contains the transcribed result of the caller's speech.
    #[serde(default)]
    pub speech_result: Option<String>,

    /// If `<Gather input>` included `speech`, might contain (not guaranteed) a confidence score (between 0.0 and 1.0) of the accuracy of the transcription.
    #[serde(default)]
    pub confidence: Option<f64>,

    /// Any digits dialed by the caller in response to a <Gather>. Excludes the digit of the `finishOnKey` <Gather> parameter. Empty if no digits were entered.
    #[serde(default)]
    pub digits: Option<Digits>,

    /// Additional parameters included in a Twilio StatusCallback request.
    #[serde(flatten, deserialize_with = "deserialize_status_callback")]
    pub status_callback: Option<StatusCallback>,

    /// Any unknown parameters that we did not capture above.
    /// NOTE: Twilio might send some redundant fields that would show up in this map:
    /// - `Caller` = `From`
    /// - `CallerCity` = `FromCity`
    /// - `CallerState` = `FromState`
    /// - `CallerZip` = `FromZip`
    /// - `CallerCountry` = `FromCountry`
    /// - `Called` = `To`
    /// - `CalledCity` = `ToCity`
    /// - `CalledState` = `ToState`
    /// - `CalledZip` = `ToZip`
    /// - `CalledCountry` = `ToCountry`
    #[serde(flatten, deserialize_with = "deserialize_extra")]
    pub extra: HashMap<String, Value>,
}

impl Request {
    pub fn to_map(&self) -> crate::Result<BTreeMap<String, String>> {
        let encoded = serde_urlencoded::to_string(self)?;
        let pairs: Vec<(String, String)> = serde_urlencoded::from_str(&encoded)?;
        Ok(pairs.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Digit, errors::DigitsError};
    use serde_json::json;

    #[test]
    fn test_to_btreemap() {
        let request = Request {
            call_sid: "CA123".to_string(),
            account_sid: "AC456".to_string(),
            from: "+15551234".to_string(),
            to: "+15554321".to_string(),
            api_version: "2023-12-31".to_string(),
            direction: Direction::Inbound,
            call_status: CallStatus::Completed,
            digits: Some("123#".parse().unwrap()),
            ..Default::default()
        };

        let btreemap = request.to_map().expect("Failed to convert to BTreeMap");

        let keys = btreemap.keys().cloned().collect::<Vec<_>>();
        assert_eq!(
            keys,
            &[
                "AccountSid",
                "ApiVersion",
                "CallSid",
                "CallStatus",
                "Digits",
                "Direction",
                "From",
                "To"
            ]
        );

        assert_eq!(btreemap.get("CallSid"), Some(&"CA123".to_string()));
        assert_eq!(btreemap.get("AccountSid"), Some(&"AC456".to_string()));
        assert_eq!(btreemap.get("From"), Some(&"+15551234".to_string()));
        assert_eq!(btreemap.get("To"), Some(&"+15554321".to_string()));
        assert_eq!(btreemap.get("ApiVersion"), Some(&"2023-12-31".to_string()));
        assert_eq!(btreemap.get("Direction"), Some(&"inbound".to_string()));
        assert_eq!(btreemap.get("CallStatus"), Some(&"completed".to_string()));
        assert_eq!(btreemap.get("Digits"), Some(&"123#".to_string()));
    }

    // Base template with required Request fields
    fn base_request_json_for_status_callback() -> serde_json::Value {
        json!({
            "CallSid": "CA123",
            "AccountSid": "AC456",
            "From": "+15551234",
            "To": "+15554321",
            "ApiVersion": "2023-12-31",
            "Direction": "inbound",
            "CallStatus": "completed"
        })
    }

    #[test]
    fn test_deserialize_and_serialize_request() {
        let digits_str = "31#*";
        let params = format!(
            "Digits={digits_str}&CallSid=4321abcdef0&AccountSid=5678feabcd1&ApiVersion=2010&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress"
        );
        let req: Request =
            serde_urlencoded::de::from_str(&params).expect("failed to deserialize Request");
        let digits = req.digits.expect("digits should be Some");
        assert_eq!(digits.to_string(), digits_str);
        assert_eq!(digits.iter().last(), Some(&Digit::Star));
        let number = digits.to_int();
        assert_eq!(number.unwrap(), 31);
        assert_eq!(req.call_sid, "4321abcdef0");
        assert_eq!(req.account_sid, "5678feabcd1");
        assert_eq!(req.api_version, "2010");
        assert_eq!(req.direction, Direction::Inbound);
        assert_eq!(req.to, "+12125551234");
        assert_eq!(req.from, "+19193332345");
        assert_eq!(req.call_status, CallStatus::InProgress);
    }

    #[test]
    fn test_call_status_deserialization() {
        // Test all CallStatus variants through the Request struct
        for (status_str, status_enum) in [
            ("queued", CallStatus::Queued),
            ("ringing", CallStatus::Ringing),
            ("in-progress", CallStatus::InProgress),
            ("completed", CallStatus::Completed),
            ("busy", CallStatus::Busy),
            ("failed", CallStatus::Failed),
            ("no-answer", CallStatus::NoAnswer),
            ("canceled", CallStatus::Canceled),
        ] {
            let params = format!(
                "CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus={status_str}"
            );
            let req: Request = serde_urlencoded::from_str(&params).unwrap();
            assert_eq!(req.call_status, status_enum);
        }
    }

    #[test]
    fn test_direction_deserialization() {
        // Test all Direction variants through the Request struct
        for (direction_str, direction_enum) in [
            ("inbound", Direction::Inbound),
            ("outbound-api", Direction::OutboundApi),
            ("outbound-dial", Direction::OutboundDial),
        ] {
            let params = format!(
                "CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction={direction_str}&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress"
            );
            let req: Request = serde_urlencoded::from_str(&params).unwrap();
            assert_eq!(req.direction, direction_enum);
        }
    }

    #[test]
    fn test_request_with_optional_fields() {
        let params = "CallSid=abc123&AccountSid=AC123456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress&FromCity=Raleigh&FromState=NC&FromZip=27601&FromCountry=US&ToCity=New+York&ToState=NY&ToZip=10001&ToCountry=US&ForwardedFrom=%2B18005551212&CallerName=John+Doe&ParentCallSid=CAparent123&CallToken=token123";

        let req: Request = serde_urlencoded::from_str(params).unwrap();

        // Optional fields
        assert_eq!(req.from_city, Some("Raleigh".to_string()));
        assert_eq!(req.from_state, Some("NC".to_string()));
        assert_eq!(req.from_zip, Some("27601".to_string()));
        assert_eq!(req.from_country, Some("US".to_string()));
        assert_eq!(req.to_city, Some("New York".to_string()));
        assert_eq!(req.to_state, Some("NY".to_string()));
        assert_eq!(req.to_zip, Some("10001".to_string()));
        assert_eq!(req.to_country, Some("US".to_string()));
        assert_eq!(req.forwarded_from, Some("+18005551212".to_string()));
        assert_eq!(req.caller_name, Some("John Doe".to_string()));
        assert_eq!(req.parent_call_sid, Some("CAparent123".to_string()));
        assert_eq!(req.call_token, Some("token123".to_string()));
    }

    #[test]
    fn test_request_without_optional_fields() {
        let params = "CallSid=abc123&AccountSid=AC123456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress";

        let req: Request = serde_urlencoded::from_str(params).unwrap();

        // Optional fields should be None
        assert_eq!(req.from_city, None);
        assert_eq!(req.from_state, None);
        assert_eq!(req.from_zip, None);
        assert_eq!(req.from_country, None);
        assert_eq!(req.to_city, None);
        assert_eq!(req.to_state, None);
        assert_eq!(req.to_zip, None);
        assert_eq!(req.to_country, None);
        assert_eq!(req.forwarded_from, None);
        assert_eq!(req.caller_name, None);
        assert_eq!(req.parent_call_sid, None);
        assert_eq!(req.call_token, None);
    }

    #[test]
    fn test_empty_digits() {
        let params = "Digits=&CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress";
        let req: Request = serde_urlencoded::from_str(params).unwrap();

        // Check empty digits
        let digits = req.digits.expect("digits should be Some");
        assert!(digits.is_empty());
        assert_eq!(digits.to_string(), "");
        assert!(matches!(digits.to_int(), Err(DigitsError::Empty)));
    }

    #[test]
    fn test_missing_digits() {
        // Test when Digits parameter is entirely missing
        let params = "CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress";
        let req: Request = serde_urlencoded::from_str(params).unwrap();

        // Digits should be None when parameter is missing
        assert!(req.digits.is_none());
    }

    #[test]
    fn test_malformed_request() {
        // Test with missing required fields
        let params = "CallSid=CA123"; // Missing most required fields
        let result = serde_urlencoded::from_str::<Request>(params);
        assert!(result.is_err());

        // Test with invalid enum value
        let params = "CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=invalid&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress";
        let result = serde_urlencoded::from_str::<Request>(params);
        assert!(result.is_err());

        // Test with invalid call status
        let params = "CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=not-a-status";
        let result = serde_urlencoded::from_str::<Request>(params);
        assert!(result.is_err());
    }

    #[test]
    fn test_url_encoded_characters() {
        // Test with URL-encoded special characters in fields
        let params = "CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress&CallerName=John%20Doe%20%26%20Family";
        let req: Request = serde_urlencoded::from_str(params).unwrap();

        // Check the decoded special characters
        assert_eq!(req.caller_name, Some("John Doe & Family".to_string()));
    }

    #[test]
    fn test_plus_decoded_as_space() {
        // Test that + in URL-encoded data is decoded as space
        let params = "CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress&CallerName=John+Doe";
        let req: Request = serde_urlencoded::from_str(params).unwrap();

        // + should be decoded as space
        assert_eq!(req.caller_name, Some("John Doe".to_string()));
    }

    #[test]
    fn test_complex_digits() {
        // Test with various digit combinations
        let test_cases = [
            ("123", Some(123)),
            ("456#", Some(456)),
            ("789*", Some(789)),
            ("0*#", Some(0)),
            ("0A*#", None),
            ("*123", None),  // Starts with *, can't convert to int
            ("#456", None),  // Starts with #, can't convert to int
            ("12*34", None), // can't convert to int
            ("56#78", None), // can't convert to int
            ("*#", None),    // No digits to convert
        ];

        for (digit_str, expected_int) in test_cases {
            let params = format!(
                "Digits={digit_str}&CallSid=CA123&AccountSid=AC456&ApiVersion=2010-04-01&Direction=inbound&To=%2B12125551234&From=%2B19193332345&CallStatus=in-progress"
            );
            let req: Request = serde_urlencoded::from_str(&params).unwrap();

            let digits = req.digits.expect("digits should be Some");
            assert_eq!(digits.to_string(), digit_str);
            assert_eq!(digits.to_int().ok(), expected_int);
        }
    }

    #[test]
    fn status_callback_with_mixed_number_formats() {
        let mut json = base_request_json_for_status_callback();
        json["Duration"] = json!(30); // Number
        json["CallDuration"] = json!("45"); // String
        json["CallbackSource"] = json!("test");

        let req: Request = serde_json::from_value(json).unwrap();
        let sc = req.status_callback.unwrap();

        assert_eq!(sc.duration, Some(30));
        assert_eq!(sc.call_duration, Some(45));
        assert_eq!(sc.callback_source, Some("test".into()));
    }

    #[test]
    fn no_status_callback_fields() {
        let json = base_request_json_for_status_callback(); // No StatusCallback fields
        let req: Request = serde_json::from_value(json).unwrap();

        assert!(req.status_callback.is_none());
        assert!(req.extra.is_empty()); // All fields accounted for
    }

    #[test]
    fn partial_status_callback_fields() {
        let mut json = base_request_json_for_status_callback();
        json["SequenceNumber"] = json!("2");
        json["SipResponseCode"] = json!(404);

        let req: Request = serde_json::from_value(json).unwrap();
        let sc = req.status_callback.unwrap();

        assert_eq!(sc.sequence_number, Some(2));
        assert_eq!(sc.sip_response_code, Some(404));
        assert!(sc.duration.is_none(), "duration should be None");
        assert!(req.extra.is_empty(), "extra should be empty");
    }

    #[test]
    fn partial_status_callback_and_extra_fields() {
        let mut json = base_request_json_for_status_callback();
        json["SequenceNumber"] = json!("2");
        json["SipResponseCode"] = json!(404);
        json["SomeOtherField"] = json!("hello");

        let req: Request = serde_json::from_value(json).unwrap();
        let sc = req.status_callback.unwrap();

        assert_eq!(sc.sequence_number, Some(2));
        assert_eq!(sc.sip_response_code, Some(404));
        assert!(!req.extra.is_empty(), "extra should NOT be empty");
        assert_eq!(req.extra["SomeOtherField"], Value::from("hello"));
    }

    #[test]
    fn invalid_number_format() {
        let mut json = base_request_json_for_status_callback();
        json["Duration"] = json!("invalid");

        let result: Result<Request, _> = serde_json::from_value(json);
        assert!(result.is_err());
    }

    #[test]
    fn extra_fields() {
        let mut json = base_request_json_for_status_callback();
        json["UnknownField1"] = json!("some value");
        json["UnknownField2"] = json!(std::f64::consts::PI);

        let req: Request = serde_json::from_value(json).unwrap();
        assert!(!req.extra.is_empty());
        assert_eq!(req.extra["UnknownField1"], "some value");
        assert_eq!(req.extra["UnknownField2"], std::f64::consts::PI);
    }
}
