use crate::models::{context::Context};
use std::time::Duration;
use std::io::{self, BufRead, Error};
use std::{path::PathBuf};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use ratatui::style::Stylize;
use ratatui::{
    backend::Backend, 
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll, read},
    layout::{Constraint, Direction, Layout}, 
    style::{Color, Modifier, Style}, 
    text::{Line, Span, Text}, 
    widgets::{Block, Paragraph, Borders}, 
    Frame, Terminal
};


pub fn score_options(options: Vec<PathBuf>, query: String, tags: Option<Vec<String>>) -> Vec<QueryResponse> {
    let _tags = tags.unwrap_or(vec![String::from("")]);
    let mut results: Vec<QueryResponse> = Vec::new();
    let matcher = SkimMatcherV2::default();

    for option in options {
        let name = option.file_name().expect("bad utf-8").to_str().unwrap();
        if let Some((score, indices)) = matcher.fuzzy_indices(&name, &query) {
            results.push(QueryResponse::new(option.clone(), String::from(name), score, indices));
        }
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

pub fn query_tui(context: Context, input: String) -> Result<QueryResponse, std::io::Error>
{
    // would be cool to have inline frame but it doesn't seem like it can be cleaned up 
    //let mut terminal = ratatui::init_with_options(TerminalOptions {
    //    viewport: Viewport::Inline(options.len() as u16),
    //});
    
    // try and match directly to a note before falling back to fuzzy terminal

    let options = context.notes.into_iter().map(|note| note.path).collect::<Vec<PathBuf>>();
    let scores = score_options(options.clone(), input.clone(), None);
    if scores.len() == 1 {
        return Ok(scores[0].to_owned());
    }
    for result in &scores {
        if result.filename == input {
            return Ok(result.clone());
        } 
    }

    let filtered_scores = scores.iter()
        .filter(|response| response.score > 0)
        .map(|response| response.to_owned())
        .collect::<Vec<QueryResponse>>();
    if filtered_scores.len() == 1 {
        return Ok(filtered_scores[0].to_owned());
    }

    let mut terminal = ratatui::init();
    let mut query = FZFQuery::new(input, options);

    // flush stdin
    while poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = read(); // discard the event
    }

    let result = query.run(&mut terminal);
    ratatui::restore();

    result
}

#[derive(Debug, Clone)]
pub struct QueryResponse {
    pub path: PathBuf,
    pub filename: String,
    pub score: i64,
    pub matching_indices: Vec<usize>,
}

impl QueryResponse {
    fn new(path: PathBuf, name: String, score: i64, indices: Vec<usize>) -> Self {
        Self {
            path: path,
            filename: name,
            score: score,
            matching_indices: indices,
        }
    }
}

pub struct FZFQuery {
    input: String,
    selected: u8,
    options: Vec<PathBuf>,
    rankings: Vec<QueryResponse>,
}

impl FZFQuery
{
    fn new(inp: String, opt: Vec<PathBuf>) -> Self {
        Self {
            input: inp.clone(),
            selected: 0,
            options: opt.clone(),
            rankings: score_options(opt, inp, None),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<QueryResponse, std::io::Error> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue; // ignore non-press events
                }
                
                match key.code {
                    KeyCode::Esc => {
                        // flush stdin
                        while poll(Duration::from_millis(0)).unwrap_or(false) {
                            let _ = read(); // discard the event
                        }
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "user canceled query"));
                    }
                    KeyCode::Enter => {
                        if self.rankings.len() as i8 == 0 {
                            return Err(Error::new(std::io::ErrorKind::InvalidInput, "no valid options"));
                        }
                        return Ok(self.rankings[self.selected as usize].clone());
                    }
                    KeyCode::Up => {
                        self.selected = (self.selected + 1) % self.rankings.len() as u8;
                    }
                    KeyCode::Down => {
                        if self.selected == 0 {
                            self.selected = self.rankings.len() as u8;
                        }
                        self.selected = self.selected.saturating_sub(1);
                    }
                    KeyCode::Backspace => {
                        if self.input.len() > 0 {
                            self.input.pop();
                        }
                    }
                    KeyCode::Char(to_insert) => 
                    {
                            self.input.push(to_insert);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame)
    {
        // run fuzzy matching and layout generation first so we can determine spacing
        let mut fzf_opts: Vec<String> = vec![];
        let mut styled_opts: Vec<Line<'static>> = vec![];
        self.rankings = score_options(self.options.clone(), self.input.clone(), None);
        self.selected = self.selected.min((self.rankings.len() as u8).saturating_sub(1).max(0));

        for (i, query) in self.rankings.iter().enumerate()
        {
            if query.score > 0 || self.input.clone() == "" {
                fzf_opts.push(query.filename.clone());
                //styled_opts.insert(0 as usize, Line::styled(str, Style::default().bg(Color::LightRed)));
                let mut line = highlight_matches_ascii(&query.filename, &query.matching_indices);
                if i == self.selected as usize {
                    line.style = Style::default().add_modifier(Modifier::REVERSED);
                }
                styled_opts.insert(0 as usize, line);
            }
        }

        let area = frame.area();
        let area_split = Layout::default().direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Percentage(65)
            ]).split(area);
        let file_area = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
            ]).split(area_split[1]);
        let chunks = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),         
                Constraint::Length(styled_opts.len() as u16 + 2),
                Constraint::Length(3),      // bottom line for input
            ]).split(area_split[0]);
        
        // One line made of styled spans
        let line = Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Red)),
            Span::styled(self.input.as_str(), Style::default().fg(Color::LightRed)),
        ]);

        let text = Text::from(line); // Text from one Line
        let para = Paragraph::new(text).block(Block::default().borders(Borders::ALL).border_style(Style::default().light_yellow()));
        frame.render_widget(para, chunks[2]);

        let opt_lines = Text::from(styled_opts);
        let opt_paragraph = Paragraph::new(opt_lines).block(Block::default().borders(Borders::ALL).title_top("Matched Notes").border_style(Style::default().light_yellow()));
        frame.render_widget(opt_paragraph, chunks[1]);
        
        let mut file_lines: Vec<Line> = vec![];
        if self.rankings.len() > 0 {
            match std::fs::File::open(self.rankings[self.selected as usize].path.clone()) {
                Ok(file) => {
                    let bufreader = io::BufReader::new(file).lines();
                    for (i, line) in bufreader.enumerate() {
                        let padded_nums = format!("{: >3}", (i+1).to_string());
                        let lc_span = Span::from(padded_nums).style(Style::default().fg(Color::Gray));
                        let letter_span = Span::from(line.unwrap_or(String::from("")));
                        file_lines.push(Line::from(vec![lc_span, Span::from(" "), letter_span]));
                    }
                }
                _ => {
                    file_lines = vec![Line::from(String::from("no preview available"))];
                }
            }
        }
        else
        {
            file_lines = vec![Line::from(String::from(""))];
        }
        let preview_para = Paragraph::new(file_lines).block(Block::default().borders(Borders::ALL).title_top("File Preview").border_style(Style::default().light_yellow()));
        frame.render_widget(preview_para, file_area[0]);
    }
}

fn highlight_matches_ascii(line: &str, matches: &[usize]) -> Line<'static> {
    let mut spans = Vec::with_capacity(line.len());

    for (i, ch) in line.chars().enumerate() {
        let style = if matches.contains(&i) {
            Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        spans.push(Span::styled(ch.to_string(), style));
    }

    Line::from(spans)
}

