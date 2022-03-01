// mod buffer;
// mod cursor;
use walkdir::WalkDir;
use glium::glutin::{ElementState, ModifiersState, VirtualKeyCode};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::rusttype::Rect;
use glium_glyph::glyph_brush::{rusttype::Font, GlyphCruncher, Section};
use glium_glyph::GlyphBrush;
// use syntect::easy::HighlightLines;
// use syntect::parsing::SyntaxSet;
// use syntect::highlighting::{ThemeSet, Style};
// use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// use buffer::Buffer;
// use cursor::Cursor;

// use crate::constants::{BASE_FONT_SIZE, CTRL_HOLD, NO_MODIFIERS, SHIFT_HOLD};
use crate::layout_manager::View;
use crate::ui;

pub struct FileView<'a, 'b> {
    // pub buffer: Buffer,
    // cursor: Cursor,
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    offset_y: usize,
    viewport_rows: usize,
    letter_size: Rect<f32>,
    last_column: i32,
    scale: f32,
}

impl<'a, 'b> FileView<'a, 'b> {
    pub fn new(file: &str, display: &Display) -> FileView<'a, 'b> {
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let mut scale = 0 as f32;
        let mut font_size = (BASE_FONT_SIZE - scale) * hidpi_factor;
        let mut gb = GlyphBrush::new(display, fonts);
        let __display = display;
        // let __target = target;
        let letter_size = gb
            .glyph_bounds(Section {
                text: "0",
                // scale: glyph_brush::rusttype::Scale::uniform(5 as f32),
                scale: glyph_brush::rusttype::Scale::uniform(font_size),
                ..Section::default()
            })
            .unwrap();

        FileView {
            // buffer: Buffer::new(file),
            // cursor: Cursor::new(),
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
        }
    }

impl<'a, 'b> View for FileView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let mut font_size = (BASE_FONT_SIZE - self.scale) * hidpi_factor;
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

        for entry in WalkDir::new("./assets") {
            println!("{}", entry?.path().display());
        }
        // let content_to_draw = self
        //     .buffer
        //     .get_lines(self.offset_y, self.offset_y + self.viewport_rows);
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

        self.glyph_brush.queue(Section {
            text: &content_to_draw,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: ((self.padding / 2.0), (self.padding / 2.0)),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: ui::color::hex("#E6FFFF").as_slice(),
            ..Section::default()
        });

        self.glyph_brush.queue(Section {
            text: "█",
            bounds: (
                screen_dims.0 as f32 - self.padding,
                screen_dims.1 as f32 - self.padding,
            ),
            screen_position: (
                (self.padding / 2.0) + (self.letter_size.width() * self.cursor.col as f32),
                (self.padding / 2.0) + (self.font_size * self.cursor.row as f32),
                // (self.padding / 2.0) + (self.letter_size.height() * self.cursor.row as f32),
            ),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: ui::color::hexa("#3A60D7", 0.4).as_slice(),
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

    fn handle_mouse_input(&mut self, button:glium::glutin::MouseButton){
        println!("click");
        match (button) {
            glium::glutin::MouseButton::Left => {
                println!("left click");
            }
            glium::glutin::MouseButton::Right => {
                println!("right click");
            }
            glium::glutin::MouseButton::Middle => {
                println!("Middle click");
            }
            glium::glutin::MouseButton::Other(i) => {
                println!("other{} click",i);
            }
            _ => println!("click"),
        }
    }

    fn update_mouse_pos(&mut self, position:glium::glutin::dpi::LogicalPosition){
        println!("from editor:{}, {}", position.x, position.y);
    }
    
// 
    fn push_char(&mut self, c: char) {
        return 0;
    }

    fn pop_char(&mut self) {
        return 0;
    }
}


// let mut entries = fs::read_dir(".")?
//         .map(|res| res.map(|e| e.path()))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     // The order in which `read_dir` returns entries is not guaranteed. If reproducible
//     // ordering is required the entries should be explicitly sorted.

//     entries.sort();