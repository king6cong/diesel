mod backend;
mod connection;
mod types;

pub mod query_builder;

use std::sync::Arc;
pub use self::backend::{Sqlite, SqliteType};
pub use self::connection::SqliteConnection;
pub use self::query_builder::SqliteQueryBuilder;

pub trait ErrorHook {
    fn on_error(&self, code: i32);
}

static mut ERROR_HOOK: Option<Box<Arc<ErrorHook>>> = None;

fn on_error(code: i32) {
    unsafe {
        if let Some(ref hook) = ERROR_HOOK {
            hook.on_error(code);
        }
    }
}

/// init error hook, must init first
pub fn init_error_hook(err_hook: Box<Arc<ErrorHook>>) {
    unsafe {
        ERROR_HOOK = Some(err_hook);
    }
}
