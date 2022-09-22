use marketplace_domain::HexPrefixedString;
use okapi::openapi3::SchemaObject;
use schemars::{
	schema::{InstanceType, StringValidation},
	JsonSchema,
};
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct HexPrefixedStringDto(HexPrefixedString);

impl From<HexPrefixedStringDto> for HexPrefixedString {
	fn from(value: HexPrefixedStringDto) -> Self {
		value.0
	}
}

impl JsonSchema for HexPrefixedStringDto {
	fn schema_name() -> String {
		"HexPrefixedString".to_string()
	}

	fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
		let schema = SchemaObject {
			instance_type: Some(InstanceType::String.into()),
			string: Some(Box::new(StringValidation {
				min_length: Some(3),
				max_length: Some(66),
				pattern: Some("\\b0x[0-9a-f]+\\b".to_string()),
			})),
			..Default::default()
		};

		schema.into()
	}
}

#[derive(Error, Debug)]
pub enum FromHexPrefixedStringError {
	#[error("Failed to convert")]
	Convertion(#[from] anyhow::Error),
}

pub trait TryFromHexPrefixedString: Sized {
	type Error;
	fn try_from_hex_prefixed_string(value: HexPrefixedString) -> Result<Self, Self::Error>;
}

impl TryFromHexPrefixedString for FieldElement {
	type Error = FromHexPrefixedStringError;

	fn try_from_hex_prefixed_string(value: HexPrefixedString) -> Result<Self, Self::Error> {
		FieldElement::from_hex_be(&value.to_string())
			.map_err(anyhow::Error::msg)
			.map_err(FromHexPrefixedStringError::from)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use marketplace_domain::HexPrefixedString;
	use rstest::rstest;
	use std::str::FromStr;

	#[rstest]
	fn try_from_hex_prefixed_string() {
		let string = HexPrefixedString::from_str("0x112233").unwrap();
		assert_eq!(string.to_string(), "0x00112233");
		let felt: FieldElement = FieldElement::try_from_hex_prefixed_string(string).unwrap().into();
		assert_eq!(felt.to_string(), "1122867");
	}
}