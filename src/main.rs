use raylib::prelude::*;
mod color_config;
mod grid_builder;
mod pages;
mod ui_legos;
use pages::transaction_page_handler::*;
use pages::page_core::*;
fn main() {
    // Initialize the window
    let screen_width = 960;
    let screen_height = 540;
    let x_grid_count = 50; 
    let y_grid_count = 50; 
    let canvas_grid = grid_builder::Grid::new(x_grid_count, y_grid_count,screen_width,screen_height);
    let (mut rl, thread) = raylib::init()
        .size(screen_width,screen_height)
        .title("Raylib Button Example")
        .build();
    let my_font = rl.load_font(&thread, "resources/Lato.ttf").unwrap();
    let mut current_page = StructPage {id: PageEnum::Transaction, elements: Vec::new(), layers: [0,0,0,0], instantiated: false };
    // Main loop
    while !rl.window_should_close() {
        // Start drawing
        let mut d = rl.begin_drawing(&thread);

        // Clear the screen with background color 
        d.clear_background(color_config::color_scheme::BACKGROUND_GREEN);
        // Debug looop
/*        for x in 0..canvas_grid.x_count
        {
            for y in 0..canvas_grid.y_count
            {
                // Debug Grid 
                d.draw_rectangle_lines(x * canvas_grid.x_cell_width as i32 , y * canvas_grid.y_cell_height as i32, canvas_grid.x_cell_width as i32, canvas_grid.y_cell_height as i32, Color::SKYBLUE);
            }
        }
*/
        // Get the mouse position
       ////////////////// let mouse_pos = d.get_mouse_position();
        if !&current_page.instantiated 
        {
            current_page = init_page_tree(PageEnum::Transaction)
        }

        draw_page_tree(&canvas_grid, &mut d, &my_font, &current_page)
        // Check if the mouse is inside button 1
        /*if is_point_in_rectangle(mouse_pos.x, mouse_pos.y, button_1)
            && d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            println!("Button 1 clicked!");
        }

        // Check if the mouse is inside button 2
        if is_point_in_rectangle(mouse_pos.x, mouse_pos.y, button_2)
            && d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            println!("Button 2 clicked!");
        }
        */
    }
}

// Helper function to check if a point is inside a rectangle
fn is_point_in_rectangle(px: f32, py: f32, rect: Rectangle) -> bool {
    px >= rect.x && px <= rect.x + rect.width && py >= rect.y && py <= rect.y + rect.height
}


