extern crate glium;

use glium::glutin::{ElementState, ModifiersState, VirtualKeyCode};
use glium::{Display, Frame};

pub trait View {
    fn update(&mut self, display: &Display);
    fn draw(&mut self, display: &Display, target: &mut Frame);
    fn handle_input(
        &mut self,
        key_code: VirtualKeyCode,
        state: ElementState,
        modifiers: ModifiersState,
    );
    fn handle_mouse_input(&mut self, button:glium::glutin::MouseButton) -> (&str, String);
    fn update_mouse_pos(&mut self, position:glium::glutin::dpi::LogicalPosition);
    fn push_char(&mut self, c: char);
    fn pop_char(&mut self);
    fn update_value(&mut self, value: String, check: &str);
    fn is_visible(&mut self) -> (bool, &str);
    fn on_other_visible(&mut self, visable: &Vec<String>);
}

pub struct LayoutManager {
    pub views: Vec<Box<dyn View>>,
}

impl LayoutManager {
    pub fn update_views(&mut self, display: &Display) {
        for view in self.views.iter_mut() {
            view.update(display);
        }
    }

    pub fn draw(&mut self, display: &Display, target: &mut Frame) {
        for view in self.views.iter_mut() {
            view.draw(display, target);
        }
    }

    pub fn handle_input(
        &mut self,
        key_code: VirtualKeyCode,
        state: ElementState,
        modifiers: ModifiersState,
    ) {
        for view in self.views.iter_mut() {
            view.handle_input(key_code, state, modifiers);
        }
    }

    pub fn handle_mouse_input(&mut self, button:glium::glutin::MouseButton) -> (&str, &str, String) {
        let (mut u, mut uv): (&str, String);
        for view in self.views.iter_mut() {
            // let a: &[usize] = 
            let (a,b) = view.handle_mouse_input(button);
            u = a;
            uv = b;
            println!("output of a:{:?} b {:?}",u, format!("{}",uv));
            match a {
                "file" => {
                    return ("editor", u, format!("{}",uv));
                }
                _ => (println!("empty response;")),
            }
            // if a == None {

            // }
        }
        return ("","",String::new());
    }

    pub fn update_mouse_pos(&mut self, position:glium::glutin::dpi::LogicalPosition){
        for view in self.views.iter_mut() {
            view.update_mouse_pos(position);
        }
    }

    pub fn push_char(&mut self, c: char) {
        for view in self.views.iter_mut() {
            view.push_char(c);
        }
    }

    pub fn pop_char(&mut self) {
        for view in self.views.iter_mut() {
            view.pop_char();
        }
    }
    pub fn visible_views(&mut self) -> Vec<String> {
        let mut arr = vec![];
        // let mut i = 0;
        let view_iter = self.views.iter_mut();
        for view in view_iter {
            let (a,b) = view.is_visible();
            if a {
                arr.push(String::from(b));
            }
            // i = i + 1;
        }
        return arr
    }
    pub fn update_value_views(&mut self) {
        let a = self.visible_views();
        for view in self.views.iter_mut() {
            view.on_other_visible(&a.clone());
        }
    }
}


// pub fn visible_views(&mut self) -> [&str; 3] {
//     let mut arr = ["", "", ""];
//     let mut i = 0;
//     for view in self.views.iter_mut() {
//         let (a,b) = view.is_visible();
//         if a {
//             arr[i] = b;
//         }
//         i = i + 1;
//     }
//     return arr
// }