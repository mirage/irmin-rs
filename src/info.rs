use crate::{irmin, Type};

#[derive(Debug, Clone, PartialEq, PartialOrd, Type)]
pub struct Info {
    pub date: i64,
    pub author: String,
    pub message: String,
}

impl Default for Info {
    fn default() -> Info {
        Info::new()
    }
}

impl Info {
    pub fn new() -> Info {
        let now = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH);
        let date = match now {
            Ok(x) => x.as_secs(),
            Err(_) => 0,
        };

        Info {
            date: date as i64,
            author: String::from("irmin-rs"),
            message: String::new(),
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }
}
