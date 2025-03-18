pub struct CostMatrix {
    data: Vec<usize>,
    size: usize,
}

impl CostMatrix {
    pub fn new(size: usize) -> CostMatrix {
        let vector_length = size.pow(2);

        CostMatrix {
            data: vec![0; vector_length],
            size,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (self.size * y) + x
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        self.data[self.get_index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, cost: usize) {
        let index = self.get_index(x, y);

        self.data[index] = cost;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}