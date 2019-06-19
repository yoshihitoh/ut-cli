use std::fmt::Display;
use std::str::FromStr;

use failure::Fail;
use strum::IntoEnumIterator;

#[derive(Fail, Debug, PartialEq)]
pub enum FindError {
    #[fail(display = "No matching item found. given: {}", _0)]
    NotFound(String),

    #[fail(
        display = "Multiple candidates found. given: {}, candidates: {:?}",
        _0, _1
    )]
    Ambiguous(String, Vec<String>),
}

pub fn find_items<E, I>(items: I, name: &str) -> Vec<E>
where
    E: ToString + Copy,
    I: Iterator<Item = E>,
{
    items.filter(|x| x.to_string().starts_with(name)).collect()
}

pub fn find_enum_item<E, I>(name: &str) -> Result<E, FindError>
where
    E: IntoEnumIterator<Iterator = I> + FromStr + Copy + Display,
    I: Iterator<Item = E>,
{
    E::from_str(name).map(|x| Ok(x)).unwrap_or_else(|_| {
        let items = find_items(E::iter(), name);
        if items.len() == 1 {
            Ok(*items.first().unwrap())
        } else if items.is_empty() {
            Err(FindError::NotFound(name.to_string()))
        } else {
            let names = items.into_iter().map(|x| x.to_string()).collect();
            Err(FindError::Ambiguous(name.to_string(), names))
        }
    })
}

pub fn enum_names<E, I>(items: I) -> Vec<String>
where
    E: ToString,
    I: Iterator<Item = E>,
{
    items.map(|x| x.to_string()).collect()
}
