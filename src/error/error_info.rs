use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub file_name: Rc<PathBuf>,
    pub line_content: Rc<String>,
    pub line_num: usize,
    pub start: usize,
    pub end: usize,
}

impl ErrorInfo {
    pub fn new(
        file_name: Rc<PathBuf>,
        line_content: Rc<String>,
        line_num: usize,
        start: usize,
        end: usize
    ) -> Self {
        Self {
            file_name,
            line_content,
            line_num,
            start,
            end
        }
    }
}
