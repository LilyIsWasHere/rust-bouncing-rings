use nannou::{color::named, prelude::*, state::mouse::ButtonMap};
use rand::Rng;
use std::string::ToString;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Debug, Clone, Copy)]
enum Color{
    Honeydew,
    SteelBlue,
    Black,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

type Rgb = Srgb<u8>;

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        named::from_str(&c.to_string()).unwrap()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

trait Nannou {
    fn display(&self, draw: &Draw);
    fn update(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RingDirection {
    Growing,
    Shrinking,
}

#[derive(Debug, Clone)]
struct Ring {
    color: Rgb,
    origin: Point,
    radius: f32,
    weight: f32,
    growth_rate: f32,
    direction: RingDirection,
}

impl Ring {
    fn new() -> Self {
        Self::default()
    }

    fn is_intersecting(&self, other: &Self) -> bool {
        if self == other {
            return false;
        }
        match (self.direction, other.direction) {
            (RingDirection::Shrinking, RingDirection::Shrinking) => return false,
            _ => (),
        }
        let distance = self.origin.distance_to(&other.origin);

        let r1 = self.radius;
        let r2 = other.radius;

        let external_range = r1 + r2 - self.growth_rate..r1 + r2 + self.growth_rate;
        let internal_range = (r1 - r2).abs() - self.growth_rate..(r1 - r2).abs() + self.growth_rate;

        let intersecting = distance > external_range.start && distance < external_range.end || distance > internal_range.start && distance < internal_range.end;
        if intersecting == true {
            println!("[{:?}, rad={:?}, direction={:?}] and [{:?}, rad={:?}, direction={:?}] intersect!", self.origin, self.radius, self.direction, other.origin, other.radius, other.direction);
        }
        intersecting
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .no_fill()
            .w(self.radius*2.0)
            .h(self.radius*2.0)
            .x_y(self.origin.x, self.origin.y)
            .stroke(Rgb::from(self.color))
            .stroke_weight(self.weight);
    }

    fn update(&mut self, rings: &Vec<Ring>) {
        let mut intersecting = false;
        for other in rings {
            if (self.is_intersecting(other)) {
                intersecting = true;
                break;
            }
        }
        if intersecting {
            self.direction = match self.direction {
                RingDirection::Growing => RingDirection::Shrinking,
                RingDirection::Shrinking => RingDirection::Growing,
            };
        }

        match self.direction {
            RingDirection::Growing => self.radius += self.growth_rate,
            RingDirection::Shrinking => self.radius -= self.growth_rate,
        }

        match (self.radius < 0.0, self.direction) {
            (true, RingDirection::Shrinking) => self.direction = RingDirection::Growing,
            _ => (),
        }
    }

    fn set_origin(&mut self, x: f32, y: f32) {
        self.origin = Point::new(x, y);
    }

}

impl PartialEq for Ring {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.radius == other.radius && self.weight == other.weight && self.growth_rate == other.growth_rate && self.direction == other.direction
    }
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            color: rgb(rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(0..255), rand::thread_rng().gen_range(200..255)),
            origin: Point::default(),
            radius: 0.0,
            weight: 3.0,
            growth_rate: 0.5,
            direction: RingDirection::Growing,
        }
    }
}

struct Model {
    bg_color: Color,
    current_bg: usize,
    rings: Vec<Ring>,
    button_state: nannou::state::mouse::ButtonPosition,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            bg_color: Color::Black,
            current_bg: usize::default(),
            rings: Vec::default(),
            button_state: nannou::state::mouse::ButtonPosition::Up,
        }
    }
}

impl Nannou for Model {
    fn display(&self, draw: &Draw) {
        draw.background().color(Rgb::from(self.bg_color));
        self.rings.display(draw);
    }
    /// Update this model
    fn update(&mut self) {
        self.rings.update();
    }
}


impl Nannou for Vec<Ring> {
    fn display(&self, draw: &Draw) {
        for ring in self.iter() {
            ring.display(draw);
        }
    }
    fn update(&mut self) {
        let clone = self.clone();
        for ring in self.iter_mut() {
            ring.update(&clone);
        }
    }
}



//
// Nannou interface
//


/// Nannou app model
fn model(_app: &App) -> Model {
    Model::default()
}

/// Nannou app updates
fn update(_app: &App, model: &mut Model, _update: Update) {
    /*if _app.mouse.buttons != model.button_state {
        let mut new_ring = Ring::new();
        new_ring.set_origin(_app.mouse.x, _app.mouse.y);
        model.rings.push(new_ring);
    }*/
    let left_button = _app.mouse.buttons.left();
    if model.button_state != *left_button {
        match left_button {
            nannou::state::mouse::ButtonPosition::Up => (),
            nannou::state::mouse::ButtonPosition::Down(pos) => {
                let mut new_ring = Ring::new();
                new_ring.set_origin(pos.x, pos.y);
                model.rings.push(new_ring);
            },
        }
        model.button_state = *left_button;
    }
    model.update();

    
}

/// Nannou app view
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // Draw model
    model.display(&draw); 

    // Render frame
    draw.to_frame(app, &frame).unwrap();

}