use actix_web::Result;
use diesel::connection::{TransactionManager, AnsiTransactionManager};
use crate::PgPooledConnection;
use crate::error::{bad_request, conflict};

pub mod task;
pub mod user;
pub mod session;
pub mod reward;

// Run a function inside of an sql transaction.
// If it returns an error, rollback, otherwise commit
pub fn atomically<T, F>(conn: &PgPooledConnection, updates: F) -> Result<T>
    where F: FnOnce() -> Result<T>
{
    // Start up a new transaction manager so the updates are atomic
    let transaction_manager = AnsiTransactionManager::new();
    if transaction_manager.begin_transaction(conn).is_err() {
        return Err(bad_request("Could not begin atomic transaction".to_string()));
    }
    let result = updates();
    if result.is_ok() {
        if transaction_manager.commit_transaction(conn).is_err() {
            return Err(conflict("Could not complete atomic transaction".to_string()));
        }
    } else {
        if transaction_manager.rollback_transaction(conn).is_err() {
            return Err(conflict("Could not rollback atomic transaction".to_string()));
        }
    }
    // Always return the result, even if it's an error
    result
}