pub trait Table: Copy {
    const NAME: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Column<T> {
    pub name: &'static str,
    pub table: std::marker::PhantomData<T>,
}

impl<T: Table> std::fmt::Display for Column<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`.`{}`", T::NAME, self.name)
    }
}

impl<T> Column<T> {
    #[inline]
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            table: std::marker::PhantomData,
        }
    }
}
