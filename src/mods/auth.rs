
#[allow(unused_imports)]
use std::collections::HashMap;
use std::convert::From;
use std::error::Error;
use std::fmt;

use serde_json;

#[allow(unused_imports)]
use ToResult;
use requests::SlackWebRequestSender;

/// Revokes a token.
///
/// Wraps https://api.slack.com/methods/auth.revoke

pub fn revoke<R>(client: &R,
                 request: &RevokeRequest)
                 -> Result<RevokeResponse, RevokeError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![request.token.map(|token| ("token", token)),
                      request.test.map(|test| ("test", if test { "1" } else { "0" }))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("auth.revoke", &params[..])
        .map_err(|err| RevokeError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<RevokeResponse>(&result)
                .map_err(|_| RevokeError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct RevokeRequest<'a> {
    /// Authentication token.
    pub token: Option<&'a str>,
    /// Setting this parameter to 1 triggers a testing mode where the specified token will not actually be revoked.
    pub test: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RevokeResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub revoked: Option<bool>,
}


impl<E: Error> ToResult<RevokeResponse, RevokeError<E>> for RevokeResponse {
    fn to_result(self) -> Result<RevokeResponse, RevokeError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum RevokeError<E: Error> {
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse,
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for RevokeError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => RevokeError::NotAuthed,
            "invalid_auth" => RevokeError::InvalidAuth,
            "account_inactive" => RevokeError::AccountInactive,
            "invalid_arg_name" => RevokeError::InvalidArgName,
            "invalid_array_arg" => RevokeError::InvalidArrayArg,
            "invalid_charset" => RevokeError::InvalidCharset,
            "invalid_form_data" => RevokeError::InvalidFormData,
            "invalid_post_type" => RevokeError::InvalidPostType,
            "missing_post_type" => RevokeError::MissingPostType,
            "request_timeout" => RevokeError::RequestTimeout,
            _ => RevokeError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for RevokeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for RevokeError<E> {
    fn description(&self) -> &str {
        match self {
            &RevokeError::NotAuthed => "not_authed",
            &RevokeError::InvalidAuth => "invalid_auth",
            &RevokeError::AccountInactive => "account_inactive",
            &RevokeError::InvalidArgName => "invalid_arg_name",
            &RevokeError::InvalidArrayArg => "invalid_array_arg",
            &RevokeError::InvalidCharset => "invalid_charset",
            &RevokeError::InvalidFormData => "invalid_form_data",
            &RevokeError::InvalidPostType => "invalid_post_type",
            &RevokeError::MissingPostType => "missing_post_type",
            &RevokeError::RequestTimeout => "request_timeout",
            &RevokeError::MalformedResponse => "Malformed response data from Slack.",
            &RevokeError::Unknown(ref s) => s,
            &RevokeError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &RevokeError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}

/// Checks authentication & identity.
///
/// Wraps https://api.slack.com/methods/auth.test

pub fn test<R>(client: &R, request: &TestRequest) -> Result<TestResponse, TestError<R::Error>>
    where R: SlackWebRequestSender
{

    let params = vec![Some(("token", request.token))];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    client.send("auth.test", &params[..])
        .map_err(|err| TestError::Client(err))
        .and_then(|result| {
            serde_json::from_str::<TestResponse>(&result).map_err(|_| TestError::MalformedResponse)
        })
        .and_then(|o| o.to_result())
}

#[derive(Clone, Default, Debug)]
pub struct TestRequest<'a> {
    /// Authentication token.
    /// Requires scope: identify
    pub token: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TestResponse {
    error: Option<String>,
    #[serde(default)]
    ok: bool,
    pub team: Option<String>,
    pub team_id: Option<String>,
    pub url: Option<String>,
    pub user: Option<String>,
    pub user_id: Option<String>,
}


impl<E: Error> ToResult<TestResponse, TestError<E>> for TestResponse {
    fn to_result(self) -> Result<TestResponse, TestError<E>> {
        if self.ok {
            Ok(self)
        } else {
            Err(self.error.as_ref().map(String::as_ref).unwrap_or("").into())
        }
    }
}
#[derive(Clone, Debug)]
pub enum TestError<E: Error> {
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/json application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    MalformedResponse,
    /// The response returned an error that was unknown to the library
    Unknown(String),
    /// The client had an error sending the request to Slack
    Client(E),
}

impl<'a, E: Error> From<&'a str> for TestError<E> {
    fn from(s: &'a str) -> Self {
        match s {
            "not_authed" => TestError::NotAuthed,
            "invalid_auth" => TestError::InvalidAuth,
            "account_inactive" => TestError::AccountInactive,
            "invalid_arg_name" => TestError::InvalidArgName,
            "invalid_array_arg" => TestError::InvalidArrayArg,
            "invalid_charset" => TestError::InvalidCharset,
            "invalid_form_data" => TestError::InvalidFormData,
            "invalid_post_type" => TestError::InvalidPostType,
            "missing_post_type" => TestError::MissingPostType,
            "request_timeout" => TestError::RequestTimeout,
            _ => TestError::Unknown(s.to_owned()),
        }
    }
}

impl<E: Error> fmt::Display for TestError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error> Error for TestError<E> {
    fn description(&self) -> &str {
        match self {
            &TestError::NotAuthed => "not_authed",
            &TestError::InvalidAuth => "invalid_auth",
            &TestError::AccountInactive => "account_inactive",
            &TestError::InvalidArgName => "invalid_arg_name",
            &TestError::InvalidArrayArg => "invalid_array_arg",
            &TestError::InvalidCharset => "invalid_charset",
            &TestError::InvalidFormData => "invalid_form_data",
            &TestError::InvalidPostType => "invalid_post_type",
            &TestError::MissingPostType => "missing_post_type",
            &TestError::RequestTimeout => "request_timeout",
            &TestError::MalformedResponse => "Malformed response data from Slack.",
            &TestError::Unknown(ref s) => s,
            &TestError::Client(ref inner) => inner.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &TestError::Client(ref inner) => Some(inner),
            _ => None,
        }
    }
}
