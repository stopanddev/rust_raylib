use raylib::prelude::*;
mod color_config;
mod grid_builder;
mod pages;
mod ui_legos;
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
    println!("Loaded font");
    // Define button dimensions and positions
    //let button_1 = Rectangle::new(100.0, 100.0, 200.0, 50.0);

    // Main game loop
    while !rl.window_should_close() {
        // Start drawing
        let mut d = rl.begin_drawing(&thread);

        // Clear the screen with a color (dark gray)
        d.clear_background(color_config::color_scheme::BACKGROUND_GREEN);

        for x in 0..canvas_grid.x_count
        {
            for y in 0..canvas_grid.y_count
            {
                // Debug Grid 
//                d.draw_rectangle_lines(x * canvas_grid.x_cell_width as i32 , y * canvas_grid.y_cell_height as i32, canvas_grid.x_cell_width as i32, canvas_grid.y_cell_height as i32, Color::GRAY);
            }
        }
        // Get the mouse position
       ////////////////// let mouse_pos = d.get_mouse_position();
        println!("Drawing Page");
        pages::transaction_page::draw_transaction_page(&mut d,&canvas_grid, &my_font);
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

        // Draw the buttons
        d.draw_rectangle_rec(button_1, Color::BLUE);
        d.draw_rectangle_rec(button_2, Color::GREEN);

        // Draw text on the buttons
        d.draw_text("Button 1", button_1.x as i32 + 70, button_1.y as i32 + 15, 20, Color::WHITE);
        d.draw_text("Button 2", button_2.x as i32 + 70, button_2.y as i32 + 15, 20, Color::WHITE);
        */
    }
}

// Helper function to check if a point is inside a rectangle
fn is_point_in_rectangle(px: f32, py: f32, rect: Rectangle) -> bool {
    px >= rect.x && px <= rect.x + rect.width && py >= rect.y && py <= rect.y + rect.height
}


