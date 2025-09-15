use crate::{expr::Expr, function::Fn, state::EvalState, value::Value};

pub(crate) const BUILTIN_FUNCTIONS: [(&str, Fn); 7] = [
    ("debug", Fn::BuiltIn(debug)),
    ("type_of", Fn::BuiltIn(type_of)),
    ("length", Fn::BuiltIn(length)),
    ("to_str", Fn::BuiltIn(to_str)),
    ("concat", Fn::BuiltIn(concat)),
    ("push", Fn::BuiltIn(push)),
    ("insert", Fn::BuiltIn(insert)),
];

pub(crate) fn debug(state: &mut EvalState, args: Vec<Expr>) -> Value {
    if args.len() != 1 {
        return Value::Null;
    }

    let value = args[0].to_owned().evaluate(state);
    println!("{}", value);
    value
}

pub(crate) fn type_of(state: &mut EvalState, args: Vec<Expr>) -> Value {
    if args.len() != 1 {
        return Value::Null;
    }

    let arg = args[0].to_owned().evaluate(state);

    let type_str = match arg {
        Value::Null => "null",
        Value::Boolean(_) => "boolean",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::List(_) => "list",
        Value::Map(_) => "map",
    };

    Value::String(type_str.to_string())
}

pub(crate) fn length(state: &mut EvalState, args: Vec<Expr>) -> Value {
    if args.len() != 1 {
        return Value::Null;
    }

    let arg = args[0].to_owned().evaluate(state);

    match arg {
        Value::String(s) => Value::Integer(s.chars().count() as i64),
        Value::List(arr) => Value::Integer(arr.len() as i64),
        Value::Map(map) => Value::Integer(map.len() as i64),
        _ => Value::Null,
    }
}

pub(crate) fn to_str(state: &mut EvalState, args: Vec<Expr>) -> Value {
    if args.len() != 1 {
        return Value::Null;
    }

    let arg = args[0].to_owned().evaluate(state);
    Value::String(arg.to_string())
}

pub(crate) fn concat(state: &mut EvalState, args: Vec<Expr>) -> Value {
    let mut string = String::new();

    for arg in args {
        if let Value::String(str) = arg.evaluate(state) {
            string.push_str(&str)
        }
    }

    if string.is_empty() {
        return Value::Null;
    }

    Value::String(string)
}

pub(crate) fn push(state: &mut EvalState, args: Vec<Expr>) -> Value {
    if args.len() != 2 {
        return Value::Null;
    }

    let collection = args[0].to_owned().evaluate(state);
    let value = args[1].to_owned().evaluate(state);

    match collection {
        Value::List(mut arr) => {
            arr.push(value);
            Value::List(arr)
        }
        _ => Value::Null,
    }
}

pub(crate) fn insert(state: &mut EvalState, args: Vec<Expr>) -> Value {
    if args.len() != 3 {
        return Value::Null;
    }

    let collection = args[0].to_owned().evaluate(state);
    let key = args[1].to_owned().evaluate(state);
    let value = args[2].to_owned().evaluate(state);

    match collection {
        Value::Map(mut map) => {
            if let Value::String(key_str) = key {
                map.insert(key_str, value);
                Value::Map(map)
            } else {
                Value::Null
            }
        }
        Value::List(mut arr) => {
            if let Value::Integer(index) = key {
                let idx = if index < 0 {
                    (arr.len() as i64 + index) as usize
                } else {
                    index as usize
                };
                if idx <= arr.len() {
                    arr.insert(idx, value);
                    return Value::List(arr);
                }
            }
            Value::Null
        }
        _ => Value::Null,
    }
}
