use tao::{
    dpi::*, event::*, event_loop::*, window::*
};
use wry::*;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_background_color((255, 255, 255, 0))
        .with_title("심플 브라우저")
        .with_min_inner_size(PhysicalSize::new(800, 450))
        .with_maximized(true)
        .build(&event_loop)
        .expect("윈도우 생성 실패");
    let main_view = WebViewBuilder::new()
        .with_bounds(Rect {
            position: Position::Physical(PhysicalPosition { x: 0, y: 120 }),
            size: Size::Physical(PhysicalSize { width: window.inner_size().width, height: window.inner_size().height - 120 }),
        })
        .with_url("https://google.com")
        .build_as_child(&window)
        .expect("메인 웹뷰 생성 실패");
    let sub_view = WebViewBuilder::new()
        .with_bounds(Rect {
            position: Position::Physical(PhysicalPosition { x: 0, y: 0 }),
            size: Size::Physical(PhysicalSize { width: window.inner_size().width, height: 120 }),
        })
        .with_html(include_str!("sub_view.html"))
        .build_as_child(&window)
        .expect("서브 웹뷰 생성 실패");


    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                let width = size.width;
                let height = size.height;

                main_view.set_bounds(Rect {
                    position: Position::Physical(PhysicalPosition { x: 0, y: 120 }),
                    size: Size::Physical(PhysicalSize { width: width, height: height - 120 }),
                }).unwrap();
                sub_view.set_bounds(Rect {
                    position: Position::Physical(PhysicalPosition { x: 0, y: 0 }),
                    size: Size::Physical(PhysicalSize { width: width, height: 120 }),
                }).unwrap();
                
            },
            Event::RedrawRequested(_window_id) => {
                // println!("wow!");
            }
            _ => (),
        }
    });
}