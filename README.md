# TUICH

A simple rust library to simplify TUIs develpment!

> [!WARNING]
> This lib is unstable, use it at your own risk!

## Installation

For now you can only install TUICH via git:
```toml
[dependencies]
tuich = { git = "https://github.io/bbogdan-ov/tuich" }
```

I think TUICH isn't ready to be uploded to [crates.io](https://crates.io) yet

## Simple example

Here is the simple example app using the [crossterm](https://github.com/crossterm-rs/crossterm) backend

```rust
use std::io;
use tuich::{
    backend::{crossterm::CrosstermBackend, BackendEvent},
    event::{Event, Key, KeyCode, KeyMod},
    style::{Color, Stylized},
    terminal::Terminal,
    widget::{Draw, Borders, Paragraph}
};

type Term = Terminal<CrosstermBackend<io::Stdout>>;

fn main() -> io::Result<()> {
    // Create and run a new terminal in "classic mode" with crossterm backend
    // Classic mode just hides the cursor, enters alternate screen and raw mode
    let mut term: Term = Terminal::classic(CrosstermBackend::default())?;

    let mut number: isize = 0;

    // Draw the UI for the first time
    draw_ui(&mut term, &number)?;

    loop {
        match term.read_events()? {
            Event::Key(key, _key_code) => match key {
                // Exit after pressing on 'q'
                Key(_, KeyCode::Char('q')) => break,
                // Increase the number when Ctrl + Right was pressed
                Key(KeyMod::Ctrl, KeyCode::Right) =>
                    number += 1,
                // Decrease the number when Ctrl + Left was pressed
                Key(KeyMod::Ctrl, KeyCode::Left) =>
                    number -= 1,
                _ => ()
            },
            _ => ()
        }

        // Draw UI in the loop
        draw_ui(&mut term, &number)?;
    }

    Ok(())
}

fn draw_ui(term: &mut Term, number: &isize) -> io::Result<()> {
    let rect = term.rect();
    let buf = &mut term.buffer;

    // Clear the buffer before every draw
    buf.clear();

    // Draw borders with magenta border, width of screen width and height of 3
    let borders_rect = Borders::single()
        .style(Color::Magenta)
        .fill((" ", Color::Green))
        .draw(buf, rect.with_height(3));

    // Draw text "inside" the borders
    Paragraph::new([
        // Create a span with a red foreground, gray background and italic modifier
        "Hello!"
            .red()
            .on_gray()
            .italic(),
        // Create a plain span
        // Its color/style will depend on the cell on which it is placed
        // In this situation foreground color will be green because of the borders' fill color
        format!(" The number is > {} <", number).into()
    ])
        .draw(buf, borders_rect.margin(1));

    // Display the buffer on the terminal screen
    term.draw()?;

    Ok(())
}
```

## License

**MIT** license\
Do whatever you want with this project!

@ bogdanov 2024
