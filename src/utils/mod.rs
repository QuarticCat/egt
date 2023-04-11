mod union_find;

pub use union_find::*;

use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;

type FxHashBuilder = BuildHasherDefault<FxHasher>;

pub type HashMap<K, V> = hashbrown::HashMap<K, V, FxHashBuilder>;
