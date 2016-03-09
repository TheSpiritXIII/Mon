//! Generates a `Location` enum.
use std::io::Write;

use types::generic::{LocationId, SubLocationId};

use build::{BuildResult, CodeGenerate};
use util::{IdNamePairSet, IdResource, write_disclaimer};

#[derive(Debug, Deserialize)]
pub struct LocationClassifiers
{
	locations: IdNamePairSet<LocationId>,
	#[serde(rename = "sub-locations")]
	sub_locations: IdNamePairSet<SubLocationId>,
}

impl CodeGenerate for LocationClassifiers
{
	fn is_valid(&self) -> BuildResult
	{
		try!(IdResource::sequential(&self.locations));
		IdResource::sequential(&self.sub_locations)
	}
	fn gen_rust(&self, out: &mut Write) -> BuildResult
	{
		try!(write_disclaimer(out, "location classifiers"));
		try!(IdResource::gen_rust_enum(out, "Location", &self.locations));
		IdResource::gen_rust_enum(out, "SubLocation", &self.sub_locations)
	}
	fn gen_constants(&self, out: &mut Write) -> BuildResult
	{
		try!(IdResource::gen_constants(out, "LOCATION", &self.locations));
		IdResource::gen_constants(out, "SUBLOCATION", &self.sub_locations)
	}
}
