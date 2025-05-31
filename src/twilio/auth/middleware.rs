use axum::{body::Body, extract::Request, http::StatusCode, response::Response};
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{Method, Uri, request::Parts};
use http_body_util::BodyExt;
use std::collections::HashMap;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use url::form_urlencoded;

use super::TwilioRequest;

#[derive(Clone)]
pub struct TwilioLayer {
    auth_token: String,
    options: Option<super::RequestValidatorOptions>,
}

impl TwilioLayer {
    pub fn new(auth_token: &str) -> Self {
        Self {
            auth_token: auth_token.to_string(),
            options: None,
        }
    }

    pub fn with_options(auth_token: &str, options: super::RequestValidatorOptions) -> Self {
        Self {
            auth_token: auth_token.to_string(),
            options: Some(options),
        }
    }
}

impl<S> Layer<S> for TwilioLayer {
    type Service = TwilioMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TwilioMiddleware {
            inner,
            auth_token: self.auth_token.clone(),
            options: self.options.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TwilioMiddleware<S> {
    inner: S,
    auth_token: String,
    options: Option<super::RequestValidatorOptions>,
}

impl<S> Service<Request> for TwilioMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let auth_token = self.auth_token.clone();
        let options = self.options.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract request parts and original URI
            let (parts, body) = request.into_parts();
            let original_uri = parts.uri.clone();

            // Collect the body
            let body_bytes = body
                .collect()
                .await
                .map_err(|_| ())
                .unwrap_or_default()
                .to_bytes();

            // Create a TwilioRequest implementation
            let twilio_request = TwilioRequestImpl {
                original_uri,
                parts: parts.clone(),
                body_bytes: body_bytes.clone(),
            };

            // Validate the request

            let is_valid = super::validate_incoming_request(&twilio_request, &auth_token, options);

            if !is_valid {
                return Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(if cfg!(debug_assertions) {
                        Body::from("Invalid Twilio signature")
                    } else {
                        Body::empty()
                    })
                    .unwrap());
            }

            // Create a new request with the original body
            let restored_request = Request::from_parts(parts, Body::from(body_bytes));

            let future = inner.call(restored_request);
            future.await
        })
    }
}

// Implementation of TwilioRequest trait using OriginalUri
struct TwilioRequestImpl {
    original_uri: Uri,
    parts: Parts,
    body_bytes: Bytes,
}

impl TwilioRequest for TwilioRequestImpl {
    fn protocol(&self) -> String {
        self.original_uri
            .scheme_str()
            .unwrap_or("http") // Default to http if scheme is missing
            .to_string()
    }

    fn host(&self) -> String {
        self.original_uri
            .authority()
            .map(|auth| auth.to_string())
            .unwrap_or_else(|| {
                // Fallback to Host header only if absolutely necessary
                self.parts
                    .headers
                    .get("host")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or_default()
                    .to_string()
            })
    }

    fn path_and_query(&self) -> String {
        format!(
            "{path}{query}",
            path = self.original_uri.path(),
            query = self
                .original_uri
                .query()
                .map(|q| format!("?{q}"))
                .unwrap_or_default()
        )
    }

    fn twilio_signature(&self) -> Option<String> {
        self.parts
            .headers
            .get("x-twilio-signature")
            .and_then(|h| h.to_str().ok())
            .map(String::from)
    }

    fn body(&self) -> HashMap<String, String> {
        form_urlencoded::parse(&self.body_bytes)
            .into_owned()
            .collect()
    }

    fn raw_body(&self) -> Option<String> {
        std::str::from_utf8(&self.body_bytes).ok().map(String::from)
    }

    fn method(&self) -> Method {
        self.parts.method.clone()
    }
}
