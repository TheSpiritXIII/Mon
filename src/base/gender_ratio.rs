use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use rand::distributions::range::SampleRange;
use std::ops::Add;
use num::traits::Zero;
use super::types::Gender;

pub enum GenderRatio
{
	Genderless       = 0,
	MaleOnly         = 1,
	FemaleOnly       = 2,
	EqualMaleFemale  = 3,
	ThreeMaleToOne   = 4,
	ThreeFemaleToOne = 5,
	SevenMaleToOne   = 6,
	SevenFemaleToOne = 7,
}

fn random_ratio<R: Rng, X: SampleRange + PartialOrd + Zero + Add<Output = X> + Copy, Who>(rng: &mut R, num1: X, who1: Who,
	num2: X, who2: Who) -> Who
{
	let range = Range::new(X::zero(), num1 + num2);
	if range.ind_sample(rng) < num1
	{
		return who1;
	}
	who2
}

impl GenderRatio
{
	pub fn gender<R: Rng>(ratio: GenderRatio, rng: &mut R) -> Gender
	{
		match ratio
		{
			GenderRatio::Genderless => Gender::Genderless,
			GenderRatio::MaleOnly => Gender::Male,
			GenderRatio::FemaleOnly => Gender::Female,
			GenderRatio::EqualMaleFemale => random_ratio(rng, 1, Gender::Male, 1, Gender::Female),
			GenderRatio::ThreeMaleToOne => random_ratio(rng, 3, Gender::Male, 1, Gender::Female),
			GenderRatio::ThreeFemaleToOne => random_ratio(rng, 3, Gender::Female, 1, Gender::Male),
			GenderRatio::SevenMaleToOne => random_ratio(rng, 7, Gender::Male, 1, Gender::Female),
			GenderRatio::SevenFemaleToOne => random_ratio(rng, 7, Gender::Female, 1, Gender::Male),
		}
	}
}
