#[derive(Debug, Clone)]
pub enum Name {
    Haylin,
}
impl Name {
    pub fn uppercase_str(&self) -> &'static str {
        match self {
            Name::Haylin => "Haylin",
        }
    }
    pub fn uppercase_full_str(&self) -> &'static str {
        match self {
            Name::Haylin => "Haylin Moore",
        }
    }
    pub fn domain(&self) -> &'static str {
        match self {
            Name::Haylin => "hayl.in",
        }
    }
}
