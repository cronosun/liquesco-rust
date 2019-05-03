use crate::serialization::dyn_reader::DynReader;
use crate::common::bob::Bob;
use crate::common::error::LqError;
use crate::serialization::core::BinaryReader;
use crate::serialization::tuint::TUInt;
use crate::serialization::tlist::ListHeader;
use crate::serialization::core::DeSerializer;

pub trait Validator<'a> {
    fn validate(&self, reader : &mut DynReader<'a>, config : &Config) -> Result<(), LqError>;
}

pub struct VUInt {
    min_value : u64,
    max_value : u64,
}

impl<'a> Validator<'a> for VUInt {
    fn validate(&self, reader : &mut DynReader<'a>, _ : &Config) -> Result<(), LqError> {
        let int_value = TUInt::de_serialize(reader)?;
        if int_value < self.min_value {
            return LqError::err_new(format!("Given integer {:?} is too small (minimum allowed is {:?})", int_value, self.min_value))
        }
        if int_value > self.max_value {
            return LqError::err_new(format!("Given integer {:?} is too large (maximum allowed is {:?})", int_value, self.max_value))
        }
        Result::Ok(())       
    }
}

pub struct VStruct<'a>(Bob<'a, [Bob<'a, Validator<'a>>]>);

impl<'a> Validator<'a> for VStruct<'a> {
    fn validate(&self, reader : &mut DynReader<'a>, config : &Config) -> Result<(), LqError> {
        let list = ListHeader::de_serialize(reader)?;
        let schema_number_of_items = self.0.len();
        let number_of_items = list.length();
        // length check
        if config.no_extension() {
            if number_of_items!=schema_number_of_items {
                return LqError::err_new(format!("Invalid number of items in struct. Need {:?} fields, have {:?} fields (strict mode)", schema_number_of_items,number_of_items ));
            }
        } else {
            if number_of_items<schema_number_of_items {
                return LqError::err_new(format!("Some fields are missing in the given struct. Need at least {:?} fields, have {:?} fields.", schema_number_of_items,number_of_items ));
            }
        }
        // check each item
        for index in 0..schema_number_of_items {
            let validator = &self.0[index];
            validator.validate(reader, config)?;
        }
        // skip the rest
        let renaiming = number_of_items - schema_number_of_items;
        for _ in 0..renaiming {
            reader.skip()?;
        }
        Result::Ok(())
    }
}


pub struct Config {
    no_extension : bool
}

impl Config {

    pub fn no_extension(&self) ->bool {
        self.no_extension
    }

}