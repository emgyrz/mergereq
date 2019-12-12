use percent_encoding::{percent_encode, PercentEncode, NON_ALPHANUMERIC};

pub fn encode(s: &str) -> PercentEncode {
  percent_encode(s.as_bytes(), NON_ALPHANUMERIC)
}
