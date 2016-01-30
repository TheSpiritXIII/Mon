//! Generates source code for custom species.
use build::identifier::Identifier;
use build::display::{DisplayVec, DisplayEqualsHashMap, format_null_terminated};
use build::parse;
use build::parse::Resource;
use build::types;
use build::types::{SpeciesId, FormId, BaseStatType};

use std::rc::Rc;
use std::collections::{HashMap};
use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(Debug, Deserialize)]
enum ListCode
{
	List(Vec<Rc<String>>),
	Code(Rc<String>),
}

#[derive(Debug, Deserialize)]
enum FloatCode
{
	Float(f32),
	Code(Rc<String>),
}

#[derive(Clone, Debug, Deserialize)]
struct Form
{
	name: Rc<String>,
	id: FormId,
	internal: Option<String>,
}

#[derive(Debug, Deserialize)]
enum StatCode
{
	Stat(BaseStatType),
	Code(String),
}

#[derive(Debug, Deserialize)]
struct Statistics
{
	health: StatCode,
	attack: StatCode,
	defense: StatCode,
	#[serde(rename="special-attack")]
	spattack: StatCode,
	#[serde(rename="special-defense")]
	spdefense: StatCode,
	speed: StatCode,
}

#[derive(Debug, Deserialize)]
struct Species
{
	name: Rc<String>,
	id: SpeciesId,
	internal: Option<String>,
	description: Rc<String>,
	kind: Rc<String>,
	#[serde(default)]
	forms: Vec<Form>,
	elements: ListCode,
	groups: ListCode,
	growth: Rc<String>,
	color: String,
	habitat: String,
	#[serde(rename="gender-ratio")]
	gender_ratio: Option<String>,
	weight: FloatCode,
	height: FloatCode,
	statistics: Statistics,
}

resource!(Species, SpeciesId);

