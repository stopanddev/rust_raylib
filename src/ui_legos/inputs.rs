use raylib::prelude::*;
use crate::grid_builder::Grid;
use crate::color_config::color_scheme;
pub fn draw_input_box(pos_x: i32, pos_y: i32, canvas_grid: &Grid, d: &mut RaylibDrawHandle, my_font: &Font)
{
    let translated_pos_x = (pos_x as f32 -1.0) * canvas_grid.x_cell_width;
    let translated_pos_y = (pos_y as f32 -1.0) * canvas_grid.y_cell_height;

    let input_box = Rectangle::new(translated_pos_x, translated_pos_y, canvas_grid.x_cell_width * 12.0, canvas_grid.y_cell_height * 2.0); 
    d.draw_rectangle(input_box.x as i32, input_box.y as i32, input_box.width as i32 , input_box.height as i32,Color::LIGHTGRAY); 

}
