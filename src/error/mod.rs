pub use std::error::Error;
//pub type DynErr = Box<dyn Error>;
//pub type DResult<T> = Result<T, DynErr>;

pub use anyhow::{anyhow as fmt_error,
                 Context as IErrorContext,
                 Result as DResult};