fn species_code(file: &mut File, species_map: &HashMap<types::SpeciesId, (Species, Identifier)>)
{
	let reverse_map = DisplayEqualsHashMap(species_map.iter().map(|(id, &(_, ref symbol))|
	{
		(symbol.identifier(), id.clone())
	})
	.collect::<HashMap<&String, types::SpeciesId>>());
	
	let match_index = DisplayVec(species_map.iter().map(|(id, &(_, ref symbol))|
	{
		format!("{} => Some(&self.{})", id, symbol.identifier().to_lowercase())
	})
	.collect::<Vec<String>>());
	
	let field_list = DisplayVec(species_map.iter().map(|(_, &(_, ref symbol))|
	{
		format!("{}: {}", symbol.identifier().to_lowercase(), symbol.identifier())
	})
	.collect::<Vec<String>>());
	
	if let Err(err) = writeln!(file,
"//! Generated code for species list.
use species;
use types::SpeciesId;
use resource::ResourceList;

pub use super::element_gen::Element;
pub use super::classifiers_gen;

#[allow(dead_code)]
pub enum Species
{{
	{species_ident_to_id}
}}

pub struct SpeciesList
{{
	{species_symbol_names}
}}

impl SpeciesList
{{
	pub const fn new() -> Self
	{{
		SpeciesList
		{{
			{species_symbol_names}
		}}
	}}
}}

impl ResourceList<species::Species, SpeciesId> for SpeciesList
{{
	fn get(&self, index: SpeciesId) -> Option<*const species::Species>
	{{
		return match index
		{{
			{species_match_id}
			_ => None,
		}}
	}}
	fn count() -> SpeciesId
	{{
		{species_count}
	}}
}}", species_ident_to_id = reverse_map, species_match_id = match_index,
		species_count = species_map.len(), species_symbol_names = field_list)
	{
		panic!("Unable to write species file header: {:?}", err.kind());
	}
	
	for (_, &(ref species, ref symbol)) in species_map
	{
		let mut forms = species.forms.clone();
		if forms.is_empty()
		{
			forms.push(Form
			{
				name: Rc::new("Default".to_string()),
				id: 0,
				internal: None,
			});
		}
	
		let form_idents = DisplayVec(forms.iter().map(|form|
		{
			let ident = Identifier::from_name(form.name.clone(), form.internal.clone());
			if  ident.is_none()
			{
				panic!("Unable to create species ({}) form symbol `{}`. Symbol invalid.",
					species.name, form.name);
			}
			format!("{} = {}", ident.unwrap().identifier(), form.id)
		})
		.collect::<Vec<String>>());
		
		let form_ids = forms.iter().map(|form|
		{
			form.id
		})
		.collect::<Vec<FormId>>();
		
		let form_names = DisplayVec(forms.iter().map(|form|
		{
			format!("{} => {}", form.id, format_null_terminated(&form.name))
		})
		.collect::<Vec<String>>());
		
		let element_code = match species.elements
		{
			ListCode::List(ref list) =>
			{
				Rc::new(format!("return vec![{}];", DisplayVec(list.iter().map(|x|
				{
					format!("Element::{}", x)
				})
				.collect::<Vec<String>>())))
			},
			ListCode::Code(ref code) => code.clone()
		};
		
		let group_code = match species.groups
		{
			ListCode::List(ref list) =>
			{
				Rc::new(format!("return vec![{}];", DisplayVec(list.iter().map(|x|
				{
					format!("Group::{}", x)
				})
				.collect::<Vec<String>>())))
			},
			ListCode::Code(ref code) => code.clone()
		};
		
		let gender_ratio = match species.gender_ratio
		{
			Some(ref ratio) => ratio.clone(),
			None => "Genderless".to_string(),
		};
		
		let height = match species.height
		{
			FloatCode::Float(ref num) =>
			{
				Rc::new(format!("return {} as MetricType", num))
			}
			FloatCode::Code(ref code) => code.clone()
		};
		
		let weight = match species.weight
		{
			FloatCode::Float(ref num) =>
			{
				Rc::new(format!("return {} as MetricType", num))
			}
			FloatCode::Code(ref code) => code.clone()
		};
		
		let health = match species.statistics.health
		{
			StatCode::Stat(ref num) => format!("return {};", num),
			StatCode::Code(ref code) => code.clone(),
		};
		
		let attack = match species.statistics.attack
		{
			StatCode::Stat(ref num) => format!("return {};", num),
			StatCode::Code(ref code) => code.clone(),
		};
		
		let defense = match species.statistics.defense
		{
			StatCode::Stat(ref num) => format!("return {};", num),
			StatCode::Code(ref code) => code.clone(),
		};
		
		let spattack = match species.statistics.spattack
		{
			StatCode::Stat(ref num) => format!("return {};", num),
			StatCode::Code(ref code) => code.clone(),
		};
		
		let spdefense = match species.statistics.spdefense
		{
			StatCode::Stat(ref num) => format!("return {};", num),
			StatCode::Code(ref code) => code.clone(),
		};
		
		let speed = match species.statistics.speed
		{
			StatCode::Stat(ref num) => format!("return {};", num),
			StatCode::Code(ref code) => code.clone(),
		};
		
		if let Err(err) = writeln!(file,
"
mod {lowercase}
{{
	use species;
	use super::Element;
	use types::{{SpeciesId, FormId, MetricType, BaseStatType}};
	use num::traits::FromPrimitive;
	use super::classifiers_gen::{{Group, Growth, Color, Habitat}};
	use gender_ratio::GenderRatio;
	
	#[derive(Debug, PartialEq, NumFromPrimitive)]
	pub enum Form
	{{
		{form_idents}
	}}

	pub struct {symbol};
	
	impl {symbol}
	{{
		#[allow(dead_code)]
		fn get_form(form: FormId) -> Form
		{{
			Form::from_usize(form as usize).unwrap()
		}}
	}}

	impl species::Species for {symbol}
	{{
		fn id(&self) -> SpeciesId
		{{
			{id}
		}}
		fn name(&self) -> &'static [u8]
		{{
			{name}
		}}
		fn description(&self) -> &'static [u8]
		{{
			{description}
		}}
		fn kind(&self) -> &'static [u8]
		{{
			{kind}
		}}
		fn form_ids(&self) -> Vec<FormId>
		{{
			vec!{form_ids:?}
		}}
		fn form_name(&self, form: FormId) -> &'static [u8]
		{{
			match form
			{{
				{form_names}
				_ => b\"\\0\",
			}}
		}}
		fn elements(&self, form_id: FormId) -> Vec<Element>
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{element_code}
			}}
			Vec::new()
		}}
		
		fn groups(&self) -> Vec<Group>
		{{
			{group_code}
		}}
		fn growth(&self) -> Growth
		{{
			Growth::{growth}
		}}
		fn color(&self) -> Color
		{{
			Color::{color}
		}}
		fn habitat(&self) -> Habitat
		{{
			Habitat::{habitat}
		}}
		fn gender_ratio(&self) -> GenderRatio
		{{
			GenderRatio::{gender_ratio}
		}}
		fn weight(&self, form_id: FormId) -> MetricType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{weight}
			}}
			0 as MetricType
		}}
		fn height(&self, form_id: FormId) -> MetricType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{height}
			}}
			0 as MetricType
		}}
		
		fn base_health(&self, form_id: FormId) -> BaseStatType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{base_health}
			}}
			0 as BaseStatType
		}}
		fn base_attack(&self, form_id: FormId) -> BaseStatType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{base_attack}
			}}
			0 as BaseStatType
		}}
		fn base_defense(&self, form_id: FormId) -> BaseStatType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{base_defense}
			}}
			0 as BaseStatType
		}}
		fn base_spattack(&self, form_id: FormId) -> BaseStatType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{base_spattack}
			}}
			0 as BaseStatType
		}}
		fn base_spdefense(&self, form_id: FormId) -> BaseStatType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{base_spdefense}
			}}
			0 as BaseStatType
		}}
		fn base_speed(&self, form_id: FormId) -> BaseStatType
		{{
			if let Some(_) = Form::from_usize(form_id as usize)
			{{
				{base_speed}
			}}
			0 as BaseStatType
		}}
	}}
}}

