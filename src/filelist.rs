#[derive(Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub length: u32,
}

impl FileInfo {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            name: path.rsplitn(2, "/").collect::<Vec<&str>>()[0].to_string(),
            length: 0,
        }
    }
}
