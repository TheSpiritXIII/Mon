//! Generates a `Gender` enum.
use std::collections::HashSet;
use std::io::Write;

use types::gender::{GenderId, GenderRatioId};

use build::{Error, BuildResult, CodeGenerate};
use util::{Numeric, IdNamePairSet, IdResource, Identifiable, write_disclaimer};

#[derive(Debug, Deserialize)]
pub struct IdNamePairRatio
{
	id: GenderRatioId,
	name: String,
	internal: Option<String>,
	ratio: String,
}

impl IdNamePairRatio
{
	fn get_ratio(&self) -> Option<Vec<usize>>
	{
		let mut result = Vec::new();
		for split in self.ratio.split(":")
		{
			match split.parse::<usize>()
			{
				Ok(n) => result.push(n),
				Err(_) => return None,
			}
		}
		Some(result)
	}
	fn gen_rust_ratios<IdType: Numeric>(out: &mut Write, name: &str, ids: &HashSet<IdNamePairRatio>,
		of_name: &str, of_ids: &IdNamePairSet<IdType>) -> Result<(), Error>
	{
		//try!(write_disclaimer(out, "gender classifiers"));
		try!(write!(out, "
impl {}
{{
	pub fn rand<R: Rng>(rng: &mut R, ratio: {}) -> Self
	{{
		match ratio
		{{\n", of_name, name));

		for id in ids
		{
			try!(write!(out, "\t\t\t{}::{} => rng.choose(&[", name,
				Identifiable::identifier(id)));

			let ratios = try!(id.get_ratio().ok_or(Error::SyntaxError(
				"Invalid ratio syntax".to_string())));
			if ratios.len() != of_ids.len()
			{
				try!(id.get_ratio().ok_or(Error::SyntaxError(
					"The ratio length must match the number of attributes".to_string())));
			}

			for (index, value) in ratios.iter().enumerate()
			{
				for _ in 0..*value
				{
					let i = IdType::from(index).unwrap();
					try!(write!(out, "{}::{},", of_name,
						of_ids.get::<IdType>(&i).unwrap().name()));
				}
			}

			try!(writeln!(out, "]).unwrap().clone(),"));
		}

		try!(writeln!(out, "\t\t}}\n\t}}\n}}"));
		Ok(())
	}
}

derive_for_id!(IdNamePairRatio, GenderRatioId);

#[derive(Debug, Deserialize)]
pub struct GenderClassifiers
{
	genders: IdNamePairSet<GenderId>,
	#[serde(rename = "gender-ratios")]
	gender_ratios: HashSet<IdNamePairRatio>,
}

impl CodeGenerate for GenderClassifiers
{
	fn is_valid(&self) -> BuildResult
	{
		try!(IdResource::sequential(&self.genders));
		IdResource::sequential(&self.gender_ratios)
	}
	fn gen_rust(&self, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "genders classifiers"));
		try!(writeln!(out, "use rand::Rng;\n"));
		try!(IdResource::gen_rust_enum(out, "Gender", &self.genders));
		try!(IdResource::gen_rust_enum(out, "GenderRatio", &self.gender_ratios));
		IdNamePairRatio::gen_rust_ratios(out, "GenderRatio", &self.gender_ratios, "Gender",
			&self.genders)
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		try!(IdResource::gen_constants(out, "GENDER", &self.genders));
		IdResource::gen_constants(out, "GENDER_RATIO", &self.gender_ratios)
	}
}
