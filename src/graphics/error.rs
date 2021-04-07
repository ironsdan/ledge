#[derive(Debug)]
pub enum GraphicsError {
    PipelineError(String),
    DrawError(String),
}