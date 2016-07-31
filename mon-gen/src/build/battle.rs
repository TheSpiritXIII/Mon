//! Generates enums for battle related types.
use std::io::Write;

use build::{BuildResult, CodeGenerate};
use build::util::{IdNamePairSet, IdResource, write_disclaimer};
use types::attack::CategoryId;

#[derive(Debug, Deserialize)]
pub struct BattleClassifiers
{
	categories: IdNamePairSet<CategoryId>,
}

impl CodeGenerate for BattleClassifiers
{
	fn is_valid(&self) -> BuildResult
	{
		IdResource::sequential(&self.categories)
	}
	fn gen_rust(&self, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "battle classifiers"));
		IdResource::gen_rust_enum(out, "Category", &self.categories)
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		IdResource::gen_constants(out, "ATTACK_CATEGORY", &self.categories)
	}
}
