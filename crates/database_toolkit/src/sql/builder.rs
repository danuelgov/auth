use crate::{Column, Numeric, Table};
use sqlx::{mysql::MySqlArguments, query::Query, Encode, MySql, Type};

pub struct QueryBuilder<'args> {
    inner: sqlx::QueryBuilder<'args, MySql>,
}

impl<'args> std::ops::Deref for QueryBuilder<'args> {
    type Target = sqlx::QueryBuilder<'args, MySql>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'args> std::ops::DerefMut for QueryBuilder<'args> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'args> QueryBuilder<'args> {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: sqlx::QueryBuilder::new(""),
        }
    }

    #[inline]
    pub fn write<T: std::fmt::Display>(mut self, text: T) -> Self {
        self.inner.push(text);
        self.inner.push(" ");
        self
    }

    #[inline]
    pub fn f<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }

    #[inline]
    pub fn select(self) -> Self {
        self.write("SELECT")
    }

    #[inline]
    pub fn insert_into(self, table: Table, columns: &[Column]) -> Self {
        self.write("INSERT INTO")
            .table(table)
            .nested(|builder| builder.columns(columns))
    }

    #[inline]
    pub fn update(self, table: Table) -> Self {
        self.write("UPDATE").table(table)
    }

    #[inline]
    pub fn delete_from(self, table: Table) -> Self {
        self.write("DELETE FROM").table(table)
    }

    #[inline]
    pub fn from(self, table: Table) -> Self {
        self.write("FROM").table(table)
    }

    #[inline]
    pub fn set(self) -> Self {
        self.write("SET")
    }

    #[inline]
    pub fn where_<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.write("WHERE").f(f)
    }

    #[inline]
    pub fn and(self) -> Self {
        self.write("AND")
    }

    #[inline]
    pub fn or(self) -> Self {
        self.write("OR")
    }

    #[inline]
    pub fn limit(self, limit: u64) -> Self {
        self.write("LIMIT").write(limit)
    }

    #[inline]
    pub fn offset(self, offset: u64) -> Self {
        self.write("OFFSET").write(offset)
    }

    #[inline]
    pub fn limit_offset(self, limit: u64, offset: u64) -> Self {
        self.limit(limit).offset(offset)
    }

    #[inline]
    pub fn order_by(self, column: Column) -> Self {
        self.write("ORDER BY").column(column)
    }

    #[inline]
    pub fn asc(self) -> Self {
        self.write("ASC")
    }

    #[inline]
    pub fn desc(self) -> Self {
        self.write("DESC")
    }

    #[inline]
    pub fn on(self) -> Self {
        self.write("ON")
    }

    #[inline]
    pub fn join(self, table: Table) -> Self {
        self.write("JOIN").table(table)
    }

    #[inline]
    pub fn comma(self) -> Self {
        self.write(", ")
    }

    #[inline]
    pub fn semicolon(self) -> Self {
        self.write(";")
    }

    #[inline]
    pub fn value<T>(mut self, value: T) -> Self
    where
        for<'q> T: 'q + Send + Encode<'q, MySql> + Type<MySql>,
    {
        self.inner.push_bind(value);
        self.inner.push(" ");
        self
    }

    #[inline]
    pub fn values<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.write("VALUES").f(f)
    }

    #[inline]
    pub fn placeholder(self) -> Self {
        self.write("?")
    }

    #[inline]
    pub fn symbol(mut self, name: &str) -> Self {
        self.inner.push("`");
        self.inner.push(name);
        self.inner.push("` ");
        self
    }

    #[inline]
    pub fn table(self, name: Table) -> Self {
        self.symbol(name.0)
    }

    #[inline]
    pub fn column(self, column: Column) -> Self {
        self.symbol(column.0)
    }

    #[inline]
    pub fn alias_column(self, column: Column, alias: &str) -> Self {
        self.column(column).alias(alias)
    }

    #[inline]
    pub fn condition<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.nested(f)
    }

    #[inline]
    pub fn assign_value<T>(self, column: Column, value: T) -> Self
    where
        for<'q> T: 'q + Send + Encode<'q, MySql> + Type<MySql>,
    {
        self.column(column).eq().value(value)
    }

    #[inline]
    pub fn eq(self) -> Self {
        self.write("=")
    }

    #[inline]
    pub fn ne(self) -> Self {
        self.write("<>")
    }

    #[inline]
    pub fn lt(self) -> Self {
        self.write("<")
    }

    #[inline]
    pub fn le(self) -> Self {
        self.write("<=")
    }

    #[inline]
    pub fn gt(self) -> Self {
        self.write(">")
    }

    #[inline]
    pub fn ge(self) -> Self {
        self.write(">=")
    }

    #[inline]
    pub fn between(self, left: &str, right: &str) -> Self {
        self.write("BETWEEN").write(left).and().write(right)
    }

    #[inline]
    pub fn alias(self, name: &str) -> Self {
        self.write("AS").symbol(name)
    }

    #[inline]
    pub fn null(self) -> Self {
        self.write("NULL")
    }

    #[inline]
    pub fn not_null(self) -> Self {
        self.write("NOT NULL")
    }

    #[inline]
    pub fn is_null(self) -> Self {
        self.write("IS NULL")
    }

    #[inline]
    pub fn is_not_null(self) -> Self {
        self.write("IS NOT NULL")
    }

    #[inline]
    pub fn true_(self) -> Self {
        self.write("TRUE")
    }

    #[inline]
    pub fn false_(self) -> Self {
        self.write("FALSE")
    }

    #[inline]
    pub fn boolean(self, value: bool) -> Self {
        self.write(value)
    }

    #[inline]
    pub fn numeric<T: Numeric>(self, value: T) -> Self {
        self.write(value)
    }

    #[inline]
    pub fn string(self, value: &str) -> Self {
        self.write("\"").write(value).write("\"")
    }

    #[inline]
    pub fn nested<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.write("(").f(f).write(")")
    }

    #[inline]
    pub fn separated<T, F>(self, mut iter: impl Iterator<Item = T>, f: F) -> Self
    where
        F: Fn(Self, T) -> Self,
    {
        match iter.next() {
            Some(item) => iter.fold(f(self, item), |builder, item| {
                builder.comma().f(|builder| f(builder, item))
            }),
            None => self,
        }
    }

    #[inline]
    pub fn columns(self, columns: &[Column]) -> Self {
        self.separated(columns.iter(), |builder, &column| builder.column(column))
    }

    #[inline]
    pub fn build(&mut self) -> Query<MySql, MySqlArguments> {
        self.inner.build()
    }
}
