#ifndef RESL_H
#define RESL_H

/* This file is auto-generated with cbindgen. Do not edit manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Identifies the type of a RESL value.
 */
typedef enum ReslTag {
  /**
   * < Null value
   */
  Null = 0,
  /**
   * < String value
   */
  String = 1,
  /**
   * < Integer value
   */
  Integer = 2,
  /**
   * < Float value
   */
  Float = 3,
  /**
   * < Boolean value
   */
  Boolean = 4,
  /**
   * < List of ReslValues
   */
  List = 5,
  /**
   * < Map of string keys to ReslValues
   */
  Map = 6,
} ReslTag;

/**
 * Represents a UTF-8 string as pointer + length.
 */
typedef struct ReslString {
  /**
   * < Pointer to null-terminated C string
   */
  char *ptr;
  /**
   * < Length of string in bytes
   */
  uintptr_t len;
} ReslString;

/**
 * Represents a list (array of pointers to `ReslValue`).
 */
typedef struct ReslList {
  /**
   * < Array of pointers to ReslValues
   */
  struct ReslValue **items;
  /**
   * < Number of items in the list
   */
  uintptr_t len;
} ReslList;

/**
 * Represents one key-value pair inside a map.
 */
typedef struct ReslMapEntry {
  /**
   * < Key as a ReslString
   */
  struct ReslString key;
  /**
   * < Pointer to the corresponding ReslValue
   */
  struct ReslValue *value;
} ReslMapEntry;

/**
 * Represents a map (array of key-value pairs).
 */
typedef struct ReslMap {
  /**
   * < Array of map entries
   */
  struct ReslMapEntry *entries;
  /**
   * < Number of entries in the map
   */
  uintptr_t len;
} ReslMap;

/**
 * Holds the actual data for a RESL value.
 * Which field is valid depends on `tag`.
 */
typedef union ReslPayload {
  /**
   * < String payload
   */
  struct ReslString string;
  /**
   * < Integer payload
   */
  int64_t integer;
  /**
   * < Float payload
   */
  double _float;
  /**
   * < Boolean payload
   */
  bool boolean;
  /**
   * < List payload
   */
  struct ReslList list;
  /**
   * < Map payload
   */
  struct ReslMap map;
} ReslPayload;

/**
 * Represents a RESL value (tagged union).
 */
typedef struct ReslValue {
  /**
   * < Tag indicating type of value
   */
  enum ReslTag tag;
  /**
   * < Payload holding the actual data
   */
  union ReslPayload payload;
} ReslValue;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Frees a `ReslString` allocated by the library.
 * @param s ReslString to free.
 */
void resl_string_free(struct ReslString s);

/**
 * Frees a `ReslValue` and all its children recursively.
 * @param val Pointer to `ReslValue` to free.
 * @note After calling, `val` must not be used again.
 * @warning Only free pointers returned by this library.
 */
void resl_value_free(struct ReslValue *val);

/**
 * Formats a RESL expression string.
 * @param input Null-terminated C string containing expression.
 * @param pretty Whether to pretty-print output.
 * @return ReslString allocated on heap. Must be freed with `resl_string_free`.
 */
struct ReslString resl_format(const char *input, bool pretty);

/**
 * Evaluates a RESL expression string.
 * @param input Null-terminated C string containing expression.
 * @return Pointer to heap-allocated `ReslValue`. Must be freed with `resl_value_free`.
 */
struct ReslValue *resl_evaluate(const char *input);

/**
 * Evaluates a RESL expression string and formats it.
 * @param input Null-terminated C string containing expression.
 * @param pretty Whether to pretty-print output.
 * @return ReslString allocated on heap. Must be freed with `resl_string_free`.
 */
struct ReslString resl_evaluate_and_format(const char *input, bool pretty);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* RESL_H */
