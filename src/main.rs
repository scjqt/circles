mod input;
mod state;

use ggez::{
    conf::{NumSamples, WindowMode, WindowSetup},
    event::{
        self,
        winit_event::{Event, WindowEvent},
        ControlFlow,
    },
    timer, ContextBuilder, GameResult,
};
use input::{Input, Inputs};
use state::State;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;

fn main() -> GameResult {
    let window_mode = WindowMode::default().dimensions(WIDTH, HEIGHT);
    let window_setup = WindowSetup::default()
        .title("circles")
        .samples(NumSamples::Eight)
        .vsync(true);

    let (mut ctx, event_loop) = ContextBuilder::new("circles", "sam")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;

    let mut state = State::new();
    let mut inputs = Inputs::new();
    inputs.update(&mut ctx);

    event_loop.run(move |mut event, _, control_flow| {
        let ctx = &mut ctx;
        *control_flow = ControlFlow::Poll;
        event::process_event(ctx, &mut event);

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        } else if let Event::MainEventsCleared = event {
            ctx.timer_context.tick();

            inputs.update(ctx);

            if inputs[Input::Quit] {
                *control_flow = ControlFlow::Exit;
            }

            state.update(timer::delta(ctx).as_secs_f64(), &inputs);
            state.render(ctx).unwrap();
        }
    });
}
