mod futures;

pub mod mock;

pub use self::futures::await;

use tokio;
use futures::Future;
use std::io;
