pub mod handle;
pub mod storage;
pub mod types;

pub trait Asset: 'static {}

impl<T> Asset for T where T: 'static {}