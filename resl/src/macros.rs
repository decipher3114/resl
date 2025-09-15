macro_rules! label {
    ($s:expr) => {{
        let _: &str = $s;
        winnow::error::StrContext::Label($s)
    }};
}

macro_rules! exp_char {
    ($ch:expr) => {{
        let _: char = $ch;
        winnow::error::StrContext::Expected(winnow::error::StrContextValue::CharLiteral($ch))
    }};
}

macro_rules! exp_str {
    ($s:expr) => {{
        let _: &str = $s;
        winnow::error::StrContext::Expected(winnow::error::StrContextValue::StringLiteral($s))
    }};
}

macro_rules! exp_desc {
    ($desc:expr) => {{
        let _: &str = $desc;
        winnow::error::StrContext::Expected(winnow::error::StrContextValue::Description($desc))
    }};
}

pub(crate) use exp_char;
pub(crate) use exp_desc;
pub(crate) use exp_str;
pub(crate) use label;
