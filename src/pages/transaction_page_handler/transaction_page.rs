use crate::ui_legos::buttons::*;
use crate::ui_legos::inputs::*;
use raylib::prelude::*;
use crate::grid_builder::Grid;
pub fn draw_transaction_page(d: &mut RaylibDrawHandle, canvas_grid: &Grid, my_font: &Font)
{
    draw_med_button(4, 20, canvas_grid, String::from("Big"), d, my_font);
    draw_input_box(6, 6, canvas_grid, d, my_font);
}




