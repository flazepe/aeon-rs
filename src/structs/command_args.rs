use std::fmt::Display;

pub struct CommandArgs {
    content: String,
    args: Vec<String>,
}

impl CommandArgs {
    pub fn new<T: Display>(content: T) -> Self {
        let content = content.to_string();
        let args = content.split_whitespace().map(|entry| entry.to_string()).collect();

        Self { content, args }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_pos_arg(&self, pos: usize, get_rest: bool) -> Option<String> {
        if get_rest {
            let arg = self.args.iter().skip(pos).map(|entry| entry.to_string()).collect::<Vec<String>>().join(" ");
            if arg.is_empty() { None } else { Some(arg) }
        } else {
            self.args.get(pos).cloned()
        }
    }
}

impl<T: Display> From<T> for CommandArgs {
    fn from(value: T) -> Self {
        CommandArgs::new(value)
    }
}
