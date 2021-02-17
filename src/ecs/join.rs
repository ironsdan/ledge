pub trait Joinable {
    type Value;
    type Type;

    fn join<J: Joinable>(self) -> JoinIterator<Self>
    where
        Self: Sized,
    {
        JoinIterator::new(self)
    }

    fn get_values(&self) -> Self::Value;
    fn get_keys(&self) -> Vec<usize>;
    fn get(value: &mut Self::Value, index: usize) -> Self::Type;
}

pub struct JoinIterator<J: Joinable> {
    // types: J::Type,
    keys: Box<dyn Iterator<Item = usize>>,
    values: J::Value,
}

impl<J: Joinable> JoinIterator<J> {
    pub fn new(joinable: J) -> Self {
        let values = joinable.get_values();
        let keys = Box::new(joinable.get_keys().into_iter());
        Self {
            keys,
            values,
        }
    }
}

impl<J: Joinable> Iterator for JoinIterator<J> {
    type Item = J::Type;
    fn next(&mut self) -> Option<J::Type> {
        self.keys.next().map(|index| J::get(&mut self.values, index))
    }
}