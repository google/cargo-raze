#[derive(Derivative)]
#[derivative(Debug="transparent")]
pub struct Wrapping<T>(pub T);
