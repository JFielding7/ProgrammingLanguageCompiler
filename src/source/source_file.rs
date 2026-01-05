use std::iter::{Enumerate, Map};
use std::path::{Display, PathBuf};
use std::slice::Iter;

#[derive(Debug)]
pub struct SourceFile {
    path: PathBuf,
    lines: Vec<String>,
}

impl SourceFile {
    pub fn new(path: PathBuf, lines: Vec<String>) -> Self {
        Self {
            path,
            lines,
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
