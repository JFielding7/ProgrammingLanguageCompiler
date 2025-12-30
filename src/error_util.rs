use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ErrorLocation {
    pub file_name: Rc<PathBuf>,
    pub line_content: Rc<String>,
    pub line_num: usize,
    pub start: usize,
    pub end: usize,
}

impl ErrorLocation {
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

impl std::fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span_length = self.end - self.start;
        let line_num = self.line_num.to_string();
        let pre_underline_space = " ".repeat(line_num.len());
        let underline = " ".repeat(self.start) + &"^".repeat(span_length);

        write!(f,
               "File:  {file}:{line_num_header}:{col}\n{line_num} | {line}\n{pre_underline_space} | {underline}",
               file = self.file_name.display(),
               line_num_header = self.line_num,
               col = self.start,
               line_num = line_num,
               line = self.line_content,
               pre_underline_space = pre_underline_space,
               underline = underline
        )
    }
}
