use crate::asset::handle::HandleId;
use crate::asset::handle::Handle;
use crate::asset::Asset;
use std::collections::HashMap;

pub struct AssetStorage<A> {
    assets: HashMap<HandleId, A>,
}

impl<A: Asset> AssetStorage<A> {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new()
        }
    }

    pub fn insert(&mut self, asset: A) -> Handle<A> {
        let id = HandleId::random();
        self.assets.insert(id.clone(), asset);
        Handle::from(id)
    }

    pub fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&A> {
        self.assets.get(&handle.into())
    }

    // pub fn get_handle() -> Option<Handle<A>> {

    // }
}