// use deadpool_postgres::{Client, Pool};

// use crate::errors::MyError;

// pub(crate) trait DBPoolGetter {
//     fn get_client(&self) -> Result<Client, MyError>;
// }

// impl DBPoolGetter for Pool {
//     fn get_client(&self) -> Result<Client, MyError> {
//         self.get().await.map_err(MyError::PoolError)
//     }
// }