use self::{lowercase}::{symbol};", lowercase = symbol.identifier().to_lowercase(),
			symbol = symbol.identifier(), id = species.id,
			name = format_null_terminated(&species.name),
			description = format_null_terminated(&species.description),
			kind = format_null_terminated(&species.kind), form_idents = form_idents,
			form_names = form_names, form_ids = form_ids, element_code = element_code,
			group_code = group_code, growth = species.growth, color = species.color,
			habitat = species.habitat, gender_ratio = gender_ratio,
			height = height, weight = weight, base_health = health, base_attack = attack,
			base_defense = defense, base_spattack = spattack, base_spdefense = spdefense,
			base_speed = speed)
		{
			panic!("Unable to write species file ({}): {:?}", species.name, err.kind());
		}
	}
}

pub fn parse_species()
{
	let species_map: HashMap<SpeciesId, (Species, Identifier)> =
		parse::parse_toml_resource_directory("res/species", "species",
		cfg!(feature = "sequential_species"));
	
	println!("Generating source code `src/gen/species_gen.rs`...");
	{
		let source_file = File::create("src/gen/species_gen.rs");
		if let Err(err) = source_file
		{
			panic!("Unable to create source code `src/gen/species_gen.rs`: {:?}", err.kind());
		}
		species_code(&mut source_file.unwrap(), &species_map);
	}
	
	println!("Opening `constants.txt`...");
	{
		let constants_file_result = OpenOptions::new().append(true).open("import/constants.txt");
		if let Err(err) = constants_file_result
		{
			panic!("Unable to create file `constants.txt`: {:?}", err.kind());
		}
		
		println!("Writing to file `constants.txt`...");
		let mut constants_file = constants_file_result.unwrap();
		for (id, &(ref species, ref symbol)) in &species_map
		{
			if let Err(err) = writeln!(constants_file, "MON_SPECIES_{}={}",
				symbol.identifier_capitalized(), id)
			{
				panic!("Unable to open to file `constants.txt`: {:?}", err.kind());
			}
			for form in &species.forms
			{
				let ident = Identifier::from_name(form.name.clone(), form.internal.clone());
				if  ident.is_none()
				{
					panic!("Unable to create species ({}) form symbol `{}`. Symbol invalid.",
						species.name, form.name);
				}
				if let Err(err) = writeln!(constants_file, "MON_SPECIES_{}_FORM_{}={}",
					symbol.identifier_capitalized(), ident.unwrap().identifier_capitalized(),
					form.id)
				{
					panic!("Unable to write form to file `constants.txt`: {:?}", err.kind());
				}
			}
		}
	}
}
