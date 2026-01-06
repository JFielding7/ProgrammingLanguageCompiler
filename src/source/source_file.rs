use std::path::{Display, Path, PathBuf};
use std::slice::Iter;

#[derive(Debug)]
pub struct SourceFile {
    path: PathBuf,
    lines: Vec<String>,
}

impl SourceFile {
    pub fn new(file_path: &Path, source_code: String) -> Self {
        Self {
            path: file_path.to_path_buf(),
            lines: source_code
                .lines()
                .map(|line| line.to_string()).collect::<Vec<String>>(),
        }
    }
    
    pub fn path_display(&self) -> Display<'_> {
        self.path.display()
    }

    pub fn get_line(&self, index: usize) -> &String {
        &self.lines[index]
    }
}

impl<'a> IntoIterator for &'a SourceFile {
    type Item = &'a String;
    type IntoIter = Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.iter()
    }
}
