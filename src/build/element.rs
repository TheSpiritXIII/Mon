//! Generates source code for custom elements.
use build::identifier::Identifier;
use build::display::{DisplayVec, DisplayEqualsHashMap};
use build::parse;
use build::parse::Resource;
use build::types;
use build::types::ElementId;

use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Element
{
	name: Rc<String>,
	id: types::ElementId,
	internal: Option<String>,
	weaknesses: Vec<Rc<String>>,
	resistances: Vec<Rc<String>>,
	immunities: Vec<Rc<String>>,
}

resource!(Element, ElementId);

fn element_code(file: &mut File, elements: &HashMap<types::ElementId, (Element, Identifier)>)
{
	let mut symbols: HashMap<Rc<String>, types::ElementId> = HashMap::new();
	let mut unique: HashSet<Rc<String>> = HashSet::new();
	for (id, &(ref element, _)) in elements
	{
		symbols.insert(element.name.clone(), *id);
		if !unique.insert(element.name.clone())
		{
			panic!("Duplicate element `{}`, ID {}.", element.name, *id);
		}
	}
	
	let count = elements.len();
	
	let reverse_map = DisplayEqualsHashMap(elements.iter().map(|(id, &(_, ref symbol))|
	{
		(symbol.identifier(), id.clone())
	})
	.collect::<HashMap<&String, types::ElementId>>());
	
	let names = DisplayVec(((0 as types::ElementId)..(elements.len() as types::ElementId)).map(|x|
	{
		let &(ref e, _) =  elements.get(&x).unwrap();
		format!("b\"{}\\0\"", e.name.clone())
	})
	.collect::<Vec<String>>());
	
	let mut effectiveness_chart = vec![1.0f32; elements.len() * elements.len()];
	for (_, &(ref element, _)) in elements
	{
		if let Some(id) = symbols.get(&element.name)
		{
			for weakness in &element.weaknesses
			{
				if let Some(other) = symbols.get(weakness)
				{
					effectiveness_chart[(*id as usize) * elements.len() + (*other as usize)] = 0.5;
				}
				else
				{
					panic!("Unable to find element weakness '{}' from '{}'", weakness,
						element.name);
				}
			}
			for resistance in &element.resistances
			{
				if let Some(other) = symbols.get(resistance)
				{
					effectiveness_chart[(*other as usize) * elements.len() + (*id as usize)] = 2.0;
				}
				else
				{
					panic!("Unable to find element resistance '{}' from '{}'", resistance,
						element.name);
				}
			}
			for immunity in &element.immunities
			{
				if let Some(other) = symbols.get(immunity)
				{
					effectiveness_chart[(*other as usize) * elements.len() + (*id as usize)] = 0.0;
				}
				else
				{
					panic!("Unable to find element immunity '{}' from '{}'", immunity,
						element.name);
				}
			}
		}
		else
		{
			panic!("Unable to find element '{}'", element.name);
		}
	}
	
	let effects = DisplayVec(effectiveness_chart.iter().map(|x|
	{
		format!("{} as ElementEffectType", x)
	})
	.collect::<Vec<String>>());

	if let Err(err) = writeln!(file,
"//! Generated code for elements.
use element;
use types::{{ElementId, ElementEffectType}};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Element
{{
	{element_ident_to_id}
}}

pub struct ElementList;

impl element::ElementList for ElementList
{{
	fn count() -> ElementId
	{{
		{element_count}
	}}
	
	fn name(element: ElementId) -> &'static [u8]
	{{
		const NAMES: [&'static [u8]; {element_count}] = [
			{element_names}
		];
		NAMES[element as usize]
	}}

	fn effect(offense: ElementId, defense: ElementId) -> ElementEffectType
	{{
		const EFFECTS: [f32; {element_count_squared}] = [
			{element_effectiveness}
		];
		EFFECTS[(offense * ElementList::count() + defense) as usize]
	}}
}}", element_count = count, element_count_squared = count * count,
		element_ident_to_id = reverse_map, element_names = names, element_effectiveness = effects)
	{
		panic!("Unable to write element file: {:?}", err.kind());
	}
}

pub fn parse_elements()
{
	let element_map: HashMap<types::ElementId, (Element, Identifier)> =
		parse::parse_toml_resource_directory("res/elements", "element", true);
	
	println!("Generating source code `src/gen/element_gen.rs`...");
	{
		let source_file = File::create("src/gen/element_gen.rs");
		if let Err(err) = source_file
		{
			panic!("Unable to create source code `src/gen/element_gen.rs`: {:?}", err.kind());
		}
		element_code(&mut source_file.unwrap(), &element_map);
	}
	
	println!("Opening `constants.txt`...");
	{
		let constants_file_result = OpenOptions::new().append(true).open("import/constants.txt");
		if let Err(err) = constants_file_result
		{
			panic!("Unable to open file `constants.txt`: {:?}", err.kind());
		}
		
		println!("Writing to file `constants.txt`...");
		let mut constants_file = constants_file_result.unwrap();
		for (id, &(_, ref symbol)) in &element_map
		{
			if let Err(err) = writeln!(constants_file, "MON_ELEMENT_{}={}",
				symbol.identifier_capitalized(), id)
			{
				panic!("Unable to write to file `constants.txt`: {:?}", err.kind());
			}
		}
	}
}
