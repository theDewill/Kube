//tools
use iced::widget::{button,text,column};

struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

//Logic State--->
impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}


fn main() {
    let increment = button("+").on_press(Message::Increment);
    let decrement = button("-").on_press(Message::Decrement); // APP --> Use case of msg enum..
    let counter = text(15);

    let interface = column![increment, counter, decrement];





}
