use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

use crate::error::Error;

// Detects $json:value$
static JSON_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$json:([a-zA-Z0-9_.]+)\$").unwrap());

/// The different types of response parsers
///
/// # Types
/// * `Json` - Resolves the provided JSON path and uses the value as URL
/// * `Raw` - Uses the response body as URL
#[derive(Debug, PartialEq, Eq)]
pub enum UrlParseType {
    Json { path: String },
    Raw,
}

/// Converts the configured url parser statement into a parser type.
fn convert_statement(statement: &String) -> UrlParseType {
    if let Some(captures) = JSON_REGEX.captures(&statement) {
        let path = match captures.get(1) {
            Some(capture) => capture.as_str().to_string(),
            None => return UrlParseType::Raw,
        };
        return UrlParseType::Json { path };
    }

    UrlParseType::Raw
}

/// Parses the given response using the given parser string.
///
/// # Returns
/// The image url resulting from parsing
pub fn parse_url(response: &String, parser_stmt: &String) -> Result<String, Error> {
    let parser = convert_statement(parser_stmt);

    match parser {
        UrlParseType::Json { path } => {
            let root_value = serde_json::from_str::<Value>(&response)
                .map_err(|_| Error::InvalidResponse("Invalid json".into()))?;
            let target_value = path
                .split('.')
                .fold(Some(root_value), |current, key| current?.get(key).cloned())
                .ok_or(Error::InvalidResponse("Cannot find json value".into()))?;
            Ok(target_value
                .as_str()
                .ok_or(Error::InvalidResponse("Cannot parse json value".into()))?
                .to_string())
        }
        UrlParseType::Raw => Ok(response.to_string()),
    }
}

#[cfg(test)]
pub mod tests {
    use crate::parser::{parse_url, UrlParseType};

    use super::convert_statement;

    #[test]
    pub fn test_statement_converter() {
        assert_eq!(
            convert_statement(&"$json:url$".into()),
            UrlParseType::Json { path: "url".into() }
        );
        assert_eq!(convert_statement(&"$raw$".into()), UrlParseType::Raw);
    }

    #[test]
    pub fn test_parser() {
        assert_eq!(
            parse_url(&"{\"url\": \"hello\"}".into(), &"$json:url$".into()).unwrap_or_default(),
            "hello".to_string()
        );
    }
}
