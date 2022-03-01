use std::path::Path;
use walkdir::WalkDir;
use glium::glutin::{ElementState, ModifiersState, VirtualKeyCode};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::rusttype::Rect;
use glium_glyph::glyph_brush::{rusttype::Font, GlyphCruncher, Section};
use glium_glyph::GlyphBrush;
// use std::process::Command;

use crate::constants::{BASE_FONT_SIZE, CMD_SHIFT_HOLD, NO_MODIFIERS};
use crate::layout_manager::View;
use crate::ui::panel::Panel;
use crate::ui::color;

pub struct FileView<'a, 'b> {
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    letter_size: Rect<f32>,
    command_text: String,
    visible: bool,
    background: Panel,
    dis: Display,
    // dir: &'a Path,
    dir: String,
    file_list: Vec<String>,
    mouse_pos: glium::glutin::dpi::LogicalPosition 
    // tar: &mut Frame,
}

// pub fn return_walk_dir(dir: String) -> String{
//     for entry in WalkDir::new(Path::new(&dir).parent().unwrap()).max_depth(1).into_iter().filter_map(|e| e.ok()) {
//         // a = &entry.path();
//         // s = format!("{}/{}\n",s,format!("{}",entry.path().display()).split("/").last().unwrap());
//         if let Some(filename_path) = entry.path().file_name() {
//             // s = format!("{}{}\n",s,format!("{}",filename_path.to_string_lossy()))
//         }
//         // s = format!("{}/{}\n",s,format!("{}",.to_str()).split("/").last().unwrap());
//         // s = format!("{}/{}\n",s,format!("{}",entry.path().parent().unwrap().display()));
//         // .parent().unwrap()
//         println!("{}", entry.path().display());
//         // println!("current folder:{}",Path::new(dir).parent().unwrap().display());

//     }
//     return String::new()
// }

impl<'a, 'b> FileView<'a, 'b> {
    pub fn new(dir: &str, display: &Display) -> FileView<'a, 'b> {
        let t = display.clone();
        // target.finish();
        // let tar = target;
        // let t = display.clone();
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let mut gb = GlyphBrush::new(display, fonts);
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let font_size = BASE_FONT_SIZE * hidpi_factor;
        // let a = Path::new("./");
        let mut s1 = String::new();
        let mut s2 = String::new();
        // let mut act_dir = "";
        // let mut dir_split = dir.split("/");
        // let len = dir_split.len();
        // let a = dir_split.last().unwrap();
        // println!("last of dir, {:?}",a);
        // let mut dis_dir = "";
        let mut i = 0;
        for entry in WalkDir::new(Path::new(dir).parent().unwrap()).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if i == 0 {
                i = 1;
                continue;
            }
            // a = &entry.path();
            // s = format!("{}/{}\n",s,format!("{}",entry.path().display()).split("/").last().unwrap());
            if let Some(filename_path) = entry.path().file_name() {
                if entry.path().is_dir() {
                    s2 = format!("{}\n",s2);
                    s1 = format!("{}{}/\n",s1,format!("{}",filename_path.to_string_lossy()))
                } else {
                    s1 = format!("{}\n",s1);
                    s2 = format!("{}{}\n",s2,format!("{}",filename_path.to_string_lossy()))
                }
            }
            // s = format!("{}/{}\n",s,format!("{}",.to_str()).split("/").last().unwrap());
            // s = format!("{}/{}\n",s,format!("{}",entry.path().parent().unwrap().display()));
            // .parent().unwrap()
            println!("{}", entry.path().display());
            println!("current folder:{}",Path::new(dir).parent().unwrap().display());

        }
        // s = &s;
        let letter_size = gb
            .glyph_bounds(Section {
                text: "0",
                scale: glyph_brush::rusttype::Scale::uniform(font_size),
                ..Section::default()
            })
            .unwrap();
        // let screen_dims = display.get_framebuffer_dimensions();
        let bg_w = 120.0; let bg_h = 500.0;
        // let bg_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - bg_w / 2.0;
        // let bg_y = screen_dims.1 as f32 / hidpi_factor / 2.0 - bg_h / 2.0;

        FileView {
            glyph_brush: gb,
            padding: 0.0,
            font_size: font_size,
            letter_size: letter_size,
            command_text: String::new(),
            // "".to_owned() 
            visible: true,
            background: Panel::new(&display, [0.0, 0.0], [bg_w, bg_h], color::hex("#1f272b").as_slice()),
            dis: t,
            dir: Path::new(dir).display().to_string(),
            file_list: vec!(s1,s2),
            mouse_pos: glium::glutin::dpi::LogicalPosition::new(0 as f64, 0 as f64),
            // tar: target,
        }
    }
}
// 4A148C
// struct Error;

