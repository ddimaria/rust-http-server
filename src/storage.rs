use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Storage (HashMap<String, Vec<StorageData>>);

#[derive(Clone, Debug, PartialEq)]
pub struct StorageData {
    pub body: Option<String>,
    pub content_type: String,
    pub content_length: usize,
}

impl Storage {
    pub fn new() -> Storage {
        Storage (HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
