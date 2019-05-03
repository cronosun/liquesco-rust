use std::ops::Deref;

/// Boxed-or-borrowed (bob).
pub enum Bob<'a, T : ?Sized> {    
    Borrowed(&'a T),
    Boxed(Box<T>)
}

impl<'a, T : ?Sized> Deref for Bob<'a, T> {
    type Target = T;

     fn deref(&self) -> &Self::Target {
         match self {
             Bob::Borrowed(value) => value,
             Bob::Boxed(ref value) => &value
         } 
     }
}

impl<'a, T : ?Sized> From<&'a T> for Bob<'a, T> {
    fn from(value: &'a T) -> Self {
        Bob::Borrowed(value)
    }
}

impl<T : ?Sized> From<Box<T>> for Bob<'static, T> {
    fn from(value: Box<T>) -> Self {
        Bob::Boxed(value)
    }
}
