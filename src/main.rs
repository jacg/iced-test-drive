use iced::{Rectangle, Sandbox, Point, Size, Settings, Container, Text, Length,
           Column, Alignment, Row, Canvas, Color,
           canvas::{self, Cursor, Geometry, Program, Path, Frame, Stroke, path}, Button,
           button};


fn main() -> iced::Result {
    Thingy::run(Settings::default())
}

#[derive(Default)]
struct Thingy {
    button_a_state: button::State,
    button_b_state: button::State,
    pitch: Pitch,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Click { x: f32, y: f32 }, // NOTE normalized coordinates TODO strong typing
    ButtonA,
    ButtonB,
}

impl Sandbox for Thingy {
    type Message = Option<Message>;

    fn new() -> Self {
        Thingy::default()
    }

    fn title(&self) -> String {
        "The Mighty Thingy".into()
    }

    fn update(&mut self, message: Self::Message) {
        use Message::*;
        if let Some(message) = message {
            match message {
                ButtonA => {self.pitch.draw_the_thing = true;}
                ButtonB => {self.pitch.draw_the_thing = false;}
                _ => ()
            }
        }
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        let canvas = Canvas::new(self.pitch)
            .height(Length::Fill)
            .width(Length::Fill);
        let row = Row::new()
            .push(
                Button::new(&mut self.button_a_state, Text::new("AAA"))
                    .on_press(Some(Message::ButtonA))
            )
            .push(
                Button::new(&mut self.button_b_state, Text::new("BBB"))
                    .on_press(Some(Message::ButtonB))
            )
            .spacing(30)
            ;

        let column = Column::new()
            //.padding(120)
            .spacing(50)
            // .push(Text::new("short"))
            // .push(Text::new("veeeeeeeeeeeeeeeeeeeeeeryyyyy looooooooong"))
            // .align_items(Alignment::End)
            // .push(text)
            .push(canvas)
            .push(row)
            ;
        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
//            .center_y()
            .into()
    }

    // fn background_color(&self) -> iced::Color {
    //     iced::Color::WHITE
    // }

    // fn scale_factor(&self) -> f64 {
    //     1.0
    // }

    // fn should_exit(&self) -> bool {
    //     false
    // }

    // fn run(settings: iced::Settings<()>) -> Result<(), iced::Error>
    // where
    //     Self: 'static + Sized,
    // {
    //     <Self as iced::Application>::run(settings)
    // }
}

#[derive(Default, Clone, Copy)]
struct Pitch {
    draw_the_thing: bool,
}

impl Program<Option<Message>> for Pitch {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let ends_fraction = 0.1;
        let aspect_ratio = (1. + 2. * ends_fraction) * 27. / 16.;
        let h = bounds.height.min(bounds.width / aspect_ratio);
        let w = h * aspect_ratio;
        let mut frame = Frame::new(Size::new(w, h));
        let lt = 0.01 * h; // line thickness
        let hlt = lt / 2.; // half line thickness
        let end_w_l      = w *            ends_fraction;
        let end_w_r      = w * (1. -      ends_fraction);
        let field_length = w * (1. - 2. * ends_fraction);
        let zone_radius = h * 3. / 16.;
        let colour_line  = Color::BLACK;
        let colour_field = Color::from_rgb8(120, 120, 255);
        let colour_ends  = Color::from_rgb8( 50,  50, 200);
        let colour_zone  = Color::from_rgb8(200,  50,  50);

        fn arc((ax, ay): (f32, f32), (bx, by): (f32, f32), r: f32) -> Path {
            Path::new(|b| {
                b.arc_to(Point::new(ax, ay), Point::new(bx, by), r);
            })
        }

        fn rect(w: f32, h: f32, dw: f32, dh: f32) -> Path {
            Path::rectangle(Point::new(w, h), Size::new(dw, dh))
        }

        fn draw(frame: &mut Frame, path: &Path, fill_colour: Color, stroke_colour: Color, stroke_width: f32) {
            frame.fill(&path, fill_colour);
            frame.stroke(&path, Stroke::default().with_color(stroke_colour).with_width(stroke_width));
        }

        let line = |frame: &mut Frame, w_position: f32| {
            let a = Point::new(w_position, 0.);
            let b = Point::new(w_position, h );
            let line = Path::line(a, b);
            frame.stroke(&line, Stroke::default().with_color(colour_line).with_width(lt));
        };

        let rect_ends  = rect(hlt    , hlt, w-lt        , h-lt);
        let rect_field = rect(end_w_l, hlt, field_length, h-lt);
        let semi_zonel = arc((end_w_l, h*0.5 + zone_radius),
                             (end_w_l, h*0.5 - zone_radius), zone_radius);
        let semi_zoner = arc((end_w_r, h*0.5 - zone_radius),
                             (end_w_r, h*0.5 + zone_radius), zone_radius);
        // let line_middle = todo!();
        draw(&mut frame, &rect_ends , colour_ends , colour_line, lt);
        draw(&mut frame, &rect_field, colour_field, colour_line, lt);
        draw(&mut frame, &semi_zonel, colour_zone , colour_line, lt);
        draw(&mut frame, &semi_zoner, colour_zone , colour_line, lt);
        if self.draw_the_thing {line(&mut frame, 0.5*w);}
        line(&mut frame, end_w_l);
        line(&mut frame, end_w_r);

        if let Some(Point { x, y }) = cursor.position_in(&bounds) {
            let r = rect(x, y, 50., 50.);
            draw(&mut frame, &r, colour_zone , colour_line, lt);
        }

        vec![
            frame.into_geometry(),
        ]
    }

    // fn update(
    //     &mut self,
    //     _event: iced::canvas::Event,
    //     _bounds: iced::Rectangle,
    //     _cursor: iced::canvas::Cursor,
    // ) -> (iced::canvas::event::Status, Option<()>) {
    //     (iced::canvas::event::Status::Ignored, None)
    // }

    // fn mouse_interaction(
    //     &self,
    //     _bounds: iced::Rectangle,
    //     _cursor: iced::canvas::Cursor,
    // ) -> iced_native::mouse::Interaction {
    //     iced_native::mouse::Interaction::default()
    // }
}
