/// The `Unpin` auto trait means that it is safe to move out of a `Pin`
/// reference to this type.
///
/// It is not implemented by self-referential types.
pub unsafe auto trait Unpin { }
