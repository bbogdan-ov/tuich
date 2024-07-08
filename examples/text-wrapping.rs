use std::io;

use tuich::{backend::{crossterm::CrosstermBackend, Backend, BackendEvent, BackendEventReader}, buffer::Buffer, event::{Event, KeyCode}, layout::{Align, Clip, Rect, Wrap}, style::{Color, Style, Stylized, UnderlineKind}, terminal::Terminal, text::{Span, Text}, widget::{Block, Borders, Draw, List, Paragraph}};
use unicode_width::UnicodeWidthStr;

struct State {
    mouse_x: u16,
    mouse_y: u16,
    renders: usize,
    text_align: Align,
    text_wrap: Wrap
}

fn main() -> io::Result<()> {
    let mut term = Terminal::classic(CrosstermBackend(io::stdout()))?;
    let mut event_reader = term.event_reader();
    let mut state = State {
        mouse_x: 0,
        mouse_y: 0,
        renders: 0,
        text_align: Align::Center,
        text_wrap: Wrap::Words
    };

    term.enable_mouse()?;

    draw_ui(&mut term, &state)?;

    loop {
        match event_reader.read_events()? {
            Event::Key(_, KeyCode::Char('q')) => break,
            Event::Key(_, KeyCode::Char('w')) => {
                state.text_wrap = match state.text_wrap {
                    Wrap::None => Wrap::Break,
                    Wrap::Break => Wrap::Words,
                    Wrap::Words => Wrap::BreakWords,
                    Wrap::BreakWords => Wrap::None
                }
            },
            Event::Key(_, KeyCode::Char('a')) => {
                state.text_align = match state.text_align {
                    Align::Start => Align::Center,
                    Align::Center => Align::End,
                    Align::End => Align::Start
                }
            },

            Event::Mouse(_, x, y) => {
                state.mouse_x = x;
                state.mouse_y = y;
            },

            Event::Resize(w, h) => term.resize(w, h)?,
            _ => continue
        }

        draw_ui(&mut term, &state)?;

        state.renders += 1;
    }

    Ok(())
}
fn draw_ui<B: Backend>(term: &mut Terminal<B>, state: &State) -> Result<(), B::Error> {
    term.clear();

    let buf = &mut term.buffer;
    let rect = buf.rect();

    let highlight_style = Style::new(Color::Black, Color::Cyan)
        .underline(true)
        .underline_kind(UnderlineKind::Curl);

    let text: Vec<Vec<Span>> = vec![
        vec![ (format!("{} - renders ", state.renders), Color::Yellow).into() ],
        vec![
            "𝓽𝓱𝓲𝓼 𝓽𝓮𝔁𝓽 ".green(),
            "𝔀𝓪𝓼 ".into(),
            "𝔀".on_red().black(),
            "𝓻".on_green().black(),
            "𝓲".on_yellow().black(),
            "𝓽".on_blue().black(),
            "𝓽".on_magenta().black(),
            "𝓮".on_cyan().black(),
            "𝓷".on_gray().black(),
            " 𝓽𝓸 𝓽𝓮𝓼𝓽 ".into(),
            (" *text wrapping* ", highlight_style).into(),
            " 𝓲𝓷 ".into(),
            "TUICH!".red()
        ],
        vec![
            "   Some ".into(),
            "𝕗𝕒𝕟𝕔𝕪".cyan(),
            " text and even god damn emojis! 🦛 🦛 🦛  to correctly 🦛  draw emojies you need to add one space right after the emoji".into()
        ],
        vec![ "I'll fix emojis later...".gray().italic() ],
        vec![ "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it?".into() ],
        vec![ "But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.".into() ]
    ];

    let list_rect = List::row([
        Label(format!("[a] Align: {:?}", state.text_align)),
        Label(format!("[w] Wrap: {:?}", state.text_wrap)),
    ])
        .gap(1)
        .draw(buf, rect);

    TextBox { state, text }
        .draw(buf, rect.margin((1, list_rect.height+1, 0, 0)));

    term.draw()
}

struct Label(String);
impl Draw for Label {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let width = self.0.width();
        let rect = rect
            .with_width(width as u16 + 4)
            .with_height(3);

        Borders::double()
            .style(Color::Magenta)
            .draw(buf, rect);

        Text::new(&self.0, Color::Magenta)
            .draw(buf, rect.margin((2, 1)));

        rect
    }
}

struct TextBox<'a> {
    pub state: &'a State,
    pub text: Vec<Vec<Span<'a>>>
}
impl<'a> Draw for TextBox<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let rect: Rect = (
            rect.x,
            rect.y,
            self.state.mouse_x.saturating_add(1).saturating_sub(rect.x),
            self.state.mouse_y.saturating_add(1).saturating_sub(rect.y)
        ).into();

        Block::<Text, Text>::new(
            Text::from(format!(" 👉  rect width is {} 👈  ", rect.width))
                .magenta()
                .clip(Clip::Ellipsis)
                .clip_align(Align::Center)
                .align(Align::Center)
        )
            .footer(
                Text::new(" this block footer is large as fuck! ", Color::Green)
                    .clip(Clip::Custom("<".into()))
                    .clip_align(Align::Start)
            )
            .fill((" ", (Color::Magenta, Color::LightBlack)))
            .style((Color::Blue, Color::LightBlack))
            .draw(buf, rect);

        let items: Vec<Paragraph> = self.text
            .iter()
            .map(|l| Paragraph::new(l.clone())
                .wrap(self.state.text_wrap)
                .align(self.state.text_align))
            .collect();

        let list_rect = rect.margin(1);
        if list_rect.width >= 1 {
            List::col(items)
                .gap(1)
                .draw(buf, list_rect);
        }

        rect
    }
}
