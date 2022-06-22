#[macro_use]
extern crate lazy_static;

use crate::parser::Parser;
use iced::{
    alignment, button, window, Button, Color, Column, Element, Length, Row, Rule, Sandbox,
    Settings, Text,
};
use std::str::FromStr;

mod parser;

#[derive(Default)]
struct Calculator {
    content: String,
    buttons: ButtonsGrid,
}

#[derive(Debug, Clone)]
enum Message {
    NumericButton(usize),
    Operation(Operation),
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Calculator {
        Calculator::default()
    }

    fn title(&self) -> String {
        "Scalc".to_string()
    }

    fn view(&mut self) -> Element<Message> {
        let mut too_long = " ";

        let display_text = if self.content.is_empty() {
            " "
        } else {
            if self.content.len() <= 19 {
                &self.content
            } else {
                too_long = "«";
                &self.content[(self.content.len() - 19)..]
            }
        };

        let content: Element<_> = Column::new()
            .width(Length::Shrink)
            .height(Length::Shrink)
            .align_items(alignment::Alignment::End)
            .push(
                Row::new()
                    .push(
                        Text::new(too_long)
                            .width(Length::FillPortion(6))
                            .horizontal_alignment(alignment::Horizontal::Center),
                    )
                    .push(
                        Text::new(display_text)
                            .width(Length::FillPortion(94))
                            .horizontal_alignment(alignment::Horizontal::Right),
                    ),
            )
            .push(Rule::horizontal(1))
            .push(self.buttons.view())
            .into();

        //content
        content.explain(Color::BLACK)
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NumericButton(v) => self.content += &v.to_string(),
            Message::Operation(o) => match o {
                Operation::Add => self.content += "+",
                Operation::Sub => self.content += "-",
                Operation::Mul => self.content += "*",
                Operation::Div => self.content += "/",
            },
        }
    }
}

#[derive(Default)]
struct ButtonsGrid {
    button_0: button::State,
    button_1: button::State,
    button_2: button::State,
    button_3: button::State,
    button_4: button::State,
    button_5: button::State,
    button_6: button::State,
    button_7: button::State,
    button_8: button::State,
    button_9: button::State,

    button_add: button::State,
    button_sub: button::State,
    button_mul: button::State,
    button_div: button::State,
}

impl ButtonsGrid {
    fn view(&mut self) -> Element<Message> {
        let vspace = 5;
        let hspace = 5;

        Column::new()
            .spacing(vspace)
            .push(
                Row::new()
                    .spacing(hspace)
                    .push(
                        Button::new(&mut self.button_7, Text::new("7"))
                            .on_press(Message::NumericButton(7)),
                    )
                    .push(
                        Button::new(&mut self.button_8, Text::new("8"))
                            .on_press(Message::NumericButton(8)),
                    )
                    .push(
                        Button::new(&mut self.button_9, Text::new("9"))
                            .on_press(Message::NumericButton(9)),
                    )
                    .push(
                        Button::new(&mut self.button_div, Text::new("/"))
                            .on_press(Message::Operation(Operation::Div)),
                    ),
            )
            .push(
                Row::new()
                    .spacing(hspace)
                    .push(
                        Button::new(&mut self.button_4, Text::new("4"))
                            .on_press(Message::NumericButton(4)),
                    )
                    .push(
                        Button::new(&mut self.button_5, Text::new("5"))
                            .on_press(Message::NumericButton(5)),
                    )
                    .push(
                        Button::new(&mut self.button_6, Text::new("6"))
                            .on_press(Message::NumericButton(6)),
                    )
                    .push(
                        Button::new(&mut self.button_mul, Text::new("*"))
                            .on_press(Message::Operation(Operation::Mul)),
                    ),
            )
            .push(
                Row::new()
                    .spacing(hspace)
                    .push(
                        Button::new(&mut self.button_1, Text::new("1"))
                            .on_press(Message::NumericButton(1)),
                    )
                    .push(
                        Button::new(&mut self.button_2, Text::new("2"))
                            .on_press(Message::NumericButton(2)),
                    )
                    .push(
                        Button::new(&mut self.button_3, Text::new("3"))
                            .on_press(Message::NumericButton(3)),
                    )
                    .push(
                        Button::new(&mut self.button_sub, Text::new("-"))
                            .on_press(Message::Operation(Operation::Sub)),
                    ),
            )
            .push(
                Row::new()
                    .spacing(hspace)
                    .push(
                        Button::new(&mut self.button_0, Text::new("0"))
                            .on_press(Message::NumericButton(0)),
                    )
                    .push(
                        Button::new(&mut self.button_add, Text::new("+"))
                            .on_press(Message::Operation(Operation::Add)),
                    ),
            )
            .into()
    }
}

#[derive(Debug)]
enum CalcVal {
    Number(isize),
    Operator(char),
    Error(String),
}

fn parse_expression(s: &str) {
    let parser = Parser::new()
        .push("[0-9]+", |s| {
            if let Ok(v) = s.parse::<isize>() {
                return CalcVal::Number(v);
            }
            CalcVal::Error(String::from("Unable to parse integer: ") + s)
        })
        .push("[+\\-\\*/]", |s| {
            CalcVal::Operator(s.chars().next().unwrap())
        });

    for value in parser.parse(s) {
        match value {
            Ok(v) => println!("{:?}", v),
            Err(_) => {
                println!("An error happened !");
                break;
            }
        }
    }
}

fn main() -> iced::Result {
    parse_expression("125+14");

    Calculator::run(Settings {
        window: window::Settings {
            size: (200, 300),
            resizable: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
