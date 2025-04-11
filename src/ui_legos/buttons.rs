use raylib::prelude::*;
use crate::grid_builder::Grid;
use crate::color_config::color_scheme;
use crate::ui_legos::ui_core::*;
pub struct MedButton
{
    pub label: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub content: String

}
pub fn draw_med_button(canvas_grid: &Grid, d: &mut RaylibDrawHandle, my_font: &Font, element: &UiElement)
{
    let translated_pos_x = (element.pos_x as f32 -1.0) * canvas_grid.x_cell_width;
    let translated_pos_y = (element.pos_y as f32 -1.0) * canvas_grid.y_cell_height;

    // Shadow
    let button_shadow = Rectangle::new(translated_pos_x + 2.5, translated_pos_y + 2.5, canvas_grid.x_cell_width * 5.0, canvas_grid.y_cell_height * 4.0); 
    d.draw_rectangle_rounded(button_shadow,1.0,10,color_scheme::DARK_GREEN); 

    let button = Rectangle::new(translated_pos_x, translated_pos_y, canvas_grid.x_cell_width * 5.0, canvas_grid.y_cell_height * 4.0); 
    d.draw_rectangle_rounded(button,1.0,10,color_scheme::SECONDARY_GREEN); 

    let mut current_font_size = 50.0; 
        // Use a loop to scale the words down until it fits inside the rectangle
        loop {
            // Calculate the words width and height using the current font size
            let text_width = d.measure_text(&element.label, current_font_size as i32);
            let text_height = current_font_size * 1.5;

            // Break the loop if the words fits within the rectangle
            if text_width <= button.width as i32 && text_height <= button.height {
                break;
            }
            // If the width is not changing anymore (i.e., we've hit the minimum size)
            if text_width == 43 && current_font_size <= 1.0 {
                break; 
            }
            // Decrease the font size if the words does not fit
            current_font_size -= 1.0;
        }

    // Calculate the position for the words to be centered inside the rectangle
    let final_text_width = d.measure_text(&element.label, current_font_size as i32);
    let final_text_height = current_font_size * 1.5;
    // Something is odd with font positioning. This won't be perfect
    let x_offset = (button.width - final_text_width as f32) / 2.0 ;
    let y_offset = (button.height - final_text_height) * 3.5;
    let pos = Vector2 { x: translated_pos_x + x_offset, y: translated_pos_y + y_offset};
    d.draw_text_ex(my_font, &element.label, pos, current_font_size, 0.0, color_scheme::TEXT_COLOR);
}
