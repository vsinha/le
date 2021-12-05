#[macro_use]
extern crate clap;

use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufRead, BufReader},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    style::Style,
    text::Spans,
    widgets::Paragraph,
    Frame, Terminal,
};

struct App {
    scroll: u16,
    text: Vec<String>,
}

impl App {
    fn new(filename: Option<&str>) -> Result<App, Box<dyn Error>> {
        let text = match filename {
            Some(filename) => {
                let f = File::open(filename)?;
                let f = BufReader::new(f);
                f.lines().map(|line| line.unwrap()).collect::<Vec<String>>()
            }
            None => vec!["".to_owned()],
        };
        Ok(App { scroll: 0, text })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(le =>
        (version: "0.1")
        (author: "Viraj Sinha <root@vsinha.com>")
        (about: "le is less than less")
        (@arg FILENAME: "Sets the input file to use")
    )
    .get_matches();

    let filename = matches.value_of("FILENAME");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(filename)?;
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
                KeyCode::Char('k') => {
                    if app.scroll > 0 {
                        app.scroll -= 1;
                    }
                }
                KeyCode::Char('j') => {
                    app.scroll += 1;
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
