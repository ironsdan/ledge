pub trait Joinable {
    type Value;
    type Type;

    fn join(self) -> JoinIterator<Self>
    where
        Self: Sized,
    {
        JoinIterator::new(self)
    }

    fn view(self) -> (Vec<usize>, Self::Value);
    unsafe fn get(value: &mut Self::Value, index: usize) -> Self::Type;
}

pub struct JoinIterator<J: Joinable> {
    // types: J::Type,
    keys: Box<dyn Iterator<Item = usize>>,
    values: J::Value,
}

impl<J: Joinable> JoinIterator<J> {
    pub fn new(joinable: J) -> Self {
        let (keys, values) = joinable.view();
        let keys = Box::new(keys.into_iter());
        Self {
            keys,
            values,
        }
    }
}

impl<J: Joinable> Iterator for JoinIterator<J> {
    type Item = J::Type;
    fn next(&mut self) -> Option<J::Type> {
        unsafe { self.keys.next().map(|index| J::get(&mut self.values, index)) }
    }
}