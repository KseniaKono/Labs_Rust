pub mod button;
pub mod label;
pub mod window;

pub use button::Button;
pub use label::Label;
pub use window::Window;
pub trait Widget {
    fn width(&self) -> usize;
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", buffer);
    }
}
