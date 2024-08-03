#[derive(Debug, Clone)]
pub struct Namespace {
    pub parts: Vec<String>,
}

impl Namespace {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn add_part(&mut self, part: &str) {
        self.parts.push(part.to_string());
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

impl ToString for Namespace {
    fn to_string(&self) -> String {
        self.parts.join("_")
    }
}
