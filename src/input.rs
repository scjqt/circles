use enum_map::{Enum, EnumMap};
use ggez::{
    input::{
        keyboard::{self, KeyCode as K},
        mouse::{self, MouseButton as M},
    },
    Context,
};
use glam::IVec2;
use std::ops::Index;

#[derive(Clone, Copy, Enum)]
pub enum Input {
    LeftMouse,
    RightMouse,
    Clear,
    Quit,
}

pub struct Inputs {
    current: EnumMap<Input, bool>,
    last: EnumMap<Input, bool>,
    mouse_position: IVec2,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            current: EnumMap::default(),
            last: EnumMap::default(),
            mouse_position: IVec2::ZERO,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        use Input::*;

        self.last = self.current;
        self.current.clear();
        let inputs = &mut self.current;

        for code in keyboard::pressed_keys(ctx) {
            match code {
                K::Escape => inputs[Quit] = true,
                K::Space => inputs[Clear] = true,
                _ => (),
            }
        }

        if mouse::button_pressed(ctx, M::Left) {
            inputs[LeftMouse] = true;
        }

        if mouse::button_pressed(ctx, M::Right) {
            inputs[RightMouse] = true;
        }

        let mouse_position = mouse::position(ctx);
        self.mouse_position.x = mouse_position.x as i32;
        self.mouse_position.y = mouse_position.y as i32;
    }

    pub fn last(&self, input: Input) -> bool {
        self.last[input]
    }

    pub fn mouse_position(&self) -> IVec2 {
        self.mouse_position
    }
}

impl Index<Input> for Inputs {
    type Output = bool;

    fn index(&self, index: Input) -> &Self::Output {
        &self.current[index]
    }
}
