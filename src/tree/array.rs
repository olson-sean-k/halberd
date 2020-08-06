// TODO: Support error propagation.
pub trait FromFn<T>: Sized {
    fn from_fn(f: impl FnMut() -> T) -> Self;

    fn from_iter<I>(input: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut input = input.into_iter();
        Self::from_fn(move || input.next().expect(""))
    }
}

impl<T> FromFn<T> for [T; 4] {
    fn from_fn(mut f: impl FnMut() -> T) -> Self {
        [f(), f(), f(), f()]
    }
}

impl<T> FromFn<T> for [T; 8] {
    fn from_fn(mut f: impl FnMut() -> T) -> Self {
        [f(), f(), f(), f(), f(), f(), f(), f()]
    }
}
