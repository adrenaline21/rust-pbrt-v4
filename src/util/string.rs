use std::sync::Arc;

#[derive(Default)]
pub struct InternedString(pub Arc<String>);
