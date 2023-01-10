use iced::alignment;
use iced::executor;
use iced::theme::Theme;
use iced::widget::{button, canvas::Cache, column, container, row, text, Canvas};
use iced::{Alignment, Application, Command, Element, Length};

use super::message::Direction;
use super::message::{Change, Message};
use crate::mandelbrot_calculation::mandelbrot_generate_threading;

const WIDTH: u16 = 400;
const HEIGHT: u16 = 400;

pub(crate) struct Viewer {
    pub(crate) set: Vec<(f32, f32)>,
    pub(crate) set_cache: Cache,
    zoom_level: u64,
    range_x: (f32, f32),
    range_y: (f32, f32),
}

impl Application for Viewer {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Viewer, Command<Message>) {
        (
            Viewer {
                set: vec![],
                set_cache: Default::default(),
                zoom_level: 1,
                range_x: (-2.0, 2.0),
                range_y: (-2.0, 2.0),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Mandelbrot viewer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Zoom(_) => {}
            Message::Move(_) => {}
        }

        let points = mandelbrot_generate_threading(
            1.0 / (1 << self.zoom_level) as f64,
            (self.range_x.1 + self.range_x.0) as f64 / 2.0,
            (self.range_y.1 + self.range_y.0) as f64 / 2.0,
        )
        .iter()
        .map(|(x, y)| {
            (
                (*x as f32 - self.range_x.0) * 100.0,
                (*y as f32 - self.range_y.0) * 100.0,
            )
        })
        .collect();

        self.set = points;
        self.set_cache.clear();

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let plot = Canvas::new(self)
            .width(Length::Units(WIDTH))
            .height(Length::Units(HEIGHT));

        let button = |label| {
            button(text(label).horizontal_alignment(alignment::Horizontal::Center))
                .padding(10)
                .width(Length::Units(80))
        };

        let zoom_level = text(format!("Zoom level: {}", self.zoom_level)).size(20);
        let zoom_increase_btn = button("Zoom+").on_press(Message::Zoom(Change::Increase));
        let zoom_decrease_btn = button("Zoom-").on_press(Message::Zoom(Change::Decrease));
        let zoom_controls = column![
            row![zoom_level].spacing(5).padding(5),
            row![zoom_increase_btn, zoom_decrease_btn].spacing(20)
        ]
        .align_items(Alignment::Center);

        let current_range_x = text(format!("x: [{}; {}]", self.range_x.0, self.range_x.1)).size(20);
        let current_range_y = text(format!("y: [{}; {}]", self.range_y.0, self.range_y.1)).size(20);
        let move_right_btn = button("Right").on_press(Message::Move(Direction::Right));
        let move_left_btn = button("Left").on_press(Message::Move(Direction::Left));
        let move_up_btn = button("Up").on_press(Message::Move(Direction::Up));
        let move_down_btn = button("Down").on_press(Message::Move(Direction::Down));
        let movement_controls = column![
            column![current_range_x, current_range_y]
                .spacing(5)
                .padding(5),
            row![move_up_btn].padding(2),
            row![move_left_btn, move_right_btn].spacing(2).padding(2),
            row![move_down_btn].padding(2)
        ]
        .align_items(Alignment::Center);

        let content = row![plot, column![zoom_controls, movement_controls].spacing(20)]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
