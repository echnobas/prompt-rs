use crate::errors::PromptError;
use getch::Getch;
use std::io::Write;

pub struct Prompt<'prompt> {
    max_length: u32,
    hidden: bool,
    prompt: &'prompt str,
    on_cancel: Box<dyn FnOnce()>,
}

impl<'prompt> Prompt<'prompt> {
    pub fn new() -> Self {
        Self {
            max_length: 0,
            hidden: false,
            prompt: "",
            on_cancel: Box::new(||{}),
        }
    }

    pub fn set_max_length(self, max_length: u32) -> Self {
        Self {
            max_length,
            hidden: self.hidden,
            prompt: self.prompt,
            on_cancel: self.on_cancel,
        }
    }

    pub fn set_hidden(self, hidden: bool) -> Self {
        Self {
            max_length: self.max_length,
            hidden,
            prompt: self.prompt,
            on_cancel: self.on_cancel,
        }
    }

    pub fn set_prompt(self, prompt: &'prompt str) -> Self {
        Self {
            max_length: self.max_length,
            hidden: self.hidden,
            prompt,
            on_cancel: self.on_cancel,
        }
    }

    pub fn set_cancel<F>(self, on_cancel: F) -> Self
    where
        F: FnOnce() + 'static,
    {
        Self {
            max_length: self.max_length,
            hidden: self.hidden,
            prompt: self.prompt,
            on_cancel: Box::new(on_cancel),
        }
    }

    pub fn execute(self) -> Result<String, PromptError> {
        print!("{}", self.prompt);
        let mut result = String::new();
        let mut pos: u32 = 0;
        loop {
            std::io::stdout().flush()?;
            let c: char = Getch::new().getch()? as char;
            match c {
                '\n' | '\r' => break,
                '\x08' => {
                    if pos == 0 {
                        continue;
                    }
                    pos -= 1;
                    print!("\x08 \x08");
                    result.pop();
                    continue;
                }
                _ => {
                    if c as u8 == 3 {
                        (self.on_cancel)();
                        return Err(PromptError::CancelError);
                    }
                    if pos >= self.max_length && self.max_length != 0 { // default is 0
                        continue;
                    }
                    pos += 1;
                    if self.hidden {
                        print!("*");
                    } else {
                        print!("{}", c);
                    }
                    result.push(c);
                }
            }
        }
        println!();
        Ok(result)
    }
}
