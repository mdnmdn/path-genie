use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use std::io;
use std::io::Stdout;
use std::time::{Duration, Instant};

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BorderType, Borders, ListItem};
use tui::{backend::CrosstermBackend, widgets, widgets::ListState, Frame, Terminal};

type Backend = CrosstermBackend<Stdout>;

pub struct UI<'a> {
    paths: Vec<&'a str>,
}

impl<'a> UI<'a> {
    pub fn new(paths: Vec<&'a str>) -> Self {
        Self { paths }
    }

    pub fn init(&self) -> Result<Option<&str>, Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.ui(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        terminal.show_cursor()?;

        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn ui(&self, terminal: &mut Terminal<Backend>) -> Result<Option<&'a str>, io::Error> {
        let tick_rate = Duration::from_millis(250);

        let mut list = StatefulList::with_items(self.paths.clone());
        list.next();

        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| {
                self.draw(frame, &mut list);
            });

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => return Ok(None),
                        KeyCode::Enter => {
                            if let Some(path_index) = list.state.selected() {
                                if let Some(s) = list.items.get(path_index) {
                                    return Ok(Some(*s));
                                }
                            }
                            return Ok(None);
                        }
                        KeyCode::Down => list.next(),
                        KeyCode::Up => list.previous(),
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                //app.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn draw(&self, frame: &mut Frame<Backend>, data: &mut StatefulList<&str>) {
        let w = tui::widgets::Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center)
            .title(" Select path ");
        frame.render_widget(w, frame.size());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Percentage(98)])
            .horizontal_margin(1)
            .vertical_margin(1)
            .split(frame.size());

        let list_items: Vec<_> = data.items.iter().map(|p| ListItem::new(*p)).collect();
        let list = widgets::List::new(list_items)
            //.highlight_style(Style::default().bg(Color::White).fg(Color::Black))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, chunks[1], &mut data.state);
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}
