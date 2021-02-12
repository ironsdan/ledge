// Inspired by hibitset and amethyst this is a hierarchial bitset that is used for speedy joining.
const USIZE_BITS: usize = std::mem::size_of::<usize>() * 8;
const LAYER_FACTOR: usize = 4;
const LAYER1: usize = USIZE_BITS / LAYER_FACTOR;

#[derive(Default)]
pub struct LayeredBitMap {
    pub layer1: Vec<usize>,
    pub layer0: Vec<usize>,
    max_index: usize,
}

impl LayeredBitMap {
    pub fn new() -> Self {
        Self {
            layer1: Vec::new(),
            layer0: Vec::new(),
            max_index: 0,
        }
    }

    pub fn insert(&mut self, index: usize) {
        if index > self.max_index {
            for _ in 0..((index / USIZE_BITS) + 1 - self.layer0.len()) {
                self.layer0.push(0);
            }
            for _ in 0..((index / LAYER1) + 1 - self.layer1.len()) {
                self.layer1.push(0);
            }
            self.max_index = index;
        }
        self.layer0[index / USIZE_BITS] |= 1 << (index % USIZE_BITS);
        self.layer1[index / LAYER1] |= 1 << (index % USIZE_BITS) + (LAYER_FACTOR - 1) - (index % LAYER_FACTOR);
    }

    pub fn remove(&mut self, index: usize) {
        self.layer0[index / USIZE_BITS] = self.layer0[index / USIZE_BITS] ^ 1 << (index % USIZE_BITS);
    }

    pub fn check(&self, index: usize) -> bool {
        if index > self.max_index {
            return false;
        }
        if self.layer1[index / LAYER1] & (1 << (index % USIZE_BITS) + ((LAYER_FACTOR - 1) - (index % LAYER_FACTOR))) != 0  {
            println!("layer1 check: true");
        }
        (self.layer0[index / USIZE_BITS] & (1 << (index % USIZE_BITS))) != 0
    }
}