use glium::glutin::{ElementState, ModifiersState, VirtualKeyCode};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::rusttype::Rect;
use glium_glyph::glyph_brush::{rusttype::Font, GlyphCruncher, Section};
use glium_glyph::GlyphBrush;
use std::process::Command;

use crate::constants::{BASE_FONT_SIZE, CMD_SHIFT_HOLD, NO_MODIFIERS};
use crate::layout_manager::View;
use crate::ui::panel::Panel;
use crate::ui::color;

pub struct CmdlineView<'a, 'b> {
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    letter_size: Rect<f32>,
    command_text: String,
    visible: bool,
    background: Panel,
    dis: Display,
    // tar: &mut Frame,
}

impl<'a, 'b> CmdlineView<'a, 'b> {
    pub fn new(display: &Display) -> CmdlineView<'a, 'b> {
        let t = display.clone();
        // target.finish();
        // let tar = target;
        // let t = display.clone();
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let mut gb = GlyphBrush::new(display, fonts);
        let __display = display;
        // let __target = target;
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let font_size = BASE_FONT_SIZE * hidpi_factor;
        let letter_size = gb
            .glyph_bounds(Section {
                text: "0",
                scale: glyph_brush::rusttype::Scale::uniform(font_size),
                ..Section::default()
            })
            .unwrap();

        let screen_dims = display.get_framebuffer_dimensions();
        let bg_w = 600.0; let bg_h = 15.0;
        let bg_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - bg_w / 2.0;
        let bg_y = screen_dims.1 as f32 / hidpi_factor / 2.0 - bg_h / 2.0;

        CmdlineView {
            glyph_brush: gb,
            padding: 30.0,
            font_size: font_size,
            letter_size: letter_size,
            command_text: String::new(),
            // "".to_owned() 
            visible: false,
            background: Panel::new(&display, [bg_x, bg_y], [bg_w, bg_h], color::hex("#D3C6AA").as_slice()),
            dis: t,
            // tar: target,
        }
    }
}
// 4A148C

impl<'a, 'b> View for CmdlineView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        // self.display = display;
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        self.font_size = BASE_FONT_SIZE * hidpi_factor;
        let screen_dims = display.get_framebuffer_dimensions();
        let text_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - (300.0 / hidpi_factor) + self.padding * hidpi_factor;
        let text_y = screen_dims.1 as f32 / 2.0 - self.font_size / 2.0;

        self.glyph_brush.queue(Section {
            text: &self.command_text,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: (text_x, text_y),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: color::hex("#22282c").as_slice(),
            ..Section::default()
        });
    }

    fn draw(&mut self, display: &Display, target: &mut Frame) {
        if self.visible {
            self.background.draw(target);
            self.glyph_brush.draw_queued(display, target);
        }
    }

    fn handle_input(
        &mut self,
        key_code: VirtualKeyCode,
        state: ElementState,
        modifiers: ModifiersState,
        // display: &Display,
        // target: &mut Frame,
    ) {
        match (key_code, state, modifiers) {
            (VirtualKeyCode::P, ElementState::Pressed, CMD_SHIFT_HOLD) => {
                self.visible = !self.visible;
                println!("visible: {}",self.visible);
                self.command_text = String::new();
                // self.background.draw(self.__target);
            }
            (VirtualKeyCode::Escape, ElementState::Pressed, NO_MODIFIERS) => {
                self.visible = false;
                println!("visible: {}",self.visible);
            }
            _ => (),
        }
    }

    fn handle_mouse_input(&mut self, button:glium::glutin::MouseButton) -> (&str, String) {
        match button {
            glium::glutin::MouseButton::Left => {
                println!("left click");
                return ("", format!(""));
            }
            glium::glutin::MouseButton::Right => {
                println!("right click");
                return ("", format!(""));
            }
            glium::glutin::MouseButton::Middle => {
                println!("Middle click");
                return ("", format!(""));
            }
            glium::glutin::MouseButton::Other(i) => {
                println!("other{} click",i);
                return ("", format!(""));
            }
        }
    }

    fn update_mouse_pos(&mut self, position:glium::glutin::dpi::LogicalPosition){
        // println!("from cmdliner:{}, {}", position.x, position.y);
    }

    fn push_char(&mut self, c: char) {
        if self.visible {
            println!("Char:{}",c as u32);
            if c as u32 == 16 {
                println!("dle!");
            } else if c as u32 == 13 {
                println!("enter!");
                // let t = self.command_text.replace(|c: char| !(c as u32 <= 127 && c as u32 != 16), "");
                // println!("t:{}", t);
                let st: &str = &self.command_text;
                // let s = s.replace("\020", "");
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                            .arg("/C")
                            .arg(st)
                            .output()
                            .expect("failed to execute process")
                } else {
                    Command::new("/bin/bash")
                            .arg("-c")
                            .arg(st)
                            .output()
                            .expect("failed to execute process")
                };
                // println!(r"Command_t:{}", t);
                println!("Command:{}", self.command_text);
                println!("status: {}", output.status);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                let display = &self.dis;
                let screen_dims = display.get_framebuffer_dimensions();
                let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
                let text_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - (300.0 / hidpi_factor) + self.padding * hidpi_factor;
                let text_y = screen_dims.1 as f32 / 2.0 - self.font_size / 2.0;
                let bg_w = 600.0; let bg_h = 15.0;
                let bg_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - bg_w / 2.0;
                let bg_y = screen_dims.1 as f32 / hidpi_factor / 2.0 - bg_h / 2.0;
                let b = Panel::new(&display, [bg_x, bg_y], [bg_w, bg_h], color::hex("#D3C6AA").as_slice());
                self.glyph_brush.queue(Section {
                    text: &self.command_text,
                    bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
                    screen_position: (text_x, text_y),
                    scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
                    color: color::hex("#FFFFFF").as_slice(),
                    ..Section::default()
                });
                let mut target = self.dis.draw();
                self.background.draw(&mut target);
                self.glyph_brush.draw_queued(display, &mut target);
                target.finish().unwrap();
                // draw(&self.dis, &mut target);
                println!("we have display! {}", text_y);
                self.visible = false;
            } else if c as u32 == 8 {
                self.command_text.pop();
            } else {
                self.command_text.push(c);
            }
        }
    }

    fn pop_char(&mut self) {
        if self.visible {
            println!("char deleted!");
            self.command_text.pop();
        }
    }

    fn update_value(&mut self, value: String, check: &str) {

    }

    fn is_visible(&mut self) -> (bool, &str) {
        if self.visible {
            return (true, "cmd");
        }
        return (false, "cmd");
    }

    fn on_other_visible(&mut self, visable: &Vec<String>) {

    }
}
