use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, BufRead, BufReader},
    time::{Duration, Instant},
};
use std::{
    fs::{self, File},
    io::Read,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

struct App {
    scroll: u16,
    text: Vec<String>,
}

impl App {
    fn new(text: Vec<String>) -> App {
        App { scroll: 0, text }
    }

    // fn on_tick(&mut self) {
    //     self.scroll += 1;
    //     self.scroll %= 10;
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let f = File::open("example-file.md")?;
    let f = BufReader::new(f);
    let text = f.lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    // let mut data = Vec::new();Vj
    // file.read_to_end(&mut data)?;

    let app = App::new(text);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                // KeyCode::Char('k') => {
                //     app.scroll -= 1;
                //     return Ok(());
                // }
                // KeyCode::Char('j') => {
                //     app.scroll += 1;
                //     return Ok(());
                // }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    // let text = vec![
    //     Spans::from("This is a line "),
    //     Spans::from(Span::styled(
    //         "This is a line   ",
    //         Style::default().fg(Color::Red),
    //     )),
    //     Spans::from(Span::styled(
    //         "This is a line",
    //         Style::default().bg(Color::Blue),
    //     )),
    //     Spans::from(Span::styled(
    //         "This is a longer line",
    //         Style::default().add_modifier(Modifier::CROSSED_OUT),
    //     )),
    //     Spans::from(Span::styled(&long_line, Style::default().bg(Color::Green))),
    //     Spans::from(Span::styled(
    //         "This is a line",
    //         Style::default()
    //             .fg(Color::Green)
    //             .add_modifier(Modifier::ITALIC),
    //     )),
    // ];
    let text = app
        .text
        .clone()
        .into_iter()
        .map(|line: String| Spans::from(line))
        .collect::<Vec<Spans>>();

    let paragraph = Paragraph::new(text)
        .style(Style::default())
        .scroll((app.scroll, 0));
    // .wrap(Wrap { trim: true });
    f.render_widget(paragraph, f.size());
}
