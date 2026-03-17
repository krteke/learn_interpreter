pub struct ValueArray {
    pub values: Vec<f64>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}
