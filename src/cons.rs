use std::borrow::Cow;
use std::hash::{BuildHasherDefault, Hash};
use std::collections::hash_map::{self, Entry, HashMap};

use seahash::SeaHasher;

/// Defines elements of the type lattice.
pub trait Constructor: PartialOrd + Clone {
    type Key: Eq + Hash;

    fn key(&self) -> Self::Key;

    fn join(&mut self, other: &Self);
    fn meet(&mut self, other: &Self);
}

pub(crate) struct ConstructorSet<C: Constructor> {
    set: HashMap<C::Key, C, BuildHasherDefault<SeaHasher>>,
}

impl<C: Constructor> ConstructorSet<C> {
    pub(crate) fn add_pos(&mut self, con: Cow<C>) {
        match self.set.entry(con.key()) {
            Entry::Occupied(mut entry) => entry.get_mut().join(&con),
            Entry::Vacant(entry) => {
                entry.insert(con.into_owned());
            }
        }
    }

    pub(crate) fn add_neg(&mut self, con: Cow<C>) {
        match self.set.entry(con.key()) {
            Entry::Occupied(mut entry) => entry.get_mut().meet(&con),
            Entry::Vacant(entry) => {
                entry.insert(con.into_owned());
            }
        }
    }

    pub(crate) fn join(&mut self, other: &Self) {
        self.set.reserve(other.set.len());
        for con in other.set.values() {
            self.add_pos(Cow::Borrowed(con));
        }
    }

    pub(crate) fn meet(&mut self, other: &Self) {
        self.set.reserve(other.set.len());
        for con in other.set.values() {
            self.add_neg(Cow::Borrowed(con));
        }
    }
}

impl<C: Constructor> Default for ConstructorSet<C> {
    fn default() -> Self {
        ConstructorSet {
            set: HashMap::default(),
        }
    }
}

impl<'a, C: Constructor> IntoIterator for &'a ConstructorSet<C> {
    type Item = &'a C;
    type IntoIter = hash_map::Values<'a, C::Key, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.values()
    }
}
