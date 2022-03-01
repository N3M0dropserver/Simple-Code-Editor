mod buffer;
mod cursor;


use glium::glutin::{ElementState, ModifiersState, VirtualKeyCode};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::rusttype::Rect;
use glium_glyph::glyph_brush::{rusttype::Font, GlyphCruncher, Section};
use glium_glyph::GlyphBrush;
// use syntect::easy::HighlightLines;
// use syntect::parsing::SyntaxSet;
// use syntect::highlighting::{ThemeSet, Style};
// use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use buffer::Buffer;
use cursor::Cursor;

use crate::constants::{BASE_FONT_SIZE, CTRL_HOLD, NO_MODIFIERS, SHIFT_HOLD};
use crate::layout_manager::View;
use crate::ui;

pub struct EditorView<'a, 'b> {
    pub buffer: Buffer,
    cursor: Cursor,
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    offset_y: usize,
    viewport_rows: usize,
    letter_size: Rect<f32>,
    last_column: i32,
    scale: f32,
    visible: Vec<String>,
    mouse_pos: glium::glutin::dpi::LogicalPosition,
}

impl<'a, 'b> EditorView<'a, 'b> {
    pub fn new(file: &str, display: &Display) -> EditorView<'a, 'b> {
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let scale = 0 as f32;
        let font_size = (BASE_FONT_SIZE - scale) * hidpi_factor;
        let mut gb = GlyphBrush::new(display, fonts);
        // let __display = display;
        // let a = vec!(String::new());
        // let __target = target;
        let letter_size = gb
            .glyph_bounds(Section {
                text: "0",
                // scale: glyph_brush::rusttype::Scale::uniform(5 as f32),
                scale: glyph_brush::rusttype::Scale::uniform(font_size),
                ..Section::default()
            })
            .unwrap();

        EditorView {
            buffer: Buffer::new(file),
            cursor: Cursor::new(),
            glyph_brush: gb,
            padding: 30.0,
            font_size: font_size,
            // font_size: 5 as f32,
            offset_y: 0,
            viewport_rows: 0,
            letter_size: letter_size,
            // letter_size: 5 as f32,
            last_column: -1,
            scale: scale,
            visible: vec!(String::new()),
            mouse_pos: glium::glutin::dpi::LogicalPosition::new(0 as f64, 0 as f64),
        }
    }

    fn scroll_down(&mut self) {
        println!("{}", self.offset_y);
        if self.offset_y + self.cursor.row as usize + 1 < self.buffer.get_lines_count() {
            self.offset_y += 1;
        }
    }

    fn scroll_up(&mut self) {
        if self.offset_y != 0 {
            self.offset_y -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.offset_y + self.cursor.row as usize > self.buffer.get_lines_count() - 3 {
            return;
        }
        println!("{}, {}", self.cursor.row, self.viewport_rows);
        if self.cursor.row  as usize > self.viewport_rows - 4 {
            self.scroll_down();
        } else {
            self.cursor.row += 1;
        }
        if self.last_column != -1 {
            self.cursor.col = self.last_column;
        }
        self.last_column = -1;
        self.move_to_eol();
    }

    fn move_cursor_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
        }
        if self.cursor.row == 0 {
            self.scroll_up();
        }
        if self.last_column != -1 {
            self.cursor.col = self.last_column;
        }
        self.last_column = -1;
        self.move_to_eol();
    }

    fn move_cursor_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else {
            let pos = self.offset_y + self.cursor.row as usize;
            if pos == 0 {
                self.last_column = -1;
                return;
            }
            let current_line = self.buffer.get_line_at(pos - 1);
            self.cursor.row -= 1;
            self.cursor.col = current_line.len() as i32 - 1;
        }
        self.last_column = -1;
    }

    fn move_cursor_right(&mut self) {
        let pos = self.offset_y + self.cursor.row as usize;
        if pos > self.buffer.get_lines_count() - 2 {
            return;
        }
        let current_line = self.buffer.get_line_at(pos);
        if current_line.len() - 1 > self.cursor.col as usize {
            self.cursor.col += 1;
        } else {
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
        self.last_column = -1;
    }

    fn move_to_bol(&mut self) {
        self.cursor.col = 0;
        self.last_column = -1;
    }

    fn move_to_eol(&mut self) {
        let current_line = self
            .buffer
            .get_line_at(self.offset_y + self.cursor.row as usize);
        let length = current_line.len() as i32 - 1;
        if self.cursor.col > length {
            self.last_column = self.cursor.col;
            self.cursor.col = length;
        }
    }
}

