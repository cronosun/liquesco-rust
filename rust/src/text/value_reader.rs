use crate::text::read_error::ReadError;
use crate::text::value::SrcPosition;
use crate::text::value::TextValue;
use crate::text::value::Value;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

const NAME_TAG: &'static str = "#name";

pub trait ValueReader {
    fn value<'a>(value: &'a TextValue<'a>) -> &'a Value<'a> {
        value.value.borrow()
    }

    fn position<'a>(value: &'a TextValue<'a>) -> Option<&'a SrcPosition> {
        value.position.as_ref()
    }

    fn name<'a>(value: &'a TextValue<'a>) -> Option<&'a str> {
        value.name.as_ref().map(|item| item.borrow())
    }

    fn name_tag() -> Option<&'static str> {
        Option::Some(NAME_TAG)
    }

    fn require_string(&self) -> Result<&str, ReadError> {
        unimplemented!()
    }

    /// The name of the current element. Since not all text formats support naming
    /// elements, the name can come from (depending on the implementation):
    ///
    /// - 1. This is a named element
    /// - 2. This is a string map and has a name tag.
    fn read_name<'a>(value: &'a TextValue<'a>) -> Option<&'a str> {
        if let Some(name) = Self::name(value) {
            Option::Some(name)
        } else {
            if let Some(name_tag) = Self::name_tag() {
                if let Some(string_map) = Self::read_string_map(value) {
                    if let Some(value) = string_map.get(name_tag) {
                        if let Some(name) = Self::read_string(value) {
                            Option::Some(name)
                        } else {
                            Option::None
                        }
                    } else {
                        Option::None
                    }
                } else {
                    Option::None
                }
            } else {
                Option::None
            }
        }
    }

    fn read_string<'a>(value: &'a TextValue<'a>) -> Option<&'a str> {
        if let Value::String(string) = Self::value(value) {
            Option::Some(string)
        } else {
            Option::None
        }
    }

    /// This returns a map if:
    /// - a) this is a map.
    /// - b) This is a sequence
    ///   - All elements in this sequence are sequences with 2 elements (entry).
    ///   - The first element in this entry must be a string.
    fn read_string_map<'a>(
        value: &'a TextValue<'a>,
    ) -> Option<Cow<'a, HashMap<Cow<'a, str>, TextValue<'a>>>> {
        let inner_value = Self::value(value);
        match inner_value {
            Value::Map(map) => Option::Some(Cow::Borrowed(map)),
            Value::Seq(seq) => {
                // could also be a sequence
                let mut result: HashMap<Cow<'a, str>, TextValue<'a>> =
                    HashMap::with_capacity(seq.len());
                for src_element in seq.iter() {
                    // must be a sequence
                    let maybe_entry_seq: Option<Cow<'a, Vec<TextValue<'a>>>> =
                        Self::read_seq(src_element);
                    match maybe_entry_seq.clone() {
                        Option::Some(entry_seq) => {
                            let entry_len = entry_seq.len();
                            if entry_len != 2 {
                                // first thing must be a string
                                let key_value: &TextValue<'a> = &(&entry_seq)[0];
                                if let Value::String(value) = key_value.value.borrow() {
                                    // TODO: WTF borrow checker???? Why is cloning the string required here? There's something wrong
                                    let cloned_key = value.to_string();
                                    let value = &entry_seq[1];
                                    result.insert(Cow::Owned(cloned_key), value.clone());
                                } else {
                                    return Option::None;
                                };
                            } else {
                                return Option::None;
                            }
                        }
                        Option::None => {
                            return Option::None;
                        }
                    };
                }
                Option::Some(Cow::Owned(result))
            }
            _ => Option::None,
        }
    }

    fn read_seq<'a>(value: &'a TextValue<'a>) -> Option<Cow<'a, Vec<TextValue<'a>>>> {
        match Self::value(value) {
            Value::Seq(ref seq) => Option::Some(Cow::Borrowed(seq)),
            Value::Map(ref map) => {
                // a map is also a sequence of tuples
                let mut resulting_vec: Vec<TextValue<'a>> = Vec::with_capacity(map.len());
                for (key, value) in map.iter() {
                    let value_key: TextValue<'a> = Value::String(key.clone()).into();
                    let entry_seq = Value::Seq(vec![value_key, value.clone()]);
                    resulting_vec.push(entry_seq.into());
                }
                Option::Some(Cow::Owned(resulting_vec))
            }
            _ => Option::None,
        }
    }
}
