use crate::parser::core::{Context, ParseError};
use crate::parser::parser::ParserContext;
use crate::yaml::deserializer::deserialize;
use liquesco_schema::core::{Config, Schema};
use liquesco_serialization::slice_reader::SliceReader;
use liquesco_serialization::vec_writer::VecWriter;
use std::marker::PhantomData;
use yaml_rust::{Yaml, YamlLoader};

pub mod deserializer;

pub fn parse_from_yaml_str<'s, S>(schema: &S, src: &str) -> Result<Vec<u8>, ParseError>
where
    S: Schema<'s>,
{
    let mut docs = YamlLoader::load_from_str(src).unwrap(); // TODO: "Unwrap"
    let yaml = docs.remove(0);
    parse_from_yaml(schema, yaml)
}

pub fn parse_from_yaml<'s, S>(schema: &S, yaml: Yaml) -> Result<Vec<u8>, ParseError>
where
    S: Schema<'s>,
{
    let value = deserialize(yaml)?;

    let mut context = ParserContext {
        schema,
        anchor_info: Option::None,
        _phantom: &PhantomData,
    };

    let mut writer = VecWriter::default();
    context.parse(&mut writer, schema.main_type(), &value)?;
    let data = writer.finish();

    // Now validate the result
    let mut reader: SliceReader = (&data).into();
    schema.validate(Config::strict(), &mut reader)?;

    Result::Ok(data)
}
