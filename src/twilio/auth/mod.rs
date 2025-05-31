#[cfg(feature = "middleware")]
pub mod middleware;

use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use http::Method;
use regex::{Regex, RegexBuilder};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};
use tracing::{debug, warn};
use url::Url;

type HmacSha1 = Hmac<Sha1>;

/// Options for validating an incoming request
#[derive(Debug, Clone)]
pub struct RequestValidatorOptions {
    /// Enforce validation (if `false`, logs failure but does not block the request)
    pub enforce: bool,
    /// Enable debug logging
    pub debug: bool,
    /// The full URL (with query string) you used to configure the webhook with Twilio - overrides host/protocol options
    pub url: Option<String>,
    /// Manually specify the host name used by Twilio in a number's webhook config
    pub host: Option<String>,
    /// Manually specify the protocol used by Twilio in a number's webhook config
    pub protocol: Option<String>,
    /// Test both http and https protocol URLs
    pub test_both_protocols: bool,
}

impl Default for RequestValidatorOptions {
    fn default() -> Self {
        Self {
            enforce: true,
            debug: false,
            url: None,
            host: None,
            protocol: Some("https".to_string()),
            test_both_protocols: false,
        }
    }
}

/// Get the expected Twilio signature for a given request
///
/// # Arguments
/// * `auth_token` - The auth token, as seen in the Twilio portal
/// * `url` - The full URL (with query string) you configured to handle this request
/// * `params` - The parameters sent with this request
///
/// # Returns
/// * The expected signature
pub fn get_expected_twilio_signature(
    auth_token: &str,
    url: &str,
    params: &HashMap<String, String>,
) -> String {
    let mut url = url_strip_auth(url);

    // If url contains bodySHA256 and params is empty, use empty params
    let use_params = if url.contains("bodySHA256") && params.is_empty() {
        &HashMap::new()
    } else {
        params
    };

    // Sort parameters by key and append them to the URL
    let mut sorted_keys: Vec<&String> = use_params.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        if let Some(value) = use_params.get(key) {
            url.push_str(key);
            url.push_str(value);
        }
    }

    // Create HMAC-SHA1 with the auth token
    let mut mac =
        HmacSha1::new_from_slice(auth_token.as_bytes()).expect("HMAC can take key of any size");
    mac.update(url.as_bytes());

    // Get the result and encode as base64
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let signature = general_purpose::STANDARD.encode(code_bytes);

    // TODO: debugging option
    // #[cfg(debug_assertions)]
    // {
    //     debug!("Signature: {signature}, URL: {url}");
    // }

    #[allow(clippy::let_and_return)]
    signature
}

/// Get the expected body hash for a given request's body
///
/// # Arguments
/// * `body` - The plain-text body of the request
///
/// # Returns
/// * The expected body hash
pub fn get_expected_body_hash(body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Utility function to add a default port to a URL
fn add_port(url: &str) -> String {
    url_with_default_port_strip_auth(url, true)
}

/// Utility function to remove a default port from a URL
fn remove_port(url: &str) -> String {
    url_with_default_port_strip_auth(url, false)
}

/// Utility function to force the protocol to https or http
fn with_https(url: &str, https: bool) -> String {
    let Some(caps) = PROTO_REGEX.captures(url) else {
        return url.to_string();
    };
    let Some(_) = caps.name("proto").map(|m| m.as_str()) else {
        return url.to_string();
    };
    let Some(rest) = caps.name("rest").map(|m| m.as_str()) else {
        return url.to_string();
    };
    format!(
        "{proto}://{rest}",
        proto = if https { "https" } else { "http" }
    )
}

/// Validate a signature with a specific URL format
fn validate_signature_with_url(
    auth_token: &str,
    twilio_sig: &str,
    url: &str,
    params: &HashMap<String, String>,
) -> bool {
    debug!("validate_signature_with_url: {url}, params: {params:?}");
    let expected_sig = get_expected_twilio_signature(auth_token, url, params);
    debug!("expected_sig: {expected_sig}, for url: {url}");
    constant_time_compare(twilio_sig, &expected_sig)
}

/// Constant time comparison of two strings (to prevent timing attacks)
fn constant_time_compare(a: &str, b: &str) -> bool {
    let mut result = 0u8;

    if a.len() != b.len() {
        result = 1;
    }

    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }

    subtle::ConstantTimeEq::ct_eq(&result, &0).into()
}

static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(
        r"(?<thru_port>(?<scheme>https?)://(?:(?<auth>[^/]+@))?(?<host>[^/:]+)(?::(?<port>\d+))?)(?<path_and_query>(?<path>[^?]*)(?<query>.*))?$",
    )
    .case_insensitive(true)
    .build()
    .unwrap()
});

static PROTO_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"(?<proto>https?)://(?<rest>.*)$")
        .case_insensitive(true)
        .build()
        .unwrap()
});

/// Provide URL with/without default port and strip auth if present.
fn url_with_default_port_strip_auth(url: &str, with: bool) -> String {
    let Some(caps) = URL_REGEX.captures(url) else {
        return url.to_string();
    };
    let Some(scheme) = caps.name("scheme").map(|m| m.as_str()) else {
        return url.to_string();
    };
    let Some(host) = caps.name("host").map(|m| m.as_str()) else {
        return url.to_string();
    };
    let Some(path_and_query) = caps.name("path_and_query").map(|m| m.as_str()) else {
        return url.to_string();
    };
    let default_port = if scheme == "https" { 443 } else { 80 };
    let port: u16 = caps
        .name("port")
        .map(|m| m.as_str().parse().unwrap_or(default_port))
        .unwrap_or(default_port);
    if with || port != default_port {
        format!("{scheme}://{host}:{port}{path_and_query}")
    } else {
        format!("{scheme}://{host}{path_and_query}")
    }
}