impl<'a, 'b> View for FileView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        // self.display = display;
        // println!("ooga: ({}) end!",self.dir);
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        self.font_size = BASE_FONT_SIZE * hidpi_factor;
        let screen_dims = display.get_framebuffer_dimensions();
        // let text_x = screen_dims.0 as f32 / hidpi_factor - (300.0 / hidpi_factor) + self.padding * hidpi_factor;
        // let text_y = screen_dims.1 as f32 / 4.0 - self.font_size / 2.0;
        let mut s1 = String::new();
        let mut s2 = String::new();
        let mut i = 0;
        // println!("self.dir:{}",self.dir);
        for entry in WalkDir::new(Path::new(&self.dir).parent().unwrap()).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if i == 0 {
                i = 1;
                continue;
            }
            // println!("{}", entry.path().display());
            if let Some(filename_path) = entry.path().file_name() {
                if entry.path().is_dir() {
                    s2 = format!("{}\n",s2);
                    s1 = format!("{}{}/\n",s1,format!("{}",filename_path.to_string_lossy()))
                } else {
                    s1 = format!("{}\n",s1);
                    s2 = format!("{}{}\n",s2,format!("{}",filename_path.to_string_lossy()))
                }
            }
        }
        // println!("update s1:{}",s1);
        // println!("update s2:{}",s2);
        self.file_list[0] = s1;
        self.file_list[1] = s2;

        self.glyph_brush.queue(Section {
            text: &self.file_list[0],
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: (0.0 , 0.0),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: color::hex("#D3C6AA").as_slice(),
            // #22282c
            ..Section::default()
        });
        self.glyph_brush.queue(Section {
            text: &self.file_list[1],
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: (0.0 , 0.0),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: color::hex("#7DB8B5").as_slice(),
            // #32CFC9
            // #22282c
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
            (VirtualKeyCode::F, ElementState::Pressed, CMD_SHIFT_HOLD) => {
                self.visible = !self.visible;
                println!("visible: {}",self.visible);
                // self.command_text = String::new();
                // self.background.draw(self.__target);
            }
            _ => (),
        }
    }

    fn handle_mouse_input(&mut self, button:glium::glutin::MouseButton) -> (&str, String) {
        match button {
            glium::glutin::MouseButton::Left => {
                if self.visible {
                    let (row,col) = (((self.mouse_pos.y - (self.mouse_pos.y%10.0))/10.0), (((self.mouse_pos.x - (self.mouse_pos.x%5.0))/5.0)));
                    println!("clicked at row:{} collumn:{}",row,col);
                    // println!("clicked at row:{} collumn:{}", ((self.mouse_pos.y - (self.mouse_pos.y%10.0))/10.0),
                    // (((self.mouse_pos.x - (self.mouse_pos.x%5.0))/5.0)));
                    let mut result: Vec<&str> = self.file_list[0].lines().collect();
                    let files_vec: Vec<&str> = self.file_list[1].lines().collect();
                    let mut i = 0;
                    // let mut n = 0;
                    let resultclone = &result.clone();
                    for a_str in resultclone{
                        if a_str == &"" {
                            result[i] = &files_vec[i];
                            // n = n+1
                        }
                        i = i+1;
                    } 
                    println!("result:{:?}",result);
                    println!("slef.dir:{}",self.dir);
                    // result.append(&mut result2);
                    if result.len() > row as usize && row >= 0.0 {
                        println!("passed first if, {}",row);
                        if result[row as usize].len() > col as usize && col >= 0.0 {
                            println!("passed second if, {}",row);
                            println!("{}",result[row as usize]);
                            let filepath = format!("{}/{}",Path::new(&self.dir).parent().unwrap().display(),result[row as usize]);
                            // let filepath = format!("{}",result[row as usize]);
                            println!("filepath:{}",filepath);
                            if Path::new(&filepath).is_dir() == false{
                                return ("file", format!("{}",filepath));
                            } else {
                                println!("its a dir");
                            }
                        }
                    }
                }
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
        self.mouse_pos = position;

    }

    fn push_char(&mut self, c: char) {
        return ();
        // if self.visible {
        //     println!("Char:{}",c as u32);
        //     if c as u32 == 16 {
        //         println!("dle!");
        //     } else if c as u32 == 13 {
        //         println!("enter!");
        //         // let t = self.command_text.replace(|c: char| !(c as u32 <= 127 && c as u32 != 16), "");
        //         // println!("t:{}", t);
        //         let st: &str = &self.command_text;
        //         // let s = s.replace("\020", "");
        //         let output = if cfg!(target_os = "windows") {
        //             Command::new("cmd")
        //                     .arg("/C")
        //                     .arg(st)
        //                     .output()
        //                     .expect("failed to execute process")
        //         } else {
        //             Command::new("/bin/bash")
        //                     .arg("-c")
        //                     .arg(st)
        //                     .output()
        //                     .expect("failed to execute process")
        //         };
        //         // println!(r"Command_t:{}", t);
        //         println!("Command:{}", self.command_text);
        //         println!("status: {}", output.status);
        //         println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //         println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        //         let display = &self.dis;
        //         let screen_dims = display.get_framebuffer_dimensions();
        //         let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        //         let text_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - (300.0 / hidpi_factor) + self.padding * hidpi_factor;
        //         let text_y = screen_dims.1 as f32 / 2.0 - self.font_size / 2.0;
        //         let bg_w = 600.0; let bg_h = 15.0;
        //         let bg_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - bg_w / 2.0;
        //         let bg_y = screen_dims.1 as f32 / hidpi_factor / 2.0 - bg_h / 2.0;
        //         let b = Panel::new(&display, [bg_x, bg_y], [bg_w, bg_h], color::hex("#D3C6AA").as_slice());
        //         // self.glyph_brush.queue(Section {
        //         //     text: &self.command_text,
        //         //     bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
        //         //     screen_position: (text_x, text_y),
        //         //     scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
        //         //     color: color::hex("#FFFFFF").as_slice(),
        //         //     ..Section::default()
        //         // });
        //         // let mut target = self.dis.draw();
        //         // self.background.draw(&mut target);
        //         // self.glyph_brush.draw_queued(display, &mut target);
        //         // target.finish().unwrap();
        //         // draw(&self.dis, &mut target);
        //         println!("we have display! {}", text_y);
        //         self.visible = false;
        //     } else if c as u32 == 8 {
        //         self.command_text.pop();
        //     } else {
        //         self.command_text.push(c);
        //     }
        // }
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
            return (true, "file");
        }
        return (false, "file");
    }

    fn on_other_visible(&mut self, visable: &Vec<String>) {

    }
}
