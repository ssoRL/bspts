use actix_web::Result;
use diesel::connection::{TransactionManager, AnsiTransactionManager};
use crate::PgPooledConnection;

pub mod task;
pub mod user;
pub mod session;

// Run a function inside of an sql transaction.
// If it returns an error, rollback, otherwise commit
pub fn atomically<T, F>(conn: &PgPooledConnection, updates: F) -> Result<T>
    where F: FnOnce() -> Result<T>
{
    // Start up a new transaction manager so the updates are atomic
    let transaction_manager = AnsiTransactionManager::new();
    transaction_manager.begin_transaction(conn);
    let result = updates();
    if result.is_ok() {
        transaction_manager.commit_transaction(conn);
    } else {
        transaction_manager.rollback_transaction(conn);
    }
    // Always return the result, even if it's an error
    result
}