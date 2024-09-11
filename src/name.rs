use std::env;

#[derive(Debug, Clone)]
pub enum Name {
    Haylin
}
impl Name {
    pub fn from_env() -> Name {
        match env::var("NAME").as_deref() {
            _ => Name::Haylin,
        }
    }
    pub fn uppercase_str(&self) -> &'static str {
        match self {
            Name::Haylin => "Haylin"
        }
    }
    pub fn uppercase_full_str(&self) -> &'static str {
        match self {
            Name::Haylin => "Haylin Moore"
        }
    }
    pub fn domain(&self) -> &'static str {
        match self {
            Name::Haylin => "haylinmoore.com"
        }
    }
}