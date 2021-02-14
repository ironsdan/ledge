// Inspired by hibitset and amethyst this is a hierarchial bitset that is used for speedy joining.
const USIZE_BITS: usize = std::mem::size_of::<usize>() * 8;
const LAYER_FACTOR: usize = 4;
const LAYER1: usize = USIZE_BITS * LAYER_FACTOR;

#[derive(Default)]
pub struct LayeredBitMap {
    pub layer1: Vec<usize>,
    pub layer0: Vec<usize>,
}

impl LayeredBitMap {
    pub fn new() -> Self {
        Self {
            layer1: Vec::new(),
            layer0: Vec::new(),
        }
    }

    pub fn insert(&mut self, index: usize) {
        if (index / USIZE_BITS) + 1 > self.layer0.len() {
            for _ in 0..((index / USIZE_BITS) + 1 - self.layer0.len()) {
                self.layer0.push(0);
            }
            for _ in 0..((index / LAYER1) + 1 - self.layer1.len()) {
                self.layer1.push(0);
            }
        }
        self.layer0[index / USIZE_BITS] |= 1 << (index % USIZE_BITS);
        self.layer1[index / (USIZE_BITS*LAYER_FACTOR)] |= 1 << (index / LAYER_FACTOR);
    }

    pub fn remove(&mut self, index: usize) {
        self.layer0[index / USIZE_BITS] &= !(1 << (index % USIZE_BITS));
               
        let mut value = 0;
        let mut test;
        let mut curr_index = index;
        loop {
            test = curr_index % LAYER_FACTOR;
            value |= self.layer0[curr_index / USIZE_BITS] & 1 << (curr_index % USIZE_BITS);
            curr_index -= 1;
            if test == 0 || value != 0 {break}
        }
        curr_index = index;
        loop {
            curr_index += 1;
            test = curr_index % LAYER_FACTOR;
            if test == 0 || value != 0 {break} 
            value |= self.layer0[curr_index / USIZE_BITS] & 1 << (curr_index % USIZE_BITS);
        }
        if value != 0 {
            value = 1;
        }
        self.layer1[index / LAYER1] = self.layer1[index / LAYER1] & !(!value << (index / LAYER_FACTOR));
    }

    pub fn check(&self, index: usize) -> bool {
        if (index / USIZE_BITS) > self.layer0.len() {
            println!("failed due to bounds check");
            return false;
        }
        if !self.layer1[index / LAYER1] & (1 << (index / LAYER_FACTOR)) != 0  {
            println!("failed due to layer 1 check");
            return false;
        }
        (self.layer0[index / USIZE_BITS] & (1 << (index % USIZE_BITS))) != 0
    }

    pub fn len(&self) -> usize {
        self.layer0.len()
    }

    pub fn join(lhs: Self, rhs: Self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut looper = &lhs;
        let mut other = &rhs;
        let mut both;

        if rhs.len() < lhs.len() {
            looper = &rhs;
            other = &lhs;
        }

        let mut curr_index = 0;

        for i in 0..looper.len() {
            both = looper.layer0[i] & other.layer0[i];

            for j in curr_index..curr_index + USIZE_BITS {
                if both & 1 << (j % USIZE_BITS) != 0 {
                    result.push(j);
                }
            }
            curr_index = curr_index + USIZE_BITS;
        }

        result
    }
}