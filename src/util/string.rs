use std::{ops::Deref, sync::Arc};

// #[derive(Default)]
// pub struct InternedString(pub Arc<String>);

pub type InternedString = Arc<String>;

// impl Deref for InternedString {
//     type Target = String;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &*self.0
//     }
// }
