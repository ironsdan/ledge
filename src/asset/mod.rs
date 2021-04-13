pub mod handle;
pub mod storage;
pub mod types;
pub mod world;

pub trait Asset: 'static {}

impl<T> Asset for T where T: 'static {}