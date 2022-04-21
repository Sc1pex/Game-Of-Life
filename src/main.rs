use game::Game;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod cell;
mod game;

#[async_std::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Test")
        .with_inner_size(PhysicalSize::<u32>::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    let mut game = Game::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                game.input(event);
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                game.update();
                match game.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => game.resize(),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            Event::MainEventsCleared => window.request_redraw(),

            _ => {}
        }
    });
}
