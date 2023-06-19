use crate::{ character::Character, types::{ structs::{ ApiParams }, enums::SortOptions } };
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
                matched &= rarity.iter().any(|r| &character.rarity == r);
            }
            if let Some(color) = &params.color {
                matched &= color.iter().any(|c| &character.color == c);
            }
            if let Some(id) = &params.id {
                matched &= id.iter().any(|c| &character.id == c);
            }
            if let Some(tags) = &params.tags {
                matched &= tags.iter().any(|tag| { character.tags.iter().any(|c| c == tag) });
            }
            if let Some(_) = &params.has_zenkai {
                matched &= character.has_zenkai;
            }
            if let Some(_) = &params.has_ultimate {
                matched &= character.ultimate_skill.is_some();
            }
            if let Some(_) = &params.is_tag {
                matched &= character.is_tag;
            }
            if let Some(_) = &params.is_lf {
                matched &= character.is_lf;
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
                SortOptions::Color => sort_by_field!(color, a, b),
                SortOptions::Rarity => sort_by_field!(rarity, a, b),
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
