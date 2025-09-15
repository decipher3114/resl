use resl::Value as ReslValue;
use toml::Value as TomlValue;

pub(crate) fn toml_to_resl(value: TomlValue) -> ReslValue {
    match value {
        TomlValue::String(s) => ReslValue::String(s),
        TomlValue::Integer(i) => ReslValue::Integer(i),
        TomlValue::Float(f) => ReslValue::Float(f),
        TomlValue::Boolean(b) => ReslValue::Boolean(b),
        TomlValue::Datetime(dt) => ReslValue::String(dt.to_string()),
        TomlValue::Array(arr) => ReslValue::List(arr.into_iter().map(toml_to_resl).collect()),
        TomlValue::Table(table) => ReslValue::Map(
            table
                .into_iter()
                .map(|(k, v)| (k, toml_to_resl(v)))
                .collect(),
        ),
    }
}

pub(crate) fn resl_to_toml(value: ReslValue) -> TomlValue {
    match value {
        ReslValue::Null => TomlValue::String(String::new()),
        ReslValue::String(s) => TomlValue::String(s),
        ReslValue::Integer(i) => TomlValue::Integer(i),
        ReslValue::Float(f) => TomlValue::Float(f),
        ReslValue::Boolean(b) => TomlValue::Boolean(b),
        ReslValue::List(list) => TomlValue::Array(list.into_iter().map(resl_to_toml).collect()),
        ReslValue::Map(map) => {
            TomlValue::Table(map.into_iter().map(|(k, v)| (k, resl_to_toml(v))).collect())
        }
    }
}
