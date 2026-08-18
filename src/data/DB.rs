pub mod db{

use std::fs;

use chrono::{DateTime, NaiveDateTime, Utc};

use mysql::{
    params,
    prelude::*,
    Pool,
    PooledConn,
    TxOpts,
};

use uuid::Uuid;

#[derive(Debug)]
pub struct Database {
    pool: Pool,
}

impl Database {

    pub fn new(url: &str) -> mysql::Result<Self> {
        Ok(Self {pool: Pool::new(url)?})
    }

    fn conn(&self) -> mysql::Result<PooledConn> {
        self.pool.get_conn()
    }

    pub fn init_dbs(&self) -> mysql::Result<()> {
        let mut conn = self.conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;
        for i in fs::read_to_string("~/RaieLZ/src/data/init.sql")?.split(";"){
            let i = i.trim();
            if !i.is_empty(){
                tx.query_drop(i)?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn clear(&self) -> mysql::Result<()>{
        let mut conn = self.conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;
            tx.exec_drop("DROP TABLE accounts;",())?;
            tx.exec_drop("DROP TABLE folders;",())?;
            tx.exec_drop("DROP TABLE threads;",())?;
            tx.exec_drop("DROP TABLE emails;",())?;
            tx.exec_drop("DROP TABLE attachments;",())?;
        tx.commit()?;
        Ok(())
    }

    



}







}