use iced::{Element, Sandbox};
use iced::widget::{text, button};

pub(crate) struct Window;

impl Sandbox for Window {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Activity & Time Logger")
    }

    fn update(&mut self, message: Self::Message) {
        match message { _ => {} }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text("Hello,eqweqwe World!").into()

    }
}
