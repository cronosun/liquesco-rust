use std::borrow::Cow;

#[derive(Hash, PartialEq, Eq, PartialOrd)]
pub struct Path(Vec<Segment>);

impl Path {
    pub fn new(segment: Segment) -> Self {
        let mut this = Self(Vec::new());
        this.0.push(segment);
        this
    }
}

#[derive(From, Into, Hash, PartialEq, Eq, PartialOrd)]
pub struct Segment(Cow<'static, str>);

impl Segment {
    pub fn new<T: Into<Cow<'static, str>>>(string: T) -> Self {
        Self(string.into())
    }
}
