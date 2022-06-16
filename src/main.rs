use iced::{
    button, text_input, Button, Column, Element, Length, Padding, Row, Sandbox, Settings, Text,
    TextInput,
};
#[derive(Default)]
struct Calculator {
    content: String,
    input: text_input::State,
    buttons: ButtonsGrid,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    NumericButton(usize),
    AddButton,
    SubButton,
    MulButton,
    DivButton,
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
        Column::new()
            .push(TextInput::new(
                &mut self.input,
                "Type something",
                "",
                Message::InputChanged,
            ))
            .push(Text::new(&self.content))
            .push(self.buttons.view())
            .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(s) => self.content += &s,
            _ => (),
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
                            .on_press(Message::DivButton),
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
                            .on_press(Message::MulButton),
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
                            .on_press(Message::SubButton),
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
                            .on_press(Message::AddButton),
                    ),
            )
            .into()
    }
}

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}
