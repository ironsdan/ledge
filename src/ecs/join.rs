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
}

pub struct JoinIterator<J: Joinable> {
    values: J::Value,
}

impl<J: Joinable> JoinIterator<J> {
    pub fn new(joinable: J) -> Self {
        let values = joinable.get_values();
        Self {
            values,
        }
    }
}

// impl<J: Joinable> Iterator for JoinIterator<J> {
//     type Item = J::Type;
//     fn next(&mut self) -> Option<J::Type> {
//         Some(self.values)
//     }
// }