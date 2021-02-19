use crate::deck::*;
use iced::*;
use iced_native::{subscription, Event};

#[derive(Default)]
pub struct DeckViewer {
    /// Index of current slide
    slix: usize,

    /// Our deck
    deck: Deck,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NextSlide,
    PrevSlide,
}

impl DeckViewer {
    fn next_slide(&mut self) {
        let len = self.deck.len();
        if self.slix + 1 < len {
            self.slix += 1;
        }
    }

    fn prev_slide(&mut self) {
        if self.slix > 0 {
            self.slix -= 1;
        }
    }
}

impl Application for DeckViewer {
    type Executor = executor::Default;
    type Flags = Deck;
    type Message = Message;

    fn new(flags: Deck) -> (Self, Command<Message>) {
        (
            DeckViewer {
                deck: flags,
                slix: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("simp")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NextSlide => self.next_slide(),
            Message::PrevSlide => self.prev_slide(),
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Right,
                    modifiers: _modifiers,
                } => Some(Message::NextSlide),
                keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Left,
                    modifiers: _modifiers,
                } => Some(Message::PrevSlide),
                _ => None,
            },
            _ => None,
        })
    }

    fn view(&mut self) -> Element<Message> {
        let cur_slix = if self.slix >= self.deck.len() {
            self.deck.len() - 1
        } else {
            self.slix
        };
        let cur = &self.deck[cur_slix];

        // We use a column: a simple vertical layout
        let mut content = Column::new();

        content = match cur {
            Slide::Text(t) => content.push(Text::new(t).size(64)),
            Slide::Image(i) => content.push(Image::new(i)),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
