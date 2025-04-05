#[derive(Debug)]
pub struct Grid 
{
    pub x_count: i32,
    pub y_count: i32,
    pub x_cell_width: f32,
    pub y_cell_height: f32,
}

impl Grid 
{
    pub fn new(x_count: i32, y_count: i32, resolution_x: i32, resolution_y: i32) -> Self
    {
        let temp_x_width = (resolution_x as f32 / x_count as f32).ceil();
        let temp_y_height = (resolution_y as f32 / y_count as f32).ceil();
        Grid {x_count, y_count, x_cell_width: temp_x_width, y_cell_height: temp_y_height}
    }
}
