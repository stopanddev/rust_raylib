use crate::ui_legos::buttons::*;
use raylib::prelude::*;
use crate::grid_builder::Grid;
pub fn draw_transaction_page(d: &mut RaylibDrawHandle, canvas_grid: &Grid, my_font: &Font)
{
    draw_med_button(4, 4, canvas_grid, String::from("Big"), d, my_font);
}




