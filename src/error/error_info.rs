#[derive(Debug, Copy, Clone)]
pub struct ErrorInfo {
    pub line_num: usize,
    pub start: usize,
    pub end: usize,
}

impl ErrorInfo {
    pub fn new(line_num: usize, start: usize, end: usize) -> Self {
        Self { line_num, start, end }
    }
}
