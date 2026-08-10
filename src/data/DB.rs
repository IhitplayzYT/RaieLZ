pub mod db{

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

        Ok(())
    }

    pub fn clear(&self) -> mysql::Result<()>{
        let mut conn = self.conn()?;
        let mut tx = conn.start_transaction(TxOpts::default())?;
            tx.exec_drop("DROP TABLE Calendar;",())?;
            tx.exec_drop("DROP TABLE Journal_task_tags;",())?;
            tx.exec_drop("DROP TABLE Journal_tasks;",())?;
            tx.exec_drop("DROP TABLE Ledger;",())?;
            tx.exec_drop("DROP TABLE Note_task_tags;",())?;
            tx.exec_drop("DROP TABLE Note_tasks;",())?;
            tx.exec_drop("DROP TABLE Todo_task_tags;",())?;
            tx.exec_drop("DROP TABLE Todo_tasks;",())?;
            tx.exec_drop("DROP TABLE tags;",())?;
        tx.commit()?;
        Ok(())
    }

}







}