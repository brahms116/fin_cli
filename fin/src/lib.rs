mod service;
mod entities;
mod repository;
mod string_err;

pub use service::*;
use entities::prelude::*;
use entities::*;
use sea_orm::entity::prelude::*;
pub use string_err::*;

impl From<DbErr> for StringErr {
    fn from(e: DbErr) -> Self {
        StringErr::new(format!("{:?}", e))
    }
}
