use raylib::prelude::*;
use crate::grid_builder::Grid;
use crate::color_config::color_scheme;
pub fn draw_med_button(pos_x: i32, pos_y: i32, canvas_grid: &Grid, words: String, d: &mut RaylibDrawHandle, my_font: &Font)
{
    let translated_pos_x = (pos_x as f32 -1.0) * canvas_grid.x_cell_width;
    let translated_pos_y = (pos_y as f32 -1.0) * canvas_grid.y_cell_height;

    // Shadow
    let button_shadow = Rectangle::new(translated_pos_x + 2.5, translated_pos_y + 2.5, canvas_grid.x_cell_width * 7.0, canvas_grid.y_cell_height * 7.0); 
    d.draw_rectangle_rounded(button_shadow,1.0,10,color_scheme::DARK_GREEN); 

    let button = Rectangle::new(translated_pos_x, translated_pos_y, canvas_grid.x_cell_width * 7.0, canvas_grid.y_cell_height * 7.0); 
    d.draw_rectangle_rounded(button,1.0,10,color_scheme::SECONDARY_GREEN); 
     // Get the current words width and height

    let mut current_font_size = 50.0; 
        // Use a loop to scale the words down until it fits inside the rectangle
        loop {
            // Calculate the words width and height using the current font size
            let text_width = d.measure_text(&words, current_font_size as i32);
            let text_height = current_font_size * 1.5;

            // Break the loop if the words fits within the rectangle
            if text_width <= button.width as i32 && text_height <= button.height {
                break;
            }
            // If the width is not changing anymore (i.e., we've hit the minimum size)
            if text_width == 43 && current_font_size <= 1.0 {
                break; // Exit the loop if the text width is fixed at a minimum
            }
            // Decrease the font size if the words does not fit
            current_font_size -= 1.0;
        }

    // Calculate the position for the words to be centered inside the rectangle
    let final_text_width = d.measure_text(&words, current_font_size as i32);
    let final_text_height = current_font_size * 1.5;
    // Something is odd with font positioning. This won't be perfect
    let x_offset = (button.width - final_text_width as f32) / 2.0 ;
    let y_offset = (button.height - final_text_height) * 3.5;
    let pos = Vector2 { x: translated_pos_x + x_offset, y: translated_pos_y + y_offset};
    d.draw_text_ex(my_font, &words, pos, current_font_size, 0.0, color_scheme::TEXT_COLOR);
}
