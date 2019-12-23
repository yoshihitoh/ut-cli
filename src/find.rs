use std::str::FromStr;

use failure::Fail;

#[derive(Fail, Debug, PartialEq)]
pub enum FindError {
    #[fail(display = "No matching item found.")]
    NotFound,

    #[fail(display = "Ambiguous item given. candidates: {:?}", _0)]
    Ambiguous(Vec<String>),
}

fn find_items<E, I>(items: I, name: &str) -> Vec<E>
where
    E: ToString + Copy,
    I: Iterator<Item = E>,
{
    let name = name.to_ascii_lowercase();
    items
        .filter(|x| x.to_string().to_ascii_lowercase().starts_with(&name))
        .collect()
}

fn find_by_name<T, I>(items: I, name: &str) -> Result<T, FindError>
where
    T: Copy + ToString,
    I: Iterator<Item = T>,
{
    let found = find_items(items, &name);
    if found.len() == 1 {
        Ok(*found.first().unwrap())
    } else if found.is_empty() {
        Err(FindError::NotFound)
    } else {
        let names = found.into_iter().map(|x| x.to_string()).collect();
        Err(FindError::Ambiguous(names))
    }
}

pub trait PossibleValues: Copy {
    type Iterator: Iterator<Item = Self>;

    fn possible_values() -> Self::Iterator;
}

pub trait PossibleNames: PossibleValues + ToString {
    fn possible_names() -> Vec<String> {
        Self::possible_values()
            .map(|x| x.to_string().to_ascii_lowercase())
            .collect()
    }
}

pub trait FindByName: PossibleValues + ToString + FromStr {
    type Error: From<FindError>;

    fn find_by_name(name: &str) -> Result<Self, Self::Error> {
        Self::from_str(name)
            .or_else(|_| find_by_name(Self::possible_values(), name).map_err(Self::Error::from))
    }

    fn find_by_name_opt(maybe_name: Option<&str>) -> Result<Option<Self>, Self::Error> {
        maybe_name
            .map(move |s| Self::find_by_name(s).map(Some))
            .unwrap_or_else(|| Ok(None))
    }
}
