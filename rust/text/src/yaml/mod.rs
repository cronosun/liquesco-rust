use crate::parser::core::{ParseError, Context};
use crate::parser::parser::ParserContext;
use liquesco_core::schema::core::{Schema, Config};
use liquesco_core::serialization::vec_writer::VecWriter;
use yaml_rust::{Yaml, YamlLoader};
use crate::yaml::deserializer::deserialize;
use liquesco_core::serialization::slice_reader::SliceReader;
use std::marker::PhantomData;

pub mod deserializer;

pub fn parse_from_yaml_str<'s, S>(schema: &S, src: &str) -> Result<Vec<u8>, ParseError>
    where S: Schema<'s> {
    let mut docs = YamlLoader::load_from_str(src).unwrap(); // TODO: "Unwrap"
    let yaml = docs.remove(0);
    parse_from_yaml(schema, yaml)
}

pub fn parse_from_yaml<'s, S>(schema: &S, yaml: Yaml) -> Result<Vec<u8>, ParseError>
    where S: Schema<'s> {
    let value = deserialize(yaml)?;

    let context = ParserContext {
        value: &value,
        schema,
        _phantom : &PhantomData,
    };

    let mut writer = VecWriter::default();
    context.parse(&mut writer, schema.main_type(), &value)?;
    let data = writer.finish();

    // Now validate the result
    let mut reader: SliceReader = (&data).into();
    schema.validate(Config::strict(), &mut reader)?;

    Result::Ok(data)
}