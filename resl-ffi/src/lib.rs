//! # RESL FFI
//!
//! C FFI bindings for RESL - Runtime Evaluated Serialization Language.
//!
//! Exposes RESL's `Value` and `evaluate` functionality to C via FFI.
//! Provides C-compatible structures (tagged union) to represent RESL values.RESL FFI

use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    os::raw::c_char,
};

use resl::{Value, evaluate, format};

/// Identifies the type of a RESL value.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReslTag {
    /// < Null value
    Null = 0,
    /// < String value
    String = 1,
    /// < Integer value
    Integer = 2,
    /// < Float value
    Float = 3,
    /// < Boolean value
    Boolean = 4,
    /// < List of ReslValues
    List = 5,
    /// < Map of string keys to ReslValues
    Map = 6,
}

/// Represents a UTF-8 string as pointer + length.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReslString {
    /// < Pointer to null-terminated C string
    pub ptr: *mut c_char,
    /// < Length of string in bytes
    pub len: usize,
}

/// Represents a list (array of pointers to `ReslValue`).
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReslList {
    /// < Array of pointers to ReslValues
    pub items: *mut *mut ReslValue,
    /// < Number of items in the list
    pub len: usize,
}

/// Represents one key-value pair inside a map.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReslMapEntry {
    /// < Key as a ReslString
    pub key: ReslString,
    /// < Pointer to the corresponding ReslValue
    pub value: *mut ReslValue,
}

/// Represents a map (array of key-value pairs).
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReslMap {
    /// < Array of map entries
    pub entries: *mut ReslMapEntry,
    /// < Number of entries in the map
    pub len: usize,
}

/// Holds the actual data for a RESL value.
/// Which field is valid depends on `tag`.
#[repr(C)]
pub union ReslPayload {
    /// < String payload
    pub string: ManuallyDrop<ReslString>,
    /// < Integer payload
    pub integer: i64,
    /// < Float payload
    pub _float: f64,
    /// < Boolean payload
    pub boolean: bool,
    /// < List payload
    pub list: ManuallyDrop<ReslList>,
    /// < Map payload
    pub map: ManuallyDrop<ReslMap>,
}

/// Represents a RESL value (tagged union).
#[repr(C)]
pub struct ReslValue {
    /// < Tag indicating type of value
    pub tag: ReslTag,
    /// < Payload holding the actual data
    pub payload: ReslPayload,
}

/// Converts a Rust `&str` into a `ReslString`.
/// @param s Rust string slice to convert.
/// @return ReslString allocated on the heap. Must be freed by caller.
fn to_resl_string(s: &str) -> ReslString {
    let cstr = CString::new(s).unwrap();
    let len = cstr.as_bytes().len();
    let ptr = cstr.into_raw();
    ReslString { ptr, len }
}

/// Converts a RESL `Value` into a heap-allocated `ReslValue`.
/// @param val Reference to a Rust `Value`.
/// @return Pointer to heap-allocated `ReslValue`.
fn to_resl_value(val: &Value) -> *mut ReslValue {
    let boxed = match val {
        Value::Null => Box::new(ReslValue {
            tag: ReslTag::Null,
            payload: unsafe { std::mem::zeroed() },
        }),
        Value::String(s) => Box::new(ReslValue {
            tag: ReslTag::String,
            payload: ReslPayload {
                string: ManuallyDrop::new(to_resl_string(s)),
            },
        }),
        Value::Integer(i) => Box::new(ReslValue {
            tag: ReslTag::Integer,
            payload: ReslPayload { integer: *i },
        }),
        Value::Float(f) => Box::new(ReslValue {
            tag: ReslTag::Float,
            payload: ReslPayload { _float: *f },
        }),
        Value::Boolean(b) => Box::new(ReslValue {
            tag: ReslTag::Boolean,
            payload: ReslPayload { boolean: *b },
        }),
        Value::List(vec) => {
            let mut items: Vec<*mut ReslValue> = vec.iter().map(to_resl_value).collect();
            let ptr = items.as_mut_ptr();
            let len = items.len();
            std::mem::forget(items);
            Box::new(ReslValue {
                tag: ReslTag::List,
                payload: ReslPayload {
                    list: ManuallyDrop::new(ReslList { items: ptr, len }),
                },
            })
        }
        Value::Map(vec) => {
            let mut entries: Vec<ReslMapEntry> = vec
                .iter()
                .map(|(k, v)| ReslMapEntry {
                    key: to_resl_string(k),
                    value: to_resl_value(v),
                })
                .collect();
            let ptr = entries.as_mut_ptr();
            let len = entries.len();
            std::mem::forget(entries);
            Box::new(ReslValue {
                tag: ReslTag::Map,
                payload: ReslPayload {
                    map: ManuallyDrop::new(ReslMap { entries: ptr, len }),
                },
            })
        }
    };
    Box::into_raw(boxed)
}

/// Frees a `ReslString` allocated by the library.
/// @param s ReslString to free.
#[unsafe(no_mangle)]
pub extern "C" fn resl_string_free(s: ReslString) {
    if !s.ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(s.ptr);
        }
    }
}

