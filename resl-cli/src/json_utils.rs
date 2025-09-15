use resl::Value as ReslValue;
use serde_json::Value as JsonValue;

pub(crate) fn json_to_resl(json_value: JsonValue) -> ReslValue {
    match json_value {
        JsonValue::Null => ReslValue::Null,
        JsonValue::Bool(b) => ReslValue::Boolean(b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                ReslValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                ReslValue::Float(f)
            } else {
                ReslValue::Null
            }
        }
        JsonValue::String(s) => ReslValue::String(s),
        JsonValue::Array(arr) => ReslValue::List(arr.into_iter().map(json_to_resl).collect()),
        JsonValue::Object(obj) => {
            ReslValue::Map(obj.into_iter().map(|(k, v)| (k, json_to_resl(v))).collect())
        }
    }
}

pub(crate) fn resl_to_json(resl_value: ReslValue) -> JsonValue {
    match resl_value {
        ReslValue::Null => JsonValue::Null,
        ReslValue::Boolean(b) => JsonValue::Bool(b),
        ReslValue::Integer(i) => JsonValue::Number(serde_json::Number::from(i)),
        ReslValue::Float(f) => {
            JsonValue::Number(serde_json::Number::from_f64(f).expect("Conversion should be valid"))
        }
        ReslValue::String(s) => JsonValue::String(s.to_owned()),

        ReslValue::List(list) => JsonValue::Array(
            list.iter()
                .map(|expr| resl_to_json(expr.to_owned()))
                .collect(),
        ),
        ReslValue::Map(map) => JsonValue::Object(
            map.iter()
                .map(|(k, v)| (k.to_owned(), resl_to_json(v.to_owned())))
                .collect(),
        ),
    }
}