impl<'a, 'b> View for EditorView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        // let (screen_width, screen_height) = display.get_framebuffer_dimensions();
        let mut space_x = 0.0;
        for item in self.visible.iter() {
            if item == &String::from("file") {
                // self.padding = self.padding + 20.0;
                // println!("screen_width:{}, screen_height:{}",screen_width, screen_height);
                space_x = 200.0;//* hidpi_factor;
            } else {
                space_x = 0.0;
            }
            if item == &String::from("cmd") {
                // println!("from editor: cmd is visable");
            }
        }
        let font_size = (BASE_FONT_SIZE - self.scale) * hidpi_factor;
        self.font_size = font_size;
        let screen_dims = display.get_framebuffer_dimensions();
        self.letter_size = self.glyph_brush
            .glyph_bounds(Section {
                text: "0",
                // scale: glyph_brush::rusttype::Scale::uniform(5 as f32),
                scale: glyph_brush::rusttype::Scale::uniform(font_size),
                ..Section::default()
            })
            .unwrap();
        self.viewport_rows = (screen_dims.1 as f32 / self.font_size) as usize;

        let content_to_draw = self
            .buffer
            .get_lines(self.offset_y, self.offset_y + self.viewport_rows);

        self.glyph_brush.queue(Section {
            text: &content_to_draw,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: (((self.padding / 2.0) + space_x), (self.padding / 2.0)),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: ui::color::hex("#E6FFFF").as_slice(),
            ..Section::default()
        });

        self.glyph_brush.queue(Section {
            text: "|", // █
            bounds: (
                screen_dims.0 as f32 - self.padding,
                screen_dims.1 as f32 - self.padding,
            ),
            screen_position: (
                ((self.padding / 2.0) + (self.letter_size.width() * self.cursor.col as f32) - (self.letter_size.width()/2.0) - 1.2) + space_x,
                ((self.padding / 2.0) + (self.font_size * self.cursor.row as f32) - 3.0),
                // (self.padding / 2.0) + (self.letter_size.height() * self.cursor.row as f32),
            ),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size + 4.0),
            color: ui::color::hexa("#3A60D7", 0.8).as_slice(),
            ..Section::default()
        });
    }

    fn draw(&mut self, display: &Display, target: &mut Frame) {
        self.glyph_brush.draw_queued(display, target);
    }

    fn handle_input(
        &mut self,
        key_code: VirtualKeyCode,
        state: ElementState,
        modifiers: ModifiersState,
    ) {
        match (key_code, state, modifiers) {
            (VirtualKeyCode::Down, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_down();
            }
            (VirtualKeyCode::Up, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_up();
            }
            (VirtualKeyCode::Left, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_left();
            }
            (VirtualKeyCode::Right, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_right();
            }
            (VirtualKeyCode::Comma, ElementState::Pressed, CTRL_HOLD) => {
                println!("ooga");
                self.move_to_bol();
            }
            (VirtualKeyCode::Period, ElementState::Pressed, CTRL_HOLD) => {
                println!("booga");
                self.move_to_eol();
            }
            // (VirtualKeyCode::Down, ElementState::Pressed, CTRL_HOLD) => {
            //     self.scroll_down(10);
            // }
            // (VirtualKeyCode::Up, ElementState::Pressed, CTRL_HOLD) => {
            //     self.scroll_up(10);
            // }
            (VirtualKeyCode::Subtract, ElementState::Pressed, CTRL_HOLD) => {
                println!("scale:{}",self.scale);
                self.scale += 1 as f32;
            }
            (VirtualKeyCode::Equals, ElementState::Pressed, CTRL_HOLD) => {
                println!("scale:{}", self.scale);
                self.scale -= 1 as f32;
            }

            _ => (),
        }
    }

    fn handle_mouse_input(&mut self, button:glium::glutin::MouseButton) -> (&str, String){
        println!("click");
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
        // println!("from editor:{}, {}", position.x, position.y);
        // return 0;
        self.mouse_pos = position;
    }
    
// 
    fn push_char(&mut self, c: char) {
        self.buffer
            .insert_char(self.offset_y + self.cursor.row as usize, self.cursor.col as usize, c);
        self.move_cursor_right();
    }

    fn pop_char(&mut self) {
        let (row,col) = (self.cursor.row as usize, self.cursor.col as usize);
        self.move_cursor_left();
        self.buffer
            .remove_char(self.offset_y + row, col);
    }

    fn update_value(&mut self, value: String, check: &str) {
        if check == "editor" {
            self.buffer = Buffer::new(&value);
            self.cursor.col = 0;
            self.cursor.row = 0;
        } else {
            println!("welp fuck")
        }
    }

    fn is_visible(&mut self) -> (bool, &str) {
        return (true, "editor");
    }

    fn on_other_visible(&mut self, visible: &Vec<String>) {
        self.visible = visible.to_vec();
        // for item in visible.iter() {
        //     if item == &String::from("file") {
        //         // println!("from editor: file is visable");
        //     }
        //     if item == &String::from("cmd") {
        //         // println!("from editor: cmd is visable");
        //     }
        // }
    }
}

        // let whitespace_content_to_draw = self
        //     .buffer
        //     .get_lines(self.offset_y, self.offset_y + self.viewport_rows);

        // self.glyph_brush.queue(Section {
        //     text: &whitespace_content_to_draw,
        //     bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
        //     screen_position: ((self.padding / 2.0), (self.padding / 2.0)),
        //     scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
        //     color: ui::color::hex("#293940").as_slice(),
        //     ..Section::default()
        // });
