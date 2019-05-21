use std::borrow::Cow;
use smallvec::SmallVec;

#[derive(Hash, PartialEq, PartialOrd)]
pub struct Path(SmallVec<[Segment; 4]>);

impl Path {
    pub fn new(segment : Segment) -> Self {
        let mut this = Self(SmallVec::new());
        this.0.push(segment);
        this
    }
}

#[derive(From, Into, Hash, PartialEq, PartialOrd)]
pub struct Segment(Cow<'static, str>);