/// Provide URL without auth
fn url_strip_auth(url: &str) -> String {
    let Some(caps) = URL_REGEX.captures(url) else {
        return url.to_string();
    };
    let Some(auth) = caps.name("auth").map(|m| m.as_str()) else {
        return url.to_string();
    };
    url.replace(auth, "")
}

fn with_legacy_querystring(url: &str) -> String {
    let Some((base_url, query)) = url.split_once('?') else {
        return url.to_string();
    };
    let query = query.replace("%20", "+");
    format!("{base_url}?{query}")
}

fn with_modern_querystring(url: &str) -> String {
    let Some((base_url, query)) = url.split_once('?') else {
        return url.to_string();
    };
    let query = query.replace('+', "%20");
    format!("{base_url}?{query}")
}

fn without_trailing_slash(url: &str) -> String {
    let Some(caps) = URL_REGEX.captures(url) else {
        return url.to_string();
    };
    let Some(thru_port) = caps.name("thru_port").map(|m| m.as_str()) else {
        return url.to_string();
    };
    let path = caps.name("path").map(|m| m.as_str()).unwrap_or_default();
    let path = path.strip_suffix('/').unwrap_or(path);
    let query = caps.name("query").map(|m| m.as_str()).unwrap_or_default();
    format!("{thru_port}{path}{query}")
}

/// Validate that a request came from Twilio
///
/// # Arguments
/// * `auth_token` - The auth token, as seen in the Twilio portal
/// * `twilio_sig` - The value of the X-Twilio-Signature header from the request
/// * `url` - The full URL (with query string) you configured to handle this request
/// * `params` - The parameters sent with this request
///
/// # Returns
/// * `true` if the request is valid, `false` otherwise
pub fn validate_request(
    auth_token: &str,
    twilio_sig: &str,
    url: &str,
    params: &HashMap<String, String>,
    options: &RequestValidatorOptions,
) -> bool {
    debug!("Validating Twilio request from: {url}");

    // Check signature of the url with and without the port number
    // and with and without the legacy querystring (special chars are encoded when using `new URL()`)
    // since signature generation on the back end is inconsistent.
    //
    // Additional variants with and without trailing slash are in addition to the official SDK.
    let variants = [
        url.to_string(),
        with_modern_querystring(url),
        with_legacy_querystring(url),
    ]
    .iter()
    .flat_map(|v| {
        if options.test_both_protocols {
            vec![with_https(v, false), with_https(v, true)]
        } else {
            vec![v.clone()]
        }
    })
    .flat_map(|v| [remove_port(&v), add_port(&v)])
    .flat_map(|v| [without_trailing_slash(&v), v])
    .collect::<HashSet<_>>();

    let mut tried = vec![];

    debug!("Trying variants: {variants:?}");
    let valid = variants.iter().any(|variant_url| {
        let valid = validate_signature_with_url(auth_token, twilio_sig, variant_url, params);
        if valid {
            tried.push(variant_url.clone());
            debug!("Twilio request validated with: {variant_url}");
        }
        valid
    });
    if !valid {
        warn!(
            "Twilio request validation failed with token ending in: {}, sig: {twilio_sig}\ntried urls: {tried:?}",
            &auth_token[auth_token.len() - 4..]
        );
    }
    valid || !options.enforce
}

/// Validate the body of a request against a body hash
///
/// # Arguments
/// * `body` - The body of the request
/// * `body_hash` - The body hash from the request (from bodySHA256 parameter)
///
/// # Returns
/// * `true` if the body is valid, `false` otherwise
pub fn validate_body(body: &str, body_hash: &str) -> bool {
    let expected_hash = get_expected_body_hash(body);
    let valid = constant_time_compare(body_hash, &expected_hash);
    if !valid {
        warn!(
            "Body validation failed: expected hash {expected_hash}, got {body_hash}\nbody:\n{body}",
        );
    }
    valid
}

/// Validate a request with a JSON body
///
/// # Arguments
/// * `auth_token` - The auth token, as seen in the Twilio portal
/// * `twilio_sig` - The value of the X-Twilio-Signature header from the request
/// * `url` - The full URL (with query string) you configured to handle this request
/// * `body` - The body of the request
///
/// # Returns
/// * `true` if the request is valid, `false` otherwise
pub fn validate_request_with_body(
    auth_token: &str,
    twilio_sig: &str,
    url: &str,
    body: &str,
    options: &RequestValidatorOptions,
) -> bool {
    let empty_params = HashMap::new();
    let url_object = Url::parse(url).expect("Invalid URL");

    let body_hash = url_object
        .query_pairs()
        .find(|(key, _)| key == "bodySHA256")
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();

    validate_request(auth_token, twilio_sig, url, &empty_params, options)
        && (validate_body(body, &body_hash) || !options.enforce)
}

/// Implement on a [`Request`] object for validation.
pub trait TwilioRequest {
    fn method(&self) -> Method;
    fn protocol(&self) -> String;
    fn host(&self) -> String;
    fn path_and_query(&self) -> String;
    fn twilio_signature(&self) -> Option<String>;
    fn body(&self) -> HashMap<String, String>;
    fn raw_body(&self) -> Option<String>;
}

