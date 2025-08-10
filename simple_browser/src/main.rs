use tao::{
    dpi::*, event::*, event_loop::*, platform::windows::WindowExtWindows, window::*
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_background_color((255, 255, 255, 0))
        .with_title("심플 브라우저")
        .with_min_inner_size(PhysicalSize::new(800, 450))
        .with_maximized(true)
        .build(&event_loop)
        .expect("윈도우 생성 실패");
    let _hwnd = window.hwnd();

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::RedrawRequested(_window_id) => {
                // println!("wow!");
            }
            _ => (),
        }
    });
}