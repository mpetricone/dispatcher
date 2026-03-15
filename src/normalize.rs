//! A trait that will normalize a struct.
//! There is no stipulation on the methods used.
pub trait Normalizer {
    fn normalize(&mut self) -> &mut Self;
}