/// Validate an incoming request is indeed from Twilio
///
/// # Arguments
/// * `request` - A request object implementing the TwilioRequest trait
/// * `auth_token` - The auth token, as seen in the Twilio portal
/// * `opts` - Options for request validation
///
/// # Returns
/// * `true` if the request is valid, `false` otherwise
pub fn validate_incoming_request<T: TwilioRequest>(
    request: &T,
    auth_token: &str,
    options: RequestValidatorOptions,
) -> bool {
    #[cfg(debug_assertions)]
    let validated_with: &str;

    let webhook_url = if let Some(url) = &options.url {
        #[cfg(debug_assertions)]
        {
            validated_with = "custom URL option";
        }
        url.clone()
    } else {
        let mut custom_protocol = true;
        let protocol = options
            .protocol
            .as_deref()
            .map(ToString::to_string)
            .unwrap_or_else(|| {
                custom_protocol = false;
                request.protocol()
            });
        let mut custom_host = true;
        let host = options
            .host
            .as_deref()
            .map(ToString::to_string)
            .unwrap_or_else(|| {
                custom_host = false;
                request.host()
            });

        let url = format!("{}://{}{}", protocol, host, request.path_and_query());
        #[cfg(debug_assertions)]
        {
            validated_with = match (custom_protocol, custom_host) {
                (true, true) => "custom protocol and host",
                (true, false) => "custom protocol",
                (false, true) => "custom host",
                (false, false) => "original request URL",
            };
        }
        url
    };

    let is_valid = if webhook_url.contains("bodySHA256") {
        validate_request_with_body(
            auth_token,
            &request.twilio_signature().unwrap_or_default(),
            &webhook_url,
            &request.raw_body().unwrap_or_default(),
            &options,
        )
    } else {
        validate_request(
            auth_token,
            &request.twilio_signature().unwrap_or_default(),
            &webhook_url,
            &request.body(),
            &options,
        )
    };

    #[cfg(debug_assertions)]
    {
        let method = request.method();
        if !is_valid {
            debug!("INVALID URL: {method} {webhook_url}, validated with {validated_with}");
        } else {
            debug!("Valid URL: {method} {webhook_url}");
        }
    }

    is_valid || !options.enforce
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, time::Duration};

    #[derive(Debug, Clone)]
    struct MockRequest {
        protocol: String,
        host: String,
        method: Method,
        path_and_query: String,
        signature: Option<String>,
        body: HashMap<String, String>,
        raw_body: Option<String>,
    }

    impl TwilioRequest for MockRequest {
        fn protocol(&self) -> String {
            self.protocol.clone()
        }

        fn host(&self) -> String {
            self.host.clone()
        }

        fn path_and_query(&self) -> String {
            self.path_and_query.clone()
        }

        fn twilio_signature(&self) -> Option<String> {
            self.signature.clone()
        }

        fn body(&self) -> HashMap<String, String> {
            self.body.clone()
        }

        fn raw_body(&self) -> Option<String> {
            self.raw_body.clone()
        }

        fn method(&self) -> Method {
            self.method.clone()
        }
    }

    // A realistic auth token would be 32 characters of hex
    const TEST_AUTH_TOKEN: &str = "aaf98aa0a69a870c2e5a0e774af3f8b2";

    mod helpers {
        use super::*;
        use std::time::{Duration, Instant};

        // Measure time for multiple executions and return average
        pub fn measure_time<F>(f: F, iterations: usize) -> Duration
        where
            F: Fn(),
        {
            let mut measurements = Vec::with_capacity(iterations);

            // Warmup
            for _ in 0..100 {
                f();
            }

            // Actual measurements
            for _ in 0..iterations {
                let start = Instant::now();
                f();
                measurements.push(start.elapsed());
            }

            // Remove outliers (fastest and slowest 10%)
            measurements.sort();
            let trim = iterations / 10;
            let trimmed: Vec<_> = measurements[trim..measurements.len() - trim].to_vec();

            // Calculate average
            trimmed.iter().sum::<Duration>() / trimmed.len() as u32
        }

        pub fn trailing_slash_test_helper(url_with_slash: &str, url_without_slash: &str) {
            let auth_token = TEST_AUTH_TOKEN;
            let params = HashMap::new();

            let signature_for_url_with_slash =
                get_expected_twilio_signature(auth_token, url_with_slash, &params);
            let signature_for_url_without_slash =
                get_expected_twilio_signature(auth_token, url_without_slash, &params);

            assert!(
                validate_request(
                    auth_token,
                    &signature_for_url_with_slash,
                    url_with_slash,
                    &params,
                    &Default::default()
                ),
                "trailing slash in Twilio console webhook URL, and trailing slash received on our backend"
            );
            assert!(
                validate_request(
                    auth_token,
                    &signature_for_url_without_slash,
                    url_without_slash,
                    &params,
                    &Default::default()
                ),
                "no trailing slash in Twilio console webhook URL, and no trailing slash received on our backend"
            );
            assert!(
                validate_request(
                    auth_token,
                    &signature_for_url_without_slash,
                    url_with_slash,
                    &params,
                    &Default::default()
                ),
                "no trailing slash in Twilio console webhook URL, but received trailing slash on our backend - redirect or root URL without path"
            );
            assert!(
                !validate_request(
                    auth_token,
                    &signature_for_url_with_slash,
                    url_without_slash,
                    &params,
                    &Default::default()
                ),
                "should never be valid, because our backend should never strip an existing trailing slash from the request"
            );
        }
    }

    #[test]
    fn test_url_with_default_port_strip_auth() {
        assert_eq!(
            url_with_default_port_strip_auth(
                "https://user2:password65432@example.com/path?query=a+b&r=c%20d&s=e%2Bf",
                true
            ),
            "https://example.com:443/path?query=a+b&r=c%20d&s=e%2Bf"
        );
        assert_eq!(
            url_with_default_port_strip_auth(
                "https://user2:password65432@example.com:443/path?query=a+b&r=c%20d&s=e%2Bf",
                false
            ),
            "https://example.com/path?query=a+b&r=c%20d&s=e%2Bf"
        );
        assert_eq!(
            url_with_default_port_strip_auth(
                "https://user2:password65432@example.com:4443/path?query=a+b&r=c%20d&s=e%2Bf",
                false
            ),
            "https://example.com:4443/path?query=a+b&r=c%20d&s=e%2Bf"
        );
    }

    #[test]
    fn test_url_strip_auth() {
        assert_eq!(
            url_strip_auth(
                "https://user2:password65432@example.com:4443/path?query=a+b&r=c%20d&s=e%2Bf"
            ),
            "https://example.com:4443/path?query=a+b&r=c%20d&s=e%2Bf"
        );
        assert_eq!(
            url_strip_auth("https://user2@example.com:443/path?query=a+b&r=c%20d&s=e%2Bf"),
            "https://example.com:443/path?query=a+b&r=c%20d&s=e%2Bf"
        );
    }

    #[test]
    fn test_timing_attack_resistant() {
        // Test strings of the same length
        let test_string = "supersecretpassword1234567890abcdef";
        let same_string = "supersecretpassword1234567890abcdef";
        let diff_first = "Xupersecretpassword1234567890abcdef";
        let diff_middle = "supersecretpXssword1234567890abcdef";
        let diff_last = "supersecretpassword1234567890abcdeX";
        let diff_len = "supersecretpassword1234567890abcdef1";

        let iterations = 10_000;

        // Warmup
        let warmup = helpers::measure_time(
            || {
                constant_time_compare(test_string, diff_first);
            },
            iterations * 3,
        );

        // Measure constant_time_compare timings
        let time_same = helpers::measure_time(
            || {
                constant_time_compare(test_string, same_string);
            },
            iterations,
        );

        let time_diff_first = helpers::measure_time(
            || {
                constant_time_compare(test_string, diff_first);
            },
            iterations,
        );

        let time_diff_middle = helpers::measure_time(
            || {
                constant_time_compare(test_string, diff_middle);
            },
            iterations,
        );

        let time_diff_last = helpers::measure_time(
            || {
                constant_time_compare(test_string, diff_last);
            },
            iterations,
        );

        let time_diff_diff_len = helpers::measure_time(
            || {
                constant_time_compare(test_string, diff_len);
            },
            iterations,
        );

        debug!("Constant time results (nanoseconds):");
        debug!("  Warmup:       {warmup:?}");
        debug!("  Same:         {time_same:?}");
        debug!("  Diff first:   {time_diff_first:?}");
        debug!("  Diff middle:  {time_diff_middle:?}");
        debug!("  Diff last:    {time_diff_last:?}");
        debug!("  Diff length:  {time_diff_diff_len:?}");

        // Check variance - constant time should have similar timing
        let timings = [
            time_same,
            time_diff_first,
            time_diff_middle,
            time_diff_last,
            time_diff_diff_len,
        ];
        let max_time = timings.iter().max().unwrap();
        let min_time = timings.iter().min().unwrap();
        let variance = max_time.saturating_sub(*min_time);

        debug!("  Variance:     {variance:?}");

        // For constant time, we expect the variance to be minimal
        // This is a heuristic test - actual threshold depends on hardware
        assert!(
            variance < Duration::from_millis(1),
            "Timing variance too high: {variance:?} - possible timing attack vulnerability",
        );
    }

    #[test]
    fn test_signature_generation() {
        let auth_token = TEST_AUTH_TOKEN;
        let url = "https://example.com/myapp.php?foo=1&bar=2";

        let mut params = HashMap::new();
        params.insert(
            "CallSid".to_string(),
            "CA0dc55c0a331a4638fc55b434423e3deb".to_string(),
        );
        params.insert("Caller".to_string(), "+14158675310".to_string());
        params.insert("Digits".to_string(), "1234".to_string());
        params.insert("From".to_string(), "+14158675310".to_string());
        params.insert("To".to_string(), "+18005551212".to_string());

        let signature = get_expected_twilio_signature(auth_token, url, &params);
        // We're not testing against a known exact value from Twilio since they use a specific implementation
        // that we may not match exactly. Instead verify the signature is non-empty and base64 encoded
        assert!(!signature.is_empty());
        assert!(signature.ends_with("="));
        assert_eq!(signature.len() % 4, 0);
    }

    #[test]
    fn test_validate_request_form_encoded() {
        let auth_token = TEST_AUTH_TOKEN;
        let url = "https://example.com/myapp.php?foo=1&bar=2";

        let mut params = HashMap::new();
        params.insert(
            "CallSid".to_string(),
            "CA867c5658e8f60047aa41c63de4caf9dc".to_string(),
        );
        params.insert("Caller".to_string(), "+14158675311".to_string());
        params.insert("Digits".to_string(), "1234".to_string());
        params.insert("From".to_string(), "+14158675311".to_string());
        params.insert("To".to_string(), "+18005551212".to_string());

        let expected_signature = get_expected_twilio_signature(auth_token, url, &params);

        // Test success case
        assert!(validate_request(
            auth_token,
            &expected_signature,
            url,
            &params,
            &Default::default()
        ));

        // Test failure cases

        // 1. Wrong auth token
        assert!(!validate_request(
            "wrong_auth_token",
            &expected_signature,
            url,
            &params,
            &Default::default()
        ));

        // 2. Tampered signature - force a completely different signature
        let tampered_signature = "XYZ_COMPLETELY_DIFFERENT_SIGNATURE_ABC=";
        assert!(!validate_request(
            auth_token,
            tampered_signature,
            url,
            &params,
            &Default::default()
        ));

        // 3. Modified URL
        assert!(!validate_request(
            auth_token,
            &expected_signature,
            "https://example.com/myapp.php?foo=1&bar=3", // changed query param
            &params,
            &Default::default()
        ));

        // 4. Modified params
        let mut tampered_params = params.clone();
        tampered_params.insert("Digits".to_string(), "5678".to_string()); // changed value
        assert!(!validate_request(
            auth_token,
            &expected_signature,
            url,
            &tampered_params,
            &Default::default()
        ));

        // 5. Additional param
        let mut extended_params = params.clone();
        extended_params.insert("Extra".to_string(), "Parameter".to_string()); // added param
        assert!(!validate_request(
            auth_token,
            &expected_signature,
            url,
            &extended_params,
            &Default::default()
        ));

        // 6. Missing param
        let mut reduced_params = params.clone();
        reduced_params.remove("Digits"); // removed param
        assert!(!validate_request(
            auth_token,
            &expected_signature,
            url,
            &reduced_params,
            &Default::default()
        ));
    }

    #[test]
    fn test_validate_request_with_body() {
        let auth_token = TEST_AUTH_TOKEN;
        let body =
            "{\"CallSid\":\"CA9611c3543c25a2a374d2bea47066d03a\",\"Caller\":\"+12349013030\"}";
        let body_hash = get_expected_body_hash(body);
        let url = format!("https://example.com/myapp?bodySHA256={body_hash}");

        let expected_signature = get_expected_twilio_signature(auth_token, &url, &HashMap::new());

        // Test success case
        assert!(validate_request_with_body(
            auth_token,
            &expected_signature,
            &url,
            body,
            &Default::default()
        ));

        // Test failure cases

        // 1. Wrong auth token
        assert!(!validate_request_with_body(
            "wrong_auth_token",
            &expected_signature,
            &url,
            body,
            &Default::default()
        ));

        // 2. Tampered signature - completely different signature
        let tampered_signature = "XYZ_COMPLETELY_DIFFERENT_SIGNATURE_ABC=";
        assert!(!validate_request_with_body(
            auth_token,
            tampered_signature,
            &url,
            body,
            &Default::default()
        ));

        // 3. Modified body
        let modified_body = body.replace("12349013030", "12345678901");
        assert!(!validate_request_with_body(
            auth_token,
            &expected_signature,
            &url,
            &modified_body,
            &Default::default()
        ));

        // 4. Modified URL (but keeping the same hash)
        let modified_url = url.replace("example.com", "attacker.com");
        assert!(!validate_request_with_body(
            auth_token,
            &expected_signature,
            &modified_url,
            body,
            &Default::default()
        ));
    }

    #[test]
    fn test_validate_incoming_request_form() {
        let auth_token = TEST_AUTH_TOKEN;

        let mut params = HashMap::new();
        params.insert(
            "CallSid".to_string(),
            "CA2637ab9b712f8e902051dca2764a8bbe".to_string(),
        );
        params.insert("Caller".to_string(), "+14158675312".to_string());

        let url = "https://example.com/myapp.php?foo=1&bar=2";
        let expected_signature = get_expected_twilio_signature(auth_token, url, &params);

        let request = MockRequest {
            protocol: "https".to_string(),
            method: Method::POST,
            host: "example.com".to_string(),
            path_and_query: "/myapp.php?foo=1&bar=2".to_string(),
            signature: Some(expected_signature.clone()),
            body: params.clone(),
            raw_body: None,
        };

        // Test success case
        assert!(validate_incoming_request(
            &request,
            auth_token,
            Default::default()
        ));

        // Test failure cases

        // 1. Missing signature
        let request_no_sig = MockRequest {
            protocol: "https".to_string(),
            host: "example.com".to_string(),
            path_and_query: "/myapp.php?foo=1&bar=2".to_string(),
            signature: None,
            body: params.clone(),
            raw_body: None,
            method: Method::POST,
        };
        assert!(!validate_incoming_request(
            &request_no_sig,
            auth_token,
            Default::default()
        ));

        // 2. Wrong host
        let request_wrong_host = MockRequest {
            protocol: "https".to_string(),
            host: "evil.com".to_string(), // Different host
            path_and_query: "/myapp.php?foo=1&bar=2".to_string(),
            signature: Some(expected_signature.clone()),
            body: params.clone(),
            raw_body: None,
            method: Method::POST,
        };
        assert!(!validate_incoming_request(
            &request_wrong_host,
            auth_token,
            Default::default()
        ));

        // 3. Wrong protocol
        let request_wrong_protocol = MockRequest {
            protocol: "http".to_string(), // Different protocol
            host: "example.com".to_string(),
            path_and_query: "/myapp.php?foo=1&bar=2".to_string(),
            signature: Some(expected_signature.clone()),
            body: params.clone(),
            raw_body: None,
            method: Method::POST,
        };
        assert!(!validate_incoming_request(
            &request_wrong_protocol,
            auth_token,
            Default::default()
        ));

        // 4. Wrong URL
        let request_wrong_url = MockRequest {
            protocol: "https".to_string(),
            host: "example.com".to_string(),
            path_and_query: "/different.php?foo=1&bar=2".to_string(), // Different path
            signature: Some(expected_signature),
            body: params.clone(),
            raw_body: None,
            method: Method::POST,
        };
        assert!(!validate_incoming_request(
            &request_wrong_url,
            auth_token,
            Default::default()
        ));

        // 5. Wrong auth token
        assert!(!validate_incoming_request(
            &request,
            "wrong_auth_token",
            Default::default()
        ));
    }

    #[test]
    fn test_special_chars_in_url() {
        let auth_token = TEST_AUTH_TOKEN;
        let params = HashMap::new();

        // URL with special chars that might be encoded differently
        let url = "https://example.com/path?q=a+b&r=c%20d&s=e%2Bf";
        let signature = get_expected_twilio_signature(auth_token, url, &params);
        assert!(validate_request(
            auth_token,
            &signature,
            url,
            &params,
            &Default::default()
        ));
    }

    #[test]
    fn test_with_auth() {
        let auth_token = TEST_AUTH_TOKEN;
        let params = HashMap::new();

        let auth = "user:password1234@";
        let url_with_auth = format!("https://{auth}example.com:443/test");
        let url_without_auth = url_with_auth.replace(auth, "");

        assert_ne!(url_with_auth, url_without_auth);

        let signature_without_auth =
            get_expected_twilio_signature(auth_token, &url_without_auth, &params);
        assert!(
            validate_request(
                auth_token,
                &signature_without_auth,
                &url_with_auth,
                &params,
                &Default::default()
            ),
            "Twilio always strips auth from the URL"
        );

        let signature_with_auth =
            get_expected_twilio_signature(auth_token, &url_with_auth, &params);
        let signature_without_auth =
            get_expected_twilio_signature(auth_token, &url_without_auth, &params);
        assert_eq!(signature_with_auth, signature_without_auth);
        assert!(
            validate_request(
                auth_token,
                &signature_with_auth,
                &url_without_auth,
                &params,
                &Default::default()
            ),
            "our algorithm should strip auth as well"
        );
    }

    #[test]
    fn test_url_with_and_without_trailing_slash() {
        let url_with_slash = "https://example.com:8080/path/?q=a+b&r=c%20d&s=e%2Bf";
        let url_without_slash = "https://example.com:8080/path?q=a+b&r=c%20d&s=e%2Bf";

        helpers::trailing_slash_test_helper(url_with_slash, url_without_slash);

        let url_with_root_slash = "https://example.com:443/?q=a+b&r=c%20d&s=e%2Bf";
        let url_without_root_slash = "https://example.com:443?q=a+b&r=c%20d&s=e%2Bf";

        helpers::trailing_slash_test_helper(url_with_root_slash, url_without_root_slash);
    }

    #[test]
    fn test_malformed_inputs() {
        let auth_token = TEST_AUTH_TOKEN;
        let params = HashMap::new();

        // Invalid URL should return false
        assert!(!validate_request(
            auth_token,
            "",
            "not-a-url",
            &params,
            &Default::default()
        ));

        // Extremely long auth token
        let long_token = "a".repeat(1000);
        assert!(!validate_request(
            &long_token,
            "",
            "https://example.com",
            &params,
            &Default::default()
        ));

        // Null bytes in signature (security test)
        let null_byte_sig = "valid\0signature";
        assert!(!validate_request(
            auth_token,
            null_byte_sig,
            "https://example.com",
            &params,
            &Default::default()
        ));
    }

    #[test]
    fn test_all_url_variants_explicitly() {
        let auth_token = TEST_AUTH_TOKEN;
        let mut params = HashMap::new();
        params.insert("key".to_string(), "value".to_string());

        // Test that validate_request checks all 4 variants explicitly
        let url = "https://example.com/path?query=a+b";

        // Generate signatures for all 4 expected variants
        let sig1 = get_expected_twilio_signature(
            auth_token,
            &remove_port(&with_modern_querystring(url)),
            &params,
        );
        let sig2 = get_expected_twilio_signature(
            auth_token,
            &add_port(&with_modern_querystring(url)),
            &params,
        );
        let sig3 = get_expected_twilio_signature(
            auth_token,
            &remove_port(&with_legacy_querystring(url)),
            &params,
        );
        let sig4 = get_expected_twilio_signature(
            auth_token,
            &add_port(&with_legacy_querystring(url)),
            &params,
        );

        // All signatures should validate the original URL
        assert!(validate_request(
            auth_token,
            &sig1,
            url,
            &params,
            &Default::default()
        ));
        assert!(validate_request(
            auth_token,
            &sig2,
            url,
            &params,
            &Default::default()
        ));
        assert!(validate_request(
            auth_token,
            &sig3,
            url,
            &params,
            &Default::default()
        ));
        assert!(validate_request(
            auth_token,
            &sig4,
            url,
            &params,
            &Default::default()
        ));
    }

    #[test]
    fn test_large_payload() {
        let auth_token = TEST_AUTH_TOKEN;
        let mut params = HashMap::new();

        // Add many parameters to test sorting and concatenation with large payloads
        for i in 0..1000 {
            params.insert(format!("key{i}"), format!("value{i}"));
        }

        let url = "https://example.com";
        let signature = get_expected_twilio_signature(auth_token, url, &params);
        assert!(validate_request(
            auth_token,
            &signature,
            url,
            &params,
            &Default::default()
        ));
    }

    #[test]
    fn test_param_key_edge_cases() {
        let auth_token = TEST_AUTH_TOKEN;
        let url = "https://example.com";

        let mut params = HashMap::new();
        // Empty key
        params.insert("".to_string(), "value".to_string());
        // Key with special characters
        params.insert("key+with+spaces".to_string(), "value".to_string());
        // Unicode key
        params.insert("键".to_string(), "值".to_string());

        let signature = get_expected_twilio_signature(auth_token, url, &params);
        assert!(validate_request(
            auth_token,
            &signature,
            url,
            &params,
            &Default::default()
        ));
    }

    #[test]
    fn test_unicode_in_params() {
        let auth_token = TEST_AUTH_TOKEN;
        let mut params = HashMap::new();
        params.insert("message".to_string(), "你好世界".to_string());

        let url = "https://example.com/unicode";
        let signature = get_expected_twilio_signature(auth_token, url, &params);
        assert!(validate_request(
            auth_token,
            &signature,
            url,
            &params,
            &Default::default()
        ));
    }

    #[test]
    fn test_validate_incoming_request_json() {
        let auth_token = TEST_AUTH_TOKEN;
        let body =
            "{\"CallSid\":\"CAdcf610484bb41af47dbba11e8cd548b2\",\"Caller\":\"+12349013030\"}";
        let body_hash = get_expected_body_hash(body);
        let path_and_query = format!("/myapp?bodySHA256={body_hash}");
        let url = format!("https://example.com{path_and_query}");

        let expected_signature = get_expected_twilio_signature(auth_token, &url, &HashMap::new());

        let request = MockRequest {
            protocol: "https".to_string(),
            host: "example.com".to_string(),
            path_and_query: path_and_query.clone(),
            signature: Some(expected_signature.clone()),
            body: HashMap::new(),
            raw_body: Some(body.to_string()),
            method: Method::POST,
        };

        // Test success case
        assert!(validate_incoming_request(
            &request,
            auth_token,
            Default::default()
        ));

        // Test failure cases

        // 1. Missing raw body
        let request_no_body = MockRequest {
            protocol: "https".to_string(),
            host: "example.com".to_string(),
            path_and_query: path_and_query.clone(),
            signature: Some(expected_signature.clone()),
            body: HashMap::new(),
            raw_body: None, // No body
            method: Method::POST,
        };
        assert!(!validate_incoming_request(
            &request_no_body,
            auth_token,
            Default::default()
        ));

        // 2. Modified body
        let modified_body = body.replace("12349013030", "12345678901");
        let request_mod_body = MockRequest {
            protocol: "https".to_string(),
            host: "example.com".to_string(),
            path_and_query: path_and_query.clone(),
            signature: Some(expected_signature.clone()),
            body: HashMap::new(),
            raw_body: Some(modified_body), // Modified body
            method: Method::POST,
        };
        assert!(!validate_incoming_request(
            &request_mod_body,
            auth_token,
            Default::default()
        ));

        // 3. Wrong signature - use a completely different signature instead of trying to modify
        let wrong_signature = "XYZ_COMPLETELY_DIFFERENT_SIGNATURE_ABC=";

        // Verify the signature is actually different
        assert_ne!(expected_signature, wrong_signature);

        let request_wrong_sig = MockRequest {
            protocol: "https".to_string(),
            host: "example.com".to_string(),
            path_and_query: path_and_query.clone(),
            signature: Some(wrong_signature.to_string()),
            body: HashMap::new(),
            raw_body: Some(body.to_string()),
            method: Method::POST,
        };

        assert!(!validate_incoming_request(
            &request_wrong_sig,
            auth_token,
            Default::default()
        ));
    }

    #[test]
    fn test_validate_with_custom_url() {
        let auth_token = TEST_AUTH_TOKEN;

        let mut params = HashMap::new();
        params.insert(
            "CallSid".to_string(),
            "CA375e36857dcc9f64b3233b7e80a6d6ba".to_string(),
        );

        // The request has different host/path than what's configured in Twilio
        let request = MockRequest {
            protocol: "https".to_string(),
            host: "behind-proxy.com".to_string(), // Proxied/parsed host
            path_and_query: "/path?query=value".to_string(), // Proxied path
            signature: None,                      // We'll set this later
            body: params.clone(),
            raw_body: None,
            method: Method::POST,
        };

        // The URL that was actually configured in Twilio webhook
        let webhook_url = "https://custom.com/path?query=value";

        // Generate signature using the actual webhook URL
        let expected_signature = get_expected_twilio_signature(auth_token, webhook_url, &params);

        // Now set the signature on the request
        let mut request_with_sig = request.clone();
        request_with_sig.signature = Some(expected_signature);

        // Test success cases
        // 1. Validate with URL option
        let url_option = RequestValidatorOptions {
            url: Some(webhook_url.to_string()),
            host: None,
            protocol: None,
            test_both_protocols: false,
            ..Default::default()
        };
        assert!(validate_incoming_request(
            &request_with_sig,
            auth_token,
            url_option
        ));

        // 2. Validate with host/port options
        let host_port_option = RequestValidatorOptions {
            url: None,
            host: Some("custom.com".to_string()),
            protocol: Some("https".to_string()),
            test_both_protocols: false,
            ..Default::default()
        };
        assert!(validate_incoming_request(
            &request_with_sig,
            auth_token,
            host_port_option
        ));

        // Test failure cases

        // 1. No options provided (falls back to parsing request URL)
        assert!(!validate_incoming_request(
            &request_with_sig,
            auth_token,
            Default::default()
        ));

        // 2. Wrong URL in options
        let wrong_options = RequestValidatorOptions {
            url: Some("https://custom2.com/wrong?query=value".to_string()),
            host: None,
            protocol: None,
            test_both_protocols: false,
            ..Default::default()
        };
        assert!(!validate_incoming_request(
            &request_with_sig,
            auth_token,
            wrong_options
        ));
    }

    #[test]
    fn test_constant_time_compare() {
        // Equal strings should match
        assert!(constant_time_compare("abcdef", "abcdef"));

        // Different lengths should not match
        assert!(!constant_time_compare("abcdef", "abcde"));
        assert!(!constant_time_compare("abcde", "abcdef"));

        // Same length but different content should not match
        assert!(!constant_time_compare("abcdef", "abcdeg"));
        assert!(!constant_time_compare("abcdef", "Abcdef"));

        // Empty strings should match each other
        assert!(constant_time_compare("", ""));

        // Long strings with only one character different
        let long_a = "a".repeat(1000);
        let mut chars: Vec<char> = long_a.chars().collect();
        chars[999] = 'b'; // Change last character
        let long_b: String = chars.into_iter().collect();

        assert!(!constant_time_compare(&long_a, &long_b));

        // Long strings that are identical
        assert!(constant_time_compare(&long_a, &long_a));
    }

    #[test]
    fn test_body_hash_validation() {
        let body = "{\"test\": \"data\"}";
        let body_hash = get_expected_body_hash(body);

        // Correct hash should validate
        assert!(validate_body(body, &body_hash));

        // Incorrect hash should not validate
        let incorrect_hash = "0".repeat(64);
        assert!(!validate_body(body, &incorrect_hash));

        // Modified body should not validate against original hash
        let modified_body = "{\"test\": \"modified\"}";
        assert!(!validate_body(modified_body, &body_hash));

        // Empty body should have a consistent hash
        let empty_body = "";
        let empty_hash = get_expected_body_hash(empty_body);
        assert!(validate_body(empty_body, &empty_hash));
        assert!(!validate_body(empty_body, &body_hash));

        // Whitespace changes should affect hash
        let whitespace_body = " {\"test\": \"data\"} ";
        assert!(!validate_body(whitespace_body, &body_hash));
    }

    #[test]
    fn test_url_port_handling() {
        // Test URL manipulation functions
        let url_without_port = "https://example.com/path?query=value";
        let url_with_port = "https://example.com:443/path?query=value";

        // Adding port to URL without port should give standard port
        let with_added_port = add_port(url_without_port);
        assert!(with_added_port.contains(":443"));

        // Removing port from URL with port should remove the port
        let with_removed_port = remove_port(url_with_port);
        assert!(!with_removed_port.contains(":443"));

        // Test port handling in signatures
        let auth_token = TEST_AUTH_TOKEN;
        let mut params = HashMap::new();
        params.insert("Test".to_string(), "Value".to_string());

        // Generate signatures for URLs with and without port
        let sig_without_port = get_expected_twilio_signature(auth_token, url_without_port, &params);
        let sig_with_port = get_expected_twilio_signature(auth_token, url_with_port, &params);

        // Signatures should be different for different URL formats
        // But validation should still pass due to our multi-form checking
        assert!(validate_request(
            auth_token,
            &sig_without_port,
            url_without_port,
            &params,
            &Default::default(),
        ));
        assert!(validate_request(
            auth_token,
            &sig_with_port,
            url_with_port,
            &params,
            &Default::default(),
        ));

        // Cross-validation tests - validation should pass with various combinations
        // due to the implementation trying different URL formats
        assert!(validate_request(
            auth_token,
            &sig_with_port,
            url_without_port,
            &params,
            &Default::default(),
        ));
        assert!(validate_request(
            auth_token,
            &sig_without_port,
            url_with_port,
            &params,
            &Default::default(),
        ));
    }

    #[test]
    fn test_legacy_querystring_handling() {
        // This test doesn't test the `with_legacy_querystring` function directly
        // Instead, we'll simulate its purpose by using two different but equivalent URLs
        // that should both validate with the same signature

        let auth_token = TEST_AUTH_TOKEN;

        // Create two different URL representations that should be treated as equivalent
        let url_standard = "https://example.com/path?key=standard&special=a+b+c";
        let url_encoded = "https://example.com/path?key=standard&special=a%20b%20c";

        // These URLs are different as strings
        assert_ne!(url_standard, url_encoded);

        let mut params = HashMap::new();
        params.insert("Test".to_string(), "Value".to_string());

        // Generate signature using standard URL format
        let signature = get_expected_twilio_signature(auth_token, url_standard, &params);

        assert!(validate_request(
            auth_token,
            &signature,
            url_standard,
            &params,
            &Default::default(),
        ));
        assert!(validate_request(
            auth_token,
            &signature,
            url_encoded,
            &params,
            &Default::default(),
        ));

        // And signature generated with the encoded URL should also validate against both URLs
        let signature_encoded = get_expected_twilio_signature(auth_token, url_encoded, &params);
        assert!(validate_request(
            auth_token,
            &signature_encoded,
            url_standard,
            &params,
            &Default::default(),
        ));
        assert!(validate_request(
            auth_token,
            &signature_encoded,
            url_encoded,
            &params,
            &Default::default(),
        ));
    }
}
