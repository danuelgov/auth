use crate::{
    Column, ColumnName, IdentityType, Key, Known, KnownName, Schema, StringType, TableName,
    TableSchema,
};
use std::collections::HashMap;

#[allow(unused_variables)]
pub trait Visitor: Sized {
    fn visit_schema(&mut self, schema: &Schema) -> Result<(), std::io::Error> {
        visit_schema(self, schema)
    }

    fn visit_table_schema(&mut self, table_schema: &TableSchema) -> Result<(), std::io::Error> {
        visit_table_schema(self, table_schema)
    }

    fn visit_key(&mut self, key: &Key) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_known(&mut self, name: &KnownName, known: &Known) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_columns(
        &mut self,
        columns: &HashMap<ColumnName, Column>,
    ) -> Result<(), std::io::Error> {
        visit_columns(self, columns)
    }

    fn visit_column(&mut self, name: &ColumnName, column: &Column) -> Result<(), std::io::Error> {
        visit_column(self, name, column)
    }

    fn visit_primary_key_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_identity_type(
        &mut self,
        name: &ColumnName,
        identity_type: &IdentityType,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_string_type(
        &mut self,
        name: &ColumnName,
        string_type: &StringType,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_date_time_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_handle_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_hash_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_salt_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_json_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_image_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_ip_addr_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_table_name(&mut self, table_name: &TableName) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn visit_knowns(&mut self, knowns: &HashMap<KnownName, Known>) -> Result<(), std::io::Error> {
        visit_knowns(self, knowns)
    }
}

pub fn visit_schema<V: Visitor>(visitor: &mut V, schema: &Schema) -> Result<(), std::io::Error> {
    visitor.visit_table_schema(&schema.table)?;
    visitor.visit_columns(&schema.columns)?;
    if let Some(knowns) = &schema.knowns {
        visitor.visit_knowns(knowns)?;
    }
    Ok(())
}

pub fn visit_table_schema<V: Visitor>(
    visitor: &mut V,
    table_schema: &TableSchema,
) -> Result<(), std::io::Error> {
    visitor.visit_table_name(&table_schema.name)
}

pub fn visit_columns<V: Visitor>(
    visitor: &mut V,
    columns: &HashMap<ColumnName, Column>,
) -> Result<(), std::io::Error> {
    let mut columns: Vec<_> = columns.iter().collect();
    columns.sort_by_key(|(name, _)| name.to_owned());
    for (name, column) in columns {
        visitor.visit_column(name, column)?;
    }

    Ok(())
}

pub fn visit_column<V: Visitor>(
    visitor: &mut V,
    name: &ColumnName,
    column: &Column,
) -> Result<(), std::io::Error> {
    match column {
        Column::PrimaryKey => visitor.visit_primary_key_type(name),
        Column::Identity(identity) => visitor.visit_identity_type(name, identity),
        Column::String(string) => visitor.visit_string_type(name, string),
        Column::DateTime => visitor.visit_date_time_type(name),
        Column::Handle => visitor.visit_handle_type(name),
        Column::Hash => visitor.visit_hash_type(name),
        Column::Salt => visitor.visit_salt_type(name),
        Column::Json => visitor.visit_json_type(name),
        Column::Image => visitor.visit_image_type(name),
        Column::IpAddr => visitor.visit_ip_addr_type(name),
    }
}

pub fn visit_knowns<V: Visitor>(
    visitor: &mut V,
    knowns: &HashMap<KnownName, Known>,
) -> Result<(), std::io::Error> {
    let mut knowns: Vec<_> = knowns.iter().collect();
    knowns.sort_by_key(|(name, _)| name.to_owned());
    for (name, known) in knowns {
        visitor.visit_known(name, known)?;
    }

    Ok(())
}
