#[macro_use]
extern crate glium;
extern crate glium_glyph;

use glium::{glutin, Surface};

mod cmdline;
mod editor;
mod filelisting;
mod constants;
mod layout_manager;
mod ui;

use std::env;
use std::any::Any;
use cmdline::CmdlineView;
use editor::EditorView;
// use editor::Buffer;
use filelisting::FileView;
use layout_manager::LayoutManager;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let args: Vec<String> = env::args().collect();
    let s: String = args[1].to_owned();
    let s_slice: &str = &s[..];
    // let a = std::env::current_dir().expect("Couldn't");
    // let out = format!("{}/{}",a.display(),s_slice);
    let out = format!("{}",s_slice);
    let out_: &str = &out[..];
    // println!("arg_a:{}\narg:{}/{}",out,a.display(),s_slice);
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(1900.0, 1034.0))
        .with_title("Zar Editor");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut layout = LayoutManager {
        views: vec![
            Box::new(EditorView::new(out_, &display)),
            Box::new(FileView::new(out_, &display)),
            Box::new(CmdlineView::new(&display)),
        ],
    };

    let mut closed = false;

    while !closed {
        layout.update_views(&display);
        // let mut visible = ;
        layout.update_value_views();
        // for view in layout.views.iter_mut() {    
        // }
        // println!("from main: {:?}",visible);

        let mut target = display.draw();
        let mut update: (&str,&str,String) = ("","", String::new());
        let color = ui::color::hex("#1B262B");
        target.clear_color_srgb(color.r, color.g, color.b, color.a);
        layout.draw(&display, &mut target);
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    // Broadcast input event
                    glutin::WindowEvent::ReceivedCharacter(c) => {
                        if (c as u32) > 31 && (c as u32) < 128{
                            // println!("adding: {}",c);
                            layout.push_char(c);
                        }
                        if c as u32 == 8 {
                            // println!("delete!");
                            layout.pop_char();
                        }
                        if (c as u32) == 13 || (c as u32) == 10 {
                            layout.push_char(c);
                        }
                    }
                    // Other window events
                    glutin::WindowEvent::Resized(logical_size) => {
                        let hidpi_factor = display.gl_window().window().get_hidpi_factor();
                        println!("main.rs resize:{:?}",logical_size.to_physical(hidpi_factor));
                        display
                            .gl_window()
                            .resize(logical_size.to_physical(hidpi_factor));
                    }
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state,
                                modifiers,
                                ..
                            },
                        ..
                    } => layout.handle_input(virtual_code, state, modifiers),

                    glutin::WindowEvent::MouseWheel {
                        delta,
                        phase,
                        ..
                    } => {
                        //
                        // let a = delta.fmt();
                        println!("{:?} {:?}", delta, phase);
                        if let glium::glutin::MouseScrollDelta::LineDelta(x, y) = delta {
                            println!("x:{} y:{}", x,y);
                        }
                    },
                    // _ => (),
                    glutin::WindowEvent::MouseInput{
                        button,
                        ..
                    } => {
                        let (view, var, val) = layout.handle_mouse_input(button);
                        match view {
                            "editor" => {
                                match var {
                                    "file" => {
                                        println!("code reached");
                                        for view in layout.views.iter_mut() {
                                            view.update_value(format!("{}",val), "editor");
                                        }
                                    }
                                    _ => {}
                                }
                            },
                            "cmd" => {

                            },
                            "file" => {

                            }
                            _ => {

                            }
                        }
                    },
                    // layout.handle_mouse_input(button),
                    glutin::WindowEvent::CursorMoved {
                        position,
                        ..
                    } => layout.update_mouse_pos(position),
                    _ => (),
                    // MouseInput {
                    //     device_id: DeviceId,
                    //     state: ElementState,
                    //     button: MouseButton,
                    //     modifiers: ModifiersState,
                    // },
                    // println!("{}, {}, {}", left, right, middle),
                    // _ => (),
                },
                _ => (),
            }
        });
    }
}