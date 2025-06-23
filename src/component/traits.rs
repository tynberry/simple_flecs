use super::Component;

/// Trait representing a component or a pair of components.
pub trait ComponentOrPair {
    /// Is component or a pair?
    const IS_PAIR: bool = false;
    /// Is a tag pair.
    const IS_TAG: bool = false;
    /// Type of first one.
    type First: Component;
    /// Type of Second one.
    type Second: Component;
}

/// Type used when that part of the pair is empty.
pub struct EmptyType;

impl Component for EmptyType {
    /// Is this a tag?
    const IS_TAG: bool = true;
}

impl<T> ComponentOrPair for T
where
    T: Component,
{
    const IS_PAIR: bool = T::IS_TAG;
    type First = T;
    type Second = EmptyType;
}

impl<L, R> ComponentOrPair for (L, R)
where
    L: Component,
    R: Component,
{
    const IS_PAIR: bool = true;
    const IS_TAG: bool = L::IS_TAG && R::IS_TAG;
    type First = L;
    type Second = R;
}
