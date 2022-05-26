// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.

//! Contains System tables, such as system.public.tables

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use catalog::{
    consts::{SYSTEM_CATALOG, SYSTEM_CATALOG_SCHEMA},
    schema::{CreateOptions, DropOptions, NameRef, Schema, SchemaRef},
    Catalog,
};
use system_catalog::SystemTableAdapter;
use table_engine::{
    engine::{CreateTableRequest, DropTableRequest},
    table::{Table, TableId, TableRef},
};

const UNSUPPORTED_MSG: &str = "system tables not supported";

pub struct SystemTablesBuilder {
    tables: HashMap<String, Arc<SystemTableAdapter>>,
}

impl SystemTablesBuilder {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn insert_table(mut self, table: SystemTableAdapter) -> Self {
        self.tables
            .insert(table.name().to_string(), Arc::new(table));
        self
    }

    pub fn build(self) -> SystemTables {
        SystemTables::new(self.tables)
    }
}

#[derive(Clone)]
pub struct SystemTables {
    tables: Arc<HashMap<String, Arc<SystemTableAdapter>>>,
}

impl SystemTables {
    pub fn new(tables: HashMap<String, Arc<SystemTableAdapter>>) -> Self {
        Self {
            tables: Arc::new(tables),
        }
    }
}

#[async_trait]
impl Schema for SystemTables {
    fn name(&self) -> NameRef {
        SYSTEM_CATALOG_SCHEMA
    }

    fn table_by_name(&self, name: NameRef) -> catalog::schema::Result<Option<TableRef>> {
        Ok(self.tables.get(name).map(|v| v.clone() as TableRef))
    }

    fn alloc_table_id(&self, _name: NameRef) -> catalog::schema::Result<TableId> {
        catalog::schema::UnSupported {
            msg: UNSUPPORTED_MSG,
        }
        .fail()
    }

    async fn create_table(
        &self,
        _request: CreateTableRequest,
        _opts: CreateOptions,
    ) -> catalog::schema::Result<TableRef> {
        catalog::schema::UnSupported {
            msg: UNSUPPORTED_MSG,
        }
        .fail()
    }

    async fn drop_table(
        &self,
        _request: DropTableRequest,
        _opts: DropOptions,
    ) -> catalog::schema::Result<bool> {
        catalog::schema::UnSupported {
            msg: UNSUPPORTED_MSG,
        }
        .fail()
    }

    fn all_tables(&self) -> catalog::schema::Result<Vec<TableRef>> {
        Ok(self
            .tables
            .iter()
            .map(|(_, v)| v.clone() as TableRef)
            .collect())
    }
}

#[async_trait]
impl Catalog for SystemTables {
    fn name(&self) -> NameRef {
        SYSTEM_CATALOG
    }

    fn schema_by_name(&self, name: NameRef) -> catalog::Result<Option<SchemaRef>> {
        if name == SYSTEM_CATALOG_SCHEMA {
            Ok(Some(Arc::new(self.clone())))
        } else {
            Ok(None)
        }
    }

    async fn create_schema<'a>(&'a self, _name: NameRef<'a>) -> catalog::Result<()> {
        catalog::UnSupported {
            msg: UNSUPPORTED_MSG,
        }
        .fail()
    }

    fn all_schemas(&self) -> catalog::Result<Vec<SchemaRef>> {
        catalog::UnSupported {
            msg: UNSUPPORTED_MSG,
        }
        .fail()
    }
}