/// Frees a `ReslValue` and all its children recursively.
/// @param val Pointer to `ReslValue` to free.
/// @note After calling, `val` must not be used again.
/// @warning Only free pointers returned by this library.
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resl_value_free(val: *mut ReslValue) {
    if val.is_null() {
        return;
    }
    let val = unsafe { Box::from_raw(val) };
    unsafe {
        match val.tag {
            ReslTag::String => {
                let s = ManuallyDrop::into_inner(val.payload.string);
                resl_string_free(s);
            }
            ReslTag::List => {
                let list = val.payload.list;
                for i in 0..list.len {
                    let item = *list.items.add(i);
                    resl_value_free(item);
                }
                Vec::from_raw_parts(list.items, list.len, list.len);
            }
            ReslTag::Map => {
                let map = ManuallyDrop::into_inner(val.payload.map);
                for i in 0..map.len {
                    let entry = &*map.entries.add(i);
                    if !entry.key.ptr.is_null() {
                        let _ = CString::from_raw(entry.key.ptr);
                    }
                    resl_value_free(entry.value);
                }
                Vec::from_raw_parts(map.entries, map.len, map.len);
            }
            _ => {}
        }
    }
}

/// Formats a RESL expression string.
/// @param input Null-terminated C string containing expression.
/// @param pretty Whether to pretty-print output.
/// @return ReslString allocated on heap. Must be freed with `resl_string_free`.
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resl_format(input: *const c_char, pretty: bool) -> ReslString {
    if input.is_null() {
        return ReslString {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
    }

    let cstr = unsafe { CStr::from_ptr(input) };
    let expr = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => {
            return ReslString {
                ptr: std::ptr::null_mut(),
                len: 0,
            };
        }
    };

    let mut output = String::new();
    match format(expr, &mut output, pretty) {
        Ok(_) => to_resl_string(&output),
        Err(_) => ReslString {
            ptr: std::ptr::null_mut(),
            len: 0,
        },
    }
}

/// Evaluates a RESL expression string.
/// @param input Null-terminated C string containing expression.
/// @return Pointer to heap-allocated `ReslValue`. Must be freed with `resl_value_free`.
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resl_evaluate(input: *const c_char) -> *mut ReslValue {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let cstr = unsafe { CStr::from_ptr(input) };
    let expr = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let value: Value = match evaluate(expr) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    to_resl_value(&value)
}

/// Evaluates a RESL expression string and formats it.
/// @param input Null-terminated C string containing expression.
/// @param pretty Whether to pretty-print output.
/// @return ReslString allocated on heap. Must be freed with `resl_string_free`.
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resl_evaluate_and_format(
    input: *const c_char,
    pretty: bool,
) -> ReslString {
    if input.is_null() {
        return ReslString {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
    }
    let cstr = unsafe { CStr::from_ptr(input) };
    let expr = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => {
            return ReslString {
                ptr: std::ptr::null_mut(),
                len: 0,
            };
        }
    };
    let mut buf = String::new();
    match resl::evaluate_and_format(expr, &mut buf, pretty) {
        Ok(_) => to_resl_string(&buf),
        Err(_) => ReslString {
            ptr: std::ptr::null_mut(),
            len: 0,
        },
    }
}

#[cfg(test)]
mod ffi_tests {
    use std::ffi::CString;

    use super::*;

    const INPUT: &str = r#"[1+2,concat("Hello"," ","World!"),true]"#;

    #[test]
    fn test_resl_format() {
        let expr = CString::new(INPUT).unwrap();
        let s = unsafe { resl_format(expr.as_ptr(), true) };
        assert!(!s.ptr.is_null());
        let formatted = unsafe { std::ffi::CStr::from_ptr(s.ptr) }
            .to_str()
            .expect("Invalid UTF-8 string");
        assert_eq!(
            formatted,
            r#"[
    1 + 2,
    concat("Hello", " ", "World!"),
    true
]"#
        );
        resl_string_free(s);
    }

    #[test]
    fn test_resl_evaluate() {
        let expr = CString::new(INPUT).unwrap();
        let val_ptr = unsafe { resl_evaluate(expr.as_ptr()) };
        assert!(!val_ptr.is_null());
        let val = unsafe { &*val_ptr };
        match val.tag {
            ReslTag::List => {
                let list = unsafe { &val.payload.list };
                assert_eq!(list.len, 3);
                let first_ptr = unsafe { *list.items.add(0) };
                let first = unsafe { &*first_ptr };
                assert_eq!(first.tag, ReslTag::Integer);
                let first_int = unsafe { first.payload.integer };
                assert_eq!(first_int, 3);

                let second_ptr = unsafe { *list.items.add(1) };
                let second = unsafe { &*second_ptr };
                assert_eq!(second.tag, ReslTag::String);
                let second_str = unsafe { &second.payload.string };
                let second_cstr = unsafe { CStr::from_ptr(second_str.ptr) }
                    .to_str()
                    .expect("Invalid UTF-8 string");
                assert_eq!(second_cstr, "Hello World!");

                let third_ptr = unsafe { *list.items.add(2) };
                let third = unsafe { &*third_ptr };
                assert_eq!(third.tag, ReslTag::Boolean);
                let third_bool = unsafe { third.payload.boolean };
                assert!(third_bool);
            }
            _ => panic!("Expected list result"),
        }
        unsafe { resl_value_free(val_ptr) };
    }

    #[test]
    fn test_resl_evaluate_and_format() {
        let expr = CString::new(INPUT).unwrap();
        let s = unsafe { resl_evaluate_and_format(expr.as_ptr(), true) };
        assert!(!s.ptr.is_null());
        let formatted = unsafe { std::ffi::CStr::from_ptr(s.ptr) }
            .to_str()
            .expect("Invalid UTF-8 string");
        assert_eq!(
            formatted,
            r#"[
    3,
    "Hello World!",
    true
]"#
        );
    }
}
