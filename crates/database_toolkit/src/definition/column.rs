#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Column(pub &'static str);

impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
