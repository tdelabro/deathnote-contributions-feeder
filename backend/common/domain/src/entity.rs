use std::{fmt::Display, hash::Hash};

use diesel::Identifiable;

pub trait Entity {
	type Id: PartialEq + Eq + Display + Hash + Clone + Sync + Send;
}

impl<'ident, T> Entity for &'ident T
where
	&'ident T: Identifiable,
	<&'ident T as Identifiable>::Id: Display + Clone + Sync + Send,
{
	type Id = <Self as Identifiable>::Id;
}
