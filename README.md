# prompt-rs
A simple crate for handling user input.

```rs
use prompt_rs::prompt::Prompt;

fn main() {
    println!(
        "Hey there {}!",
        Prompt::new()
            .set_hidden(true)
            .set_prompt("Enter name noob\n> ")
            .execute()
            .unwrap()
    );
}
```

## Features:
- Hiding the user input
- Setting a maximum length for the user input
- Setting a prompt for the user input
- Few dependencies allow for lightning fast compile times
- Reads stdin via _getch on windows and termios on unix via the getch crate
