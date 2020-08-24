
use crate::lib::{HashMap, Vec};
use std::hash::Hash;
use std::iter::Iterator;

/// Return a `HashMap` of keys mapped to a list of their corresponding values.
///
/// See [`.into_group_map()`](../trait.Itertools.html#method.into_group_map)
/// for more information.
pub fn into_group_map<I, K, V>(iter: I) -> HashMap<K, Vec<V>>
    where I: Iterator<Item=(K, V)>,
          K: Hash + Eq,
{
    let mut lookup = HashMap::new();

    for (key, val) in iter {
        lookup.entry(key).or_insert(Vec::new()).push(val);
    }

    lookup
}

#[cfg(feature = "use_std")]
pub fn into_group_map_by<I, K, V>(iter: I, f: impl Fn(&V) -> K) -> HashMap<K, Vec<V>>
    where
        I: Iterator<Item=V>,
        K: Hash + Eq,
{
    into_group_map(
        iter.map(|v| (f(&v), v))
    )
}


