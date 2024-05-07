use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

use dashmap::DashSet;

// unimplemented!(InlinedVector);

#[derive(Debug)]
pub struct InternCache<T: Eq + Hash>(DashSet<Arc<T>>);

impl<T: Eq + Hash + Clone> InternCache<T> {
    pub fn new() -> Self {
        Self(DashSet::with_capacity(256))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn lookup(&self, item: &T) -> Arc<T> {
        if !self.0.contains(item) {
            self.0.insert(Arc::new(item.clone()));
        }
        Arc::clone(&self.0.get(item).unwrap())
    }
}

// impl<T: Display + Eq + Hash> ToString for InternCache<T> {
//     fn to_string(&self) -> String {
//         self.0
//             .iter()
//             .map(|ptr| ptr.to_string())
//             .collect::<Vec<String>>()
//             .join("\n")
//     }
// }

mod test {
    #[test]
    fn intern_cache_base_string() {
        use super::InternCache;
        let cache = InternCache::new();

        let one = cache.lookup(&String::from("one"));
        assert_eq!(1, cache.len());
        assert_eq!("one", *one);

        let two = cache.lookup(&String::from("two"));
        assert_eq!(2, cache.len());
        assert_ne!(one, two);
        assert_eq!("two", *two);

        let another_one = cache.lookup(&String::from("one"));
        assert_eq!(2, cache.len());
        assert_eq!("one", *another_one);
        assert_eq!(one, another_one);
    }
}
