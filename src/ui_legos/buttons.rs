use raylib::prelude::*;
pub fn draw_med_button(pos_x: f32, pos_y: f32, x_size: f32, y_size: f32, words: String, d: &mut RaylibDrawHandle)
{
    // Shadow
    let button_shadow = Rectangle::new(pos_x+5.0, pos_y+5.0, 32.0, 25.0); 
    d.draw_rectangle_rounded(button_shadow,0.6,10,Color::DARKBLUE); 

    let button = Rectangle::new(pos_x, pos_y, 32.0, 25.0); 
    d.draw_rectangle_rounded(button,0.6,10,Color::BLUE); 
}
