use std::cell::RefCell;
use std::rc::Rc;

use iced::image::Handle as ImageHandle;
use iced::*;

struct App {
    panes: pane_grid::State<Content>,
    shared: Rc<RefCell<SharedState>>,
}

enum Content {
    Image {
        shared: Rc<RefCell<SharedState>>,
    },
    Controls {
        shared: Rc<RefCell<SharedState>>,
        scroll: scrollable::State,
    },
}

struct SharedState {
    img_handle: ImageHandle,
    logo_size: Option<LogoSize>,
}

#[derive(Debug, Clone)]
enum Event {
    LogoSizeChanged(LogoSize),
}

impl Sandbox for App {
    type Message = Event;

    fn new() -> Self {
        let shared = Rc::new(RefCell::new(SharedState {
            img_handle: ImageHandle::from_memory(vec![]),
            logo_size: None,
        }));

        let image_content = Content::Image {
            shared: shared.clone(),
        };

        let controls_content = Content::Controls {
            shared: shared.clone(),
            scroll: scrollable::State::new(),
        };

        let (mut panes, pane) = pane_grid::State::new(image_content);
        panes.split(pane_grid::Axis::Horizontal, &pane, controls_content);

        Self { panes, shared }
    }

    fn title(&self) -> String {
        "App".into()
    }

    fn update(&mut self, evt: Event) {
        match evt {
            Event::LogoSizeChanged(logo_size) => {
                let mut shared = self.shared.borrow_mut();
                shared.img_handle = ImageHandle::from_memory(logo_size.bytes());
                shared.logo_size = Some(logo_size);
            }
        }
    }

    fn view(&mut self) -> Element<Event> {
        let pane_grid = PaneGrid::new(&mut self.panes, |_pane, content, _focus| match content {
            Content::Image { shared } => {
                let image =
                    Container::new(Image::new(shared.borrow().img_handle.clone())).padding(10);

                let content = Column::new()
                    .align_items(Align::Center)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .push(image);

                let title_bar = pane_grid::TitleBar::new("Image").padding(10);

                pane_grid::Content::new(content).title_bar(title_bar)
            }
            Content::Controls { shared, scroll } => {
                let radios = Row::new()
                    .spacing(10)
                    .push(Radio::new(
                        LogoSize::Px144,
                        "144px",
                        shared.borrow().logo_size,
                        Event::LogoSizeChanged,
                    ))
                    .push(Radio::new(
                        LogoSize::Px600,
                        "600px",
                        shared.borrow().logo_size,
                        Event::LogoSizeChanged,
                    ))
                    .push(Radio::new(
                        LogoSize::Px1024,
                        "1024px",
                        shared.borrow().logo_size,
                        Event::LogoSizeChanged,
                    ));

                let content = Column::new()
                    .spacing(5)
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .push(radios);

                let scrollable = Scrollable::new(scroll).push(
                    Container::new(content)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(),
                );

                let title_bar = pane_grid::TitleBar::new("Controls").padding(10);

                pane_grid::Content::new(scrollable).title_bar(title_bar)
            }
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let content = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(pane_grid);

        Container::new(content).height(Length::Fill).into()
    }
}

fn main() {
    <App as Sandbox>::run(Settings::default())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogoSize {
    Px144,
    Px600,
    Px1024,
}

impl LogoSize {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Px144 => {
                include_bytes!("../assets/144px-Rust_programming_language_black_logo.svg.png")
                    .to_vec()
            }
            Self::Px600 => {
                include_bytes!("../assets/600px-Rust_programming_language_black_logo.svg.png")
                    .to_vec()
            }
            Self::Px1024 => {
                include_bytes!("../assets/1024px-Rust_programming_language_black_logo.svg.png")
                    .to_vec()
            }
        }
    }
}
