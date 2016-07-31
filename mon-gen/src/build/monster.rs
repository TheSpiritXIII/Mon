//! Generates enums used for `Monster` classifier values.
use std::io::Write;

use types::species::{GrowthId};

use build::{BuildResult, CodeGenerate};
use build::util::{IdNamePairSet, IdResource, Identifiable, write_disclaimer};

#[derive(Debug, Deserialize)]
pub struct MonsterClassifiers
{
	natures: IdNamePairSet<GrowthId>,
	#[serde(rename = "catch-methods")]
	catch_methods: IdNamePairSet<GrowthId>,
}

impl CodeGenerate for MonsterClassifiers
{
	fn is_valid(&self) -> BuildResult
	{
		try!(IdResource::sequential(&self.natures));
		IdResource::sequential(&self.catch_methods)
	}
	fn gen_rust(&self, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "monster classifiers"));
		try!(writeln!(out, "use rand;\n"));
		try!(IdResource::gen_rust_enum(out, "Nature", &self.natures));
		try!(writeln!(out,
"impl rand::Rand for Nature
{{
	fn rand<R: rand::Rng>(rng: &mut R) -> Self
	{{
		*rng.choose(&["));
		for nature in &self.natures
		{
			try!(writeln!(out, "\t\t\tNature::{},", nature.name()))
		}
		try!(writeln!(out, "\t\t]).unwrap()\n\t}}\n}}"));
		IdResource::gen_rust_enum(out, "CatchMethod", &self.catch_methods)
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		try!(IdResource::gen_constants(out, "NATURE", &self.natures));
		IdResource::gen_constants(out, "CATCH_METHOD", &self.catch_methods)
	}
}
