pub struct Tensor<T> {
    dims: usize,
    shape: Vec<usize>,
    strides: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone + Copy> Tensor<T> {
    pub fn new(shape: Vec<usize>, initial_value: T) -> Self {
        let dims = shape.len();

        let mut strides = Vec::new();
        let mut current_stride = 1;
        for shape_ in shape.iter() {
            strides.push(current_stride);
            current_stride *= shape_;
        }

        let mut data = Vec::new();
        data.resize(current_stride, initial_value);

        Tensor {
            dims,
            shape,
            strides,
            data,
        }
    }

    fn get_index_1d(&self, index: Vec<usize>) -> usize {
        let mut index_1d = 0;
        for (i, stride) in index.iter().zip(self.strides.iter()) {
            index_1d += i * stride;
        }
        index_1d
    }

    pub fn at(&self, index: Vec<usize>) -> T {
        let index_1d = self.get_index_1d(index);
        self.data[index_1d]
    }

    pub fn set(&mut self, index: Vec<usize>, v: T) {
        let index_1d = self.get_index_1d(index);
        self.data[index_1d] = v;
    }

    pub fn get_dims(&self) -> usize {
        self.dims
    }

    pub fn get_shape(&self) -> Vec<usize> {
        self.shape.clone()
    }
}
