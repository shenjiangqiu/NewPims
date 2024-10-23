use smallvec::SmallVec;

pub enum TensorType {
    Weight,
    Activation,
    KVCache,
}

pub struct Tensor {
    pub base_addr: usize,
    pub shape: SmallVec<[usize; 4]>,
    pub size: usize,
    pub tensor_type: TensorType,
}

impl Tensor {
    pub fn new(dims: &[usize], tensor_type: TensorType) -> Self {
        let size = dims.iter().product();
        
        Tensor {
            base_addr: 0,
            shape: dims.into(),
            size,
            tensor_type,
        }
    }
}
