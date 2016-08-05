//! Generates enums used for `Monster` classifier values.
use std::io::Write;

use build::{BuildResult, CodeGenerate};
use build::util::{IdNamePairSet, IdResource, Identifiable, write_disclaimer};
use types::species::{GrowthId};

#[derive(Debug, Deserialize)]
pub struct MonsterClassifiers
{
	natures: IdNamePairSet<GrowthId>,
	#[serde(rename = "recruit-methods")]
	recruit_methods: IdNamePairSet<GrowthId>,
}

impl CodeGenerate for MonsterClassifiers
{
	fn is_valid(&self) -> BuildResult
	{
		try!(IdResource::sequential(&self.natures));
		IdResource::sequential(&self.recruit_methods)
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
		IdResource::gen_rust_enum(out, "RecruitMethod", &self.recruit_methods)
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		try!(IdResource::gen_constants(out, "NATURE", &self.natures));
		IdResource::gen_constants(out, "RECRUIT_METHOD", &self.recruit_methods)
	}
}
