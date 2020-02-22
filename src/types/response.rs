use crate::types::primitive::Integer;
use serde::{de::Error, Deserialize, Deserializer};
use std::{error::Error as StdError, fmt};

/// API Response
#[derive(Clone, Debug)]
pub enum Response<T> {
    /// Success
    Success(T),
    /// Error
    Error(ResponseError),
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Response<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawResponse<T> = Deserialize::deserialize(deserializer)?;
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        };
        Ok(if raw.ok {
            Response::Success(required!(result))
        } else {
            Response::Error(ResponseError {
                description: required!(description),
                error_code: raw.error_code,
                parameters: raw.parameters,
            })
        })
    }
}

/// Response error
#[derive(Clone, Debug)]
pub struct ResponseError {
    /// Human-readable description
    pub description: String,
    /// Error code
    pub error_code: Option<Integer>,
    /// Parameters
    pub parameters: Option<ResponseParameters>,
}

impl ResponseError {
    /// Whether request can be repeated
    pub fn can_retry(&self) -> bool {
        self.parameters.map(|x| x.retry_after.is_some()).unwrap_or(false)
    }
}

impl StdError for ResponseError {}

impl fmt::Display for ResponseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "a telegram error has occurred: description={}", self.description)?;
        if let Some(code) = self.error_code {
            write!(out, "; error_code={}", code)?;
        }
        if let Some(parameters) = self.parameters {
            if let Some(chat_id) = parameters.migrate_to_chat_id {
                write!(out, "; migrate_to_chat_id={}", chat_id)?;
            }
            if let Some(retry_after) = parameters.retry_after {
                write!(out, "; retry_after={}", retry_after)?;
            }
        }
        Ok(())
    }
}

/// Contains information about why a request was unsuccessful
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier
    pub migrate_to_chat_id: Option<Integer>,
    /// In case of exceeding flood control,
    /// the number of seconds left to wait
    /// before the request can be repeated
    pub retry_after: Option<Integer>,
}

#[derive(Clone, Debug, Deserialize)]
struct RawResponse<T> {
    ok: bool,
    description: Option<String>,
    error_code: Option<Integer>,
    result: Option<T>,
    parameters: Option<ResponseParameters>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Clone, Debug, Deserialize)]
    struct Object {
        name: String,
    }

    #[test]
    fn deserialize() {
        let success: Response<Object> = serde_json::from_value(json!({
            "ok": true,
            "result": {"name": "test" }
        }))
        .unwrap();

        if let Response::Success(ref obj) = success {
            assert_eq!(obj.name, String::from("test"));
        } else {
            panic!("Unexpected response: {:?}", success);
        }

        let error: Response<Object> = serde_json::from_value(json!({
            "ok": false,
            "description": "test err",
            "error_code": 1,
            "parameters": {
                "migrate_to_chat_id": 2,
                "retry_after": 3
            }
        }))
        .unwrap();
        if let Response::Error(err) = error {
            assert_eq!(err.description, String::from("test err"));
            assert_eq!(err.error_code.unwrap(), 1);
            assert!(err.can_retry());
            let params = err.parameters.unwrap();
            assert_eq!(params.migrate_to_chat_id.unwrap(), 2);
            assert_eq!(params.retry_after.unwrap(), 3);
        } else {
            panic!("Unexpected response: {:?}", success);
        }

        let error: Response<Object> = serde_json::from_value(json!({
            "ok": false,
            "description": "test err"
        }))
        .unwrap();
        if let Response::Error(err) = error {
            assert_eq!(err.description, String::from("test err"));
            assert!(!err.can_retry());
            assert!(err.error_code.is_none());
            assert!(err.parameters.is_none());
        } else {
            panic!("Unexpected response: {:?}", success);
        }
    }
}
