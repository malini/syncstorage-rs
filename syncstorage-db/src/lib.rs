//! Generic db abstration.

#[cfg(test)]
#[macro_use]
extern crate slog_scope;

pub mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "postgres")]
pub type DbPoolImpl = syncstorage_postgres::PgDbPool;
#[cfg(feature = "postgres")]
pub use syncstorage_postgres::DbError;
#[cfg(feature = "postgres")]
pub type DbImpl = syncstorage_postgres::PgDb;

#[cfg(feature = "mysql")]
pub type DbPoolImpl = syncstorage_mysql::MysqlDbPool;
#[cfg(feature = "mysql")]
pub use syncstorage_mysql::DbError;
#[cfg(feature = "mysql")]
pub type DbImpl = syncstorage_mysql::MysqlDb;

#[cfg(feature = "spanner")]
pub type DbPoolImpl = syncstorage_spanner::SpannerDbPool;
#[cfg(feature = "spanner")]
pub use syncstorage_spanner::DbError;
#[cfg(feature = "spanner")]
pub type DbImpl = syncstorage_spanner::SpannerDb;

pub use syncserver_db_common::{GetPoolState, PoolState};
pub use syncstorage_db_common::error::DbErrorIntrospect;

pub use syncstorage_db_common::{
    params, results,
    util::{to_rfc3339, SyncTimestamp},
    Db, DbPool, Sorting, UserIdentifier,
};

#[cfg(all(feature = "postgres", feature = "mysql", feature = "spanner"))]
compile_error!("only one of the \"postgres\", \"mysql\" and \"spanner\" features can be enabled at a time");

#[cfg(not(any(feature = "postgres", feature = "mysql", feature = "spanner")))]
compile_error!("exactly one of the \"postgres\", \"mysql\" and \"spanner\" features must be enabled");
