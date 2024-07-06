use std::env;

#[derive(Debug, Clone)]
pub enum Name {
    Hampton,
    Haylin
}
impl Name {
    pub fn from_env() -> Name {
        match env::var("NAME").as_deref() {
            Ok("Hampton") => Name::Hampton,
            _ => Name::Haylin,
        }
    }
    pub fn uppercase_str(&self) -> &'static str {
        match self {
            Name::Hampton => "Hampton",
            Name::Haylin => "Haylin"
        }
    }
    pub fn uppercase_full_str(&self) -> &'static str {
        match self {
            Name::Hampton => "Hampton Moore",
            Name::Haylin => "Haylin Moore"
        }
    }
    pub fn lowercase_str(&self) -> &'static str {
        match self {
            Name::Hampton => "hampton",
            Name::Haylin => "haylin"
        }
    }
    pub fn domain(&self) -> &'static str {
        match self {
            Name::Hampton => "hamptonmoore.com",
            Name::Haylin => "haylinmoore.com"
        }
    }
}