pub struct Local {
    pub name: String,
    pub depth: i32,
}

impl Local {
    pub fn new(name: String, depth: i32) -> Self {
        Self { name, depth }
    }
}
