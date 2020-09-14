use crate::layers::domain::entities::Printable;

pub struct Example {
    pub name: String,
    pub content: Vec<String>,
    pub title: Option<String>,
    pub language: Option<String>,
    pub id: Option<String>
}

impl Example {
    pub fn new(name: String, content: Vec<String>, title: Option<String>, language: Option<String>, id: Option<String>) -> Example {
        Example {
            name,
            content,
            title,
            language,
            id
        }
    }
}

impl Printable for Example {
    fn print(&self) -> String {
        self.content.join("\n")
    }

    fn file_name(&self) -> String {
        self.name.clone()
    }
}
