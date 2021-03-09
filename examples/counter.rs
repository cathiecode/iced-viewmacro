/*
    Original code comes from iced example (https://github.com/hecrj/iced/tree/master/examples/counter) and it was licensed under MIT license:

    Copyright 2019 Héctor Ramón, Iced contributors

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software without restriction, including without limitation the rights to
    use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
    COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
    IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
    CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};
use iced_vmacro::view;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        /*
            Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    Button::new(&mut self.increment_button, Text::new("Increment"))
                        .on_press(Message::IncrementPressed),
                )
                .push(Text::new(self.value.to_string()).size(50))
                .push(
                    Button::new(&mut self.decrement_button, Text::new("Decrement"))
                        .on_press(Message::DecrementPressed),
                )
                .into()
        */
        view!(
            (Column::new(); padding={20} align_items={Align::Center} {
                (Button::new(&mut self.increment_button, Text::new("Increment")); on_press={Message::IncrementPressed})
                (Text::new(self.value.to_string()); size={50})
                (Button::new(&mut self.decrement_button, Text::new("Decrement")); on_press={Message::DecrementPressed})
            })
        ).into()
    }
}
