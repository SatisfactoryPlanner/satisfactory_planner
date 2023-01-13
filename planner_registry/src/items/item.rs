#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub name: &'static str,
    pub raw: bool,
}
