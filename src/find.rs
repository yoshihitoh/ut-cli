use failure::Fail;

#[derive(Fail, Debug, PartialEq)]
pub enum FindError {
    #[fail(display = "No matching item found.")]
    NotFound,

    #[fail(display = "Ambiguous item given. candidates: {:?}", _0)]
    Ambiguous(Vec<String>),
}

pub fn find_items<E, I>(items: I, name: &str) -> Vec<E>
where
    E: ToString + Copy,
    I: Iterator<Item = E>,
{
    items.filter(|x| x.to_string().starts_with(name)).collect()
}

fn find_by_name<T, I>(items: I, name: &str) -> Result<T, FindError>
where
    T: Copy + ToString,
    I: Iterator<Item = T>,
{
    let found = find_items(items, name);
    if found.len() == 1 {
        Ok(*found.first().unwrap())
    } else if found.is_empty() {
        Err(FindError::NotFound)
    } else {
        let names = found.into_iter().map(|x| x.to_string()).collect();
        Err(FindError::Ambiguous(names))
    }
}

pub trait PossibleValues: ToString + Copy {
    type Iterator: Iterator<Item = Self>;

    fn possible_values() -> Self::Iterator;
}

pub trait FindByName: PossibleValues {
    type Error: From<FindError>;

    fn find_by_name(name: &str) -> Result<Self, Self::Error> {
        Ok(find_by_name(Self::possible_values(), name)?)
    }

    fn find_by_name_opt(maybe_name: Option<&str>) -> Result<Option<Self>, Self::Error> {
        maybe_name
            .map(move |s| Self::find_by_name(s).map(Some))
            .unwrap_or_else(|| Ok(None))
    }
}

pub fn enum_names<E, I>(items: I) -> Vec<String>
where
    E: ToString,
    I: Iterator<Item = E>,
{
    items.map(|x| x.to_string()).collect()
}
