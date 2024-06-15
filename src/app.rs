use crate::colors::*;
use crate::untyped_arithmetic::UntypedArithmetic;
use crate::untyped_lambda_calculus::UntypedLambdaCalculus;
use std::fmt::Debug;

use color_eyre::{eyre::WrapErr, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::errors;
use crate::tui;
use crate::evaluator::Evaluator;
use crate::evaluator::available_evaluators;

pub fn app_main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::new(Box::new(UntypedArithmetic)).run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

#[derive(Debug)]
pub struct App {
    evaluator: Box<dyn Evaluator>,
    exit: bool,
    current_input: String,
    history_input: Vec<String>,
    history_output: Vec<String>,
    in_pick_state: bool,
}

impl App {
    pub fn new(evaluator: Box<dyn Evaluator>) -> Self {
        Self {
            evaluator,
            exit: false,
            current_input: String::new(),
            history_input: Vec::new(),
            history_output: Vec::new(),
            in_pick_state: true,
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.modifiers {
            KeyModifiers::CONTROL => match key_event.code {
                KeyCode::Char('d') => self.exit(),
                KeyCode::Char('c') => self.exit(),
                _ => {}
            },
            _ => match key_event.code {
                KeyCode::Enter => {
                    self.execute_input(self.current_input.to_string());
                    self.current_input.clear();
                }
                KeyCode::Backspace => {
                    self.current_input.pop();
                }
                KeyCode::Char(c) => {
                    self.current_input.push(c);
                }
                _ => {}
            },
        }

        Ok(())
    }

    fn execute_input(&mut self, input: String) {
        self.history_input.push(input.to_string());
        if self.in_pick_state {
            let index = input.parse::<usize>().unwrap();
            self.evaluator = match index {
                1 => Box::new(UntypedArithmetic),
                2 => Box::new(UntypedLambdaCalculus),
                _ => return,
            };
            self.in_pick_state = false;
            self.history_output.push(self.evaluator.name());
            return;
        }
        if input == "change" {
            let mut out = String::new();
            out.push_str(CYAN);
            out.push_str("Pick an evaluator:");
            out.push_str(RESET);
            out.push_str("\n");
            let evaluators = available_evaluators();
            for (i, evaluator) in evaluators.iter().enumerate() {
                out.push_str(GREEN);
                out.push_str(&format!("{}. ", i + 1));
                out.push_str(RESET);
                out.push_str(&evaluator.name());
                out.push_str("\n");
            }
            self.history_output.push(out);
            self.in_pick_state = true;
            return;
        }

        let out = self.evaluator.run(&input);
        self.history_output.push(out);
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Lambda Calculus Environment ".bold());
        let instructions =
            Title::from(Line::from(vec![" Quit ".into(), "<Ctrl-D> ".blue().bold()]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let mut lines = Vec::new();
        for i in 0..self.history_input.len() {
            lines.push(Line::from(vec![
                PROMPT.to_string().green().bold().into(),
                self.history_input[i].clone().into(),
                "\n".into(),
            ]));
            let o: Vec<Line> = self.history_output[i]
                .clone()
                .split('\n')
                .map(|s| s.to_string().into())
                .collect();
            lines.extend(o);
        }
        lines.push(Line::from(vec![
            PROMPT.to_string().green().bold().into(),
            self.current_input.clone().into(),
        ]));

        let counter_text = Text::from(lines);

        Paragraph::new(counter_text)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}
