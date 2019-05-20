pub enum Option<T> {
    /// No value
    None,
    /// Some value `T`
    Some(T),
}

impl<T> Default for Option<T> {
    /// Returns None.
    #[inline]
    fn default() -> Option<T> {
        None
    }
}
