pub struct PipelineObjectSet {
    pipelines: HashMap<BlendMode, PipelineState>,
}


// This structure is to store multiple pipelines for different blend modes.
impl PipelineObjectSet {
    pub fn new(cap: usize) -> Self {
        Self {
            pipelines: HashMap::with_capacity(cap),
        }
    }
}

pub struct ShaderGeneric {
    
}