use crate::{
    character::Character,
    types::{structs::{ApiParams},enums::SortOptions}
};
macro_rules! sort_by_field {
    ($field:ident, $a:expr, $b:expr) => {
        $a.$field.cmp(&$b.$field)
    };
}
pub fn apply_filters(params: ApiParams, characters: &Vec<Character>) -> Vec<&Character> {
    let filtered_characters = characters
        .iter()
        .filter(|&character| {
            let mut matched = true;
            if let Some(name) = &params.name {
                matched &= name.iter().any(|n| &character.name == n);
            }

            if let Some(fname) = &params.fname {
                matched &= fname.iter().any(|fnm| character.name.contains(fnm));
            }
            if let Some(rarity) = &params.rarity {
                matched &= character.rarity.as_string() == rarity.as_string();
            }
            matched
        })
        .collect::<Vec<&Character>>(); // collect into a new Vec of references

    filtered_characters // return a reference to the new vector
}

pub fn apply_sort(params: ApiParams, mut characters: Vec<&Character>) -> Vec<&Character> {
    if let Some(sort_by) = &params.sort_by {
        characters.sort_by(|a, b| {
            match sort_by {
                SortOptions::Name => sort_by_field!(name, a, b),
            }
        });
    }
    match params.reverse {
        Some(_) => {
            characters.reverse();
        }
        None => {}
    }
    characters
}