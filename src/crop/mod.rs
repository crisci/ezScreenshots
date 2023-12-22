use iced::widget::canvas;

use iced::{Color, mouse, Point, Rectangle, Renderer, Size, Theme};

use iced::event::Status;
use iced::mouse::{Cursor, Interaction};
use iced::mouse::Button::Left;
use crate::app::Message;

#[derive(Debug, Clone, Copy)]
pub struct CropState {
    initial_point: Point,
    ending_point: Point,
    is_cropping: bool,
}

impl CropState {
    pub fn new(initial_point: Point, ending_point: Point, is_cropping: bool) -> Self {
        Self {
            initial_point,
            ending_point,
            is_cropping,
        }
    }

    pub fn set_is_cropping(&self, is_cropping: bool) -> Self {
        Self {
            is_cropping,
            ..*self
        }
    }

    pub fn set_end_point(&self, ending_point: Point) -> Self {
        Self {
            ending_point,
            ..*self
        }
    }
}

pub struct CropArea {
    height: f32,
    width: f32,
    is_cropping: bool,
    cache: canvas::Cache,
}

impl canvas::Program<Message> for CropArea {
    type State = Option<CropState>;

    fn update(&self, state: &mut Self::State, event: canvas::Event, _bounds: Rectangle, cursor: Cursor) -> (Status, Option<Message>) {
        if !self.is_cropping {
            *state = None;
            return (Status::Ignored, Some(Message::None));
        };
        if let Some(_) = cursor.position_in(Rectangle::new(_bounds.position(), Size::new(self.width, self.height))) {
            return match event {
                canvas::Event::Mouse(m) => match m {
                    mouse::Event::ButtonPressed(l) => {
                        if l == Left {
                            *state = Some(CropState::new(Point::ORIGIN, Point::ORIGIN, true))
                        };
                        (Status::Captured, Some(Message::None))
                    }
                    mouse::Event::ButtonReleased(l) => {
                        if l == Left {
                            *state = Some(state.unwrap().set_is_cropping(false));
                            (Status::Captured,
                             Some(Message::ButtonReleased(
                                 Point {
                                     x: (state.unwrap().initial_point.x - _bounds.position().x) / self.width,
                                     y: (state.unwrap().initial_point.y - _bounds.position().y) / self.height,
                                 },
                                 Point {
                                     x: (state.unwrap().ending_point.x - _bounds.position().x) / self.width,
                                     y: (state.unwrap().ending_point.y - _bounds.position().y) / self.height,
                                 })))
                            /*Send message to define crop area*/
                        } else {
                            (Status::Captured,
                             Some(Message::None))
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        if let Some(s) = *state {
                            if s.is_cropping {
                                if state.unwrap().initial_point.x == Point::ORIGIN.x {
                                    *state = Some(CropState::new(position, position, true));
                                } else {
                                    *state = Some(state.unwrap().set_end_point(position))
                                }
                            }
                        }
                        (Status::Captured, Some(Message::None))
                    }
                    _ => (Status::Captured, Some(Message::None))
                },
                _ => (Status::Captured, Some(Message::None))
            };
        } else {
            if self.is_cropping {
                if let Some(s) = state {
                    if s.is_cropping {
                        s.is_cropping = false;
                        return (Status::Captured, Some(Message::ButtonReleased(
                            Point {
                                x: (s.initial_point.x - _bounds.position().x) / self.width,
                                y: (s.initial_point.y - _bounds.position().y) / self.height,
                            },
                            Point {
                                x: (s.ending_point.x - _bounds.position().x) / self.width,
                                y: (s.ending_point.y - _bounds.position().y) / self.height,
                            },
                        )));
                    }
                }
            }
            //if exit than send message of button released
            (Status::Ignored, Some(Message::None))
        }
    }

    fn draw(&self, state: &Self::State, renderer: &Renderer, _theme: &Theme, _bounds: Rectangle, _cursor: Cursor) -> Vec<canvas::Geometry> {
        if !self.is_cropping {
            self.cache.clear();
            return vec![];
        };
        return match state {
            Some(s) => {
                let width = s.ending_point.x - s.initial_point.x;
                let height = s.ending_point.y - s.initial_point.y;
                let p = state.unwrap().initial_point;
                let geom = self.cache.draw(renderer, Size::new(self.width, self.height), |frame| {
                    frame.stroke(
                        &canvas::Path::rectangle(Point::new(p.x - _bounds.position().x, p.y - _bounds.position().y), Size::new(width, height)),
                        canvas::Stroke {
                            style: iced_graphics::geometry::Style::Solid(Color { r: 0.3, g: 0.3, b: 0.3, a: 0.8 }),
                            width: 2.,
                            ..Default::default()
                        },
                    )
                });
                vec![geom]
            }
            _ => { vec![] }
        };
    }
    fn mouse_interaction(&self, _state: &Self::State, _bounds: Rectangle, _cursor: Cursor) -> Interaction {
        let rect = Rectangle::new(_bounds.position(), _bounds.size());
        if _cursor.is_over(rect) && self.is_cropping {
            Interaction::Crosshair
        } else {
            Interaction::default()
        }
    }
}

impl CropArea {
    pub fn from_point(height: f32, width: f32, is_cropping: bool) -> Self {
        CropArea { cache: canvas::Cache::default(), height, width, is_cropping }
    }
}