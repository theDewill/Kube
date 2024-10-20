//module tools
use iced::{
    widget::{button,text,column,container,image::handle,Button,Column,Container,Image,Renderer,Row,Text,Theme},
    alignment::{Horizontal,Vertical},
    window,Point,Size,Alignement,Color,Sandbox,Settings,Backword
};

//UI elements
struct KubeUI {
    page : Page,
    theme : Theme,
    login : Login,// TODO--> implement Login type
}

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

    fn run () {

    }

    fn view (&self) {
        let increment : Button<Message> = button("+").on_press(Message::Increment);
        let decrement :  Button<Message> = button("-").on_press(Message::Decrement);
        let counter = text(self.value);
        let interface = column![increment, counter, decrement];
    }

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


fn main() -> iced::Result {
    Counter::run(Settings::default());






}
