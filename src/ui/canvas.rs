use iced::widget::canvas::{Cursor, Geometry, Path, Program};
use iced::Point;
use iced::Size;
use iced::{Color, Rectangle, Theme};

use super::message::Message;
use super::viewer::Viewer;

impl Program<Message> for Viewer {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let set = self.set_cache.draw(bounds.size(), |frame| {
            let set_points = Path::new(|path| {
                for (x, y) in &self.set {
                    path.rectangle(Point::new(*x, *y), Size::UNIT);
                }
            });

            frame.fill(&set_points, Color::BLACK);
        });

        vec![set]
    }
}
