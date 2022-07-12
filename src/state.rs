use crate::input::{self, Inputs};
use ggez::{
    graphics::{self, Color, DrawMode, DrawParam},
    Context, GameResult,
};
use glam::DVec2;
use rand::Rng;

const TPS: u64 = 128;
const GRAVITY: f64 = 500.;
const REPETIIONS: u8 = 4;
const SMALLEST_RADIUS: f64 = 5.;
const LARGEST_RADIUS: f64 = 30.;
const OUTER_RADIUS: f64 = 350.;

const BACKGROUND: (u8, u8, u8) = (0, 0, 0);
const OUTER_COLOUR: (u8, u8, u8) = (30, 30, 30);

const TICK_DURATION: f64 = 1. / TPS as f64;
const TICK_GRAVITY: f64 = GRAVITY * TICK_DURATION * TICK_DURATION;

use super::{HEIGHT, WIDTH};
const CENTRE: DVec2 = DVec2::new(WIDTH as f64 / 2., HEIGHT as f64 / 2.);

pub struct State {
    accumulator: f64,
    circles: Vec<Circle>,
}

impl State {
    pub fn new() -> Self {
        Self {
            accumulator: 0.,
            circles: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f64, inputs: &Inputs) {
        use input::Input::*;

        let mouse = inputs.mouse_position().as_dvec2();

        if inputs[Clear] && !inputs.last(Clear) {
            self.circles.clear();
        }

        if inputs[LeftMouse] && !inputs.last(LeftMouse) {
            let mut upper = LARGEST_RADIUS;
            let lower = SMALLEST_RADIUS;
            let radius = lower + random().max(random()) * (upper - lower);
            for circle in &self.circles {
                let distance = circle.position.distance(mouse) - circle.radius;
                if distance < upper {
                    upper = distance;
                }
            }
            let distance = OUTER_RADIUS - CENTRE.distance(mouse);
            if distance < upper {
                upper = distance;
            }
            if upper >= lower {
                self.circles
                    .push(Circle::new(mouse, radius.min(upper), random_colour()));
            }
        }

        if inputs[RightMouse] && !inputs.last(RightMouse) {
            let mut i = 0;
            while i < self.circles.len() {
                if self.circles[i].point_within(mouse) {
                    self.circles.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }

        self.accumulator += dt;
        while self.accumulator >= TICK_DURATION {
            self.tick();
            self.accumulator -= TICK_DURATION;
        }
    }

    fn tick(&mut self) {
        for circle in self.circles.iter_mut() {
            let last = circle.position;
            circle.position += circle.position - circle.last_position;
            circle.last_position = last;
            circle.position.y += TICK_GRAVITY;
        }

        for _ in 0..REPETIIONS {
            for i in 0..self.circles.len() {
                for j in i + 1..self.circles.len() {
                    let a = &self.circles[i];
                    let b = &self.circles[j];
                    let dist_sq = a.position.distance_squared(b.position);
                    let sum_radii = a.radius + b.radius;
                    if dist_sq < sum_radii * sum_radii {
                        let midpoint = (a.position + b.position) / 2.;
                        let offset = (a.position - b.position).normalize() * sum_radii / 2.;
                        self.circles[i].position = midpoint + offset;
                        self.circles[j].position = midpoint - offset;
                    }
                }
            }
            for circle in self.circles.iter_mut() {
                let max_dist = OUTER_RADIUS - circle.radius;
                let offset = circle.position - CENTRE;
                if offset.length_squared() > max_dist * max_dist {
                    circle.position = offset.normalize() * max_dist + CENTRE;
                }
            }
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        let t = self.accumulator / TICK_DURATION;

        graphics::clear(ctx, BACKGROUND.into());

        draw_circle(ctx, CENTRE, OUTER_RADIUS, OUTER_COLOUR.into())?;

        for circle in &self.circles {
            circle.render(ctx, t)?;
        }

        graphics::present(ctx)
    }
}

struct Circle {
    position: DVec2,
    last_position: DVec2,
    radius: f64,
    colour: Color,
}

impl Circle {
    fn new(position: DVec2, radius: f64, colour: Color) -> Self {
        Self {
            position,
            last_position: position,
            radius,
            colour,
        }
    }

    fn render(&self, ctx: &mut Context, t: f64) -> GameResult {
        draw_circle(
            ctx,
            self.last_position.lerp(self.position, t),
            self.radius,
            self.colour,
        )
    }

    fn point_within(&self, pos: DVec2) -> bool {
        self.position.distance_squared(pos) < self.radius * self.radius
    }
}

fn draw_circle(ctx: &mut Context, centre: DVec2, radius: f64, colour: Color) -> GameResult {
    let mesh = graphics::Mesh::new_circle(
        ctx,
        DrawMode::fill(),
        [centre.x as f32, centre.y as f32],
        radius as f32,
        0.1,
        colour,
    )?;
    graphics::draw(ctx, &mesh, DrawParam::default())
}

fn random_colour() -> Color {
    (
        55 + (random() * 200.) as u8,
        55 + (random() * 200.) as u8,
        55 + (random() * 200.) as u8,
    )
        .into()
}

fn random() -> f64 {
    rand::thread_rng().gen()
}
