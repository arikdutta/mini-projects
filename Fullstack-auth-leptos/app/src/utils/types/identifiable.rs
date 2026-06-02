use core::fmt::Display;
use core::hash::Hash;

pub trait Identifiable {
    type Pk;
    type Id: Copy + Eq + Hash + Display;

    fn id(&self) -> Self::Id;
}
