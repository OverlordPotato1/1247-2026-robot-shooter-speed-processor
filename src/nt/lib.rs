

// pub use rmpv::{self, Value};

#[inline(always)]
pub fn log_result<T>(result: anyhow::Result<T>) -> anyhow::Result<T> {
    match &result {
        Err(err) => {
            tracing::error!("{}", err)
        }
        _ => {}
    };
    result
}
