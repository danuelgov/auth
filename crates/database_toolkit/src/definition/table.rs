#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Table(pub &'static str);

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
