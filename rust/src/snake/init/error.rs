use core::fmt;

#[derive(Debug)]
pub enum InitError {
    GetWindowError,
    GetDocumentError,
    GetRootError,
    CreateCanvasError,
    AppendCanvasError,
    GetContextError,
    ScaleCanvasError,
}

impl std::error::Error for InitError {}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InitError::GetWindowError => write!(f, "Failed to get window object"),
            InitError::GetDocumentError => write!(f, "Failed to get document object"),
            InitError::GetRootError => write!(f, "Failed to get root element with given root_id"),
            InitError::CreateCanvasError => write!(f, "Failed to create the canvas element"),
            InitError::AppendCanvasError => {
                write!(f, "Failed to append the canvas to the root element")
            }
            InitError::GetContextError => {
                write!(f, "Failed to get the context for the canvas (2d)")
            }
            InitError::ScaleCanvasError => {
                write!(f, "Failed to scale the canvas to its maximum size")
            }
        }
    }
}
