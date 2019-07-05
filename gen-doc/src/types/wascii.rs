use crate::context::{Context, ContextProvider};
use crate::context::ContextFunctions;
use liquesco_common::error::LqError;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::types::option::TOption;
use minidom::Element;
use std::marker::PhantomData;
use crate::type_writer::TypeBodyWriter;
use crate::model::row::{Row, Link};
use crate::model::row;
use crate::model::card::CardId;
use liquesco_schema::types::binary::TBinary;
use crate::types::common::Common;
use liquesco_schema::types::ascii::TAscii;

pub struct WAscii<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WAscii<'a> {
    type T = TAscii<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext: ContextProvider<'b> {
        let mut result = Vec::new();

        // information about length
        let length = typ.length();
        if length.start() == length.end() {
            result.push(Row::association_with_text("Length (number of chars)",
                                                   Common::fmt_u64(*length.start())));
        } else {
            result.push(Row::association_with_text("Min length (inclusive; number of chars)",
                                                   Common::fmt_u64(*length.start())));
            result.push(Row::association_with_text("Max length (inclusive; number of chars)",
                                                   Common::fmt_u64(*length.end())));
        }

        // allowed codes
        let codes = typ.codes();
        let number_of_ranges = codes.len() / 2;
        for index in 0..number_of_ranges {
            let start = codes[index * 2];
            let end = codes[index * 2 + 1];

            result.push(Row::association_with_text(
                format!("Allowed code range #{}", index + 1),
                format!(
                    "{start} (inclusive) - {end} (exclusive); [{start_ascii}-{end_ascii}] \
                     (inclusive).",
                    start = start,
                    end = end,
                    start_ascii = char::from(start),
                    end_ascii = char::from(end - 1)
                )));
        }

        Ok(result)
    }
}
