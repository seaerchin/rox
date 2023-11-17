pub trait Substring {
    fn substring(&self, start: usize, end: usize) -> &str;
}

impl Substring for String {
    fn substring(&self, start: usize, end: usize) -> &str {
        &self[start..end]
    }
}
