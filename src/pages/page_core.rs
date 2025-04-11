use crate::ui_legos::ui_core::*;
use crate::ui_legos::inputs::*;
use crate::ui_legos::buttons::*;
use raylib::prelude::*;
use crate::grid_builder::Grid;
use crate::pages::transaction_page_handler::*;
pub enum PageEnum
{
    Transaction,
}

pub struct StructPage
{
    pub id: PageEnum,
    pub elements: Vec<UiElement>,
    pub layers: [i32; 4],
    pub instantiated: bool,
}


pub fn draw_element_tree(grid: &Grid, d: &mut RaylibDrawHandle, my_font: &Font, element: &UiElement)
{
    match element.ui_type
    {
        UiElementEnum::MedButton => draw_med_button(grid, d, my_font, element),
        UiElementEnum::MedInputBox => draw_input_box(grid, d, my_font, element),
        _ => println!("Draw_Element_Tree: Bad Ui_Type")    
    }

}

// Create and return the page struct for main to use to draw the right page
pub fn init_page_tree(page: PageEnum) -> StructPage
{
    match page
    {
        PageEnum::Transaction => init_transaction_page() 
    }

}

// Draw page based on page id type
pub fn draw_page_tree( grid: &Grid, d: &mut RaylibDrawHandle, my_font: &Font, page: &StructPage)
{
    match page.id
    {
        PageEnum::Transaction => draw_transaction_page(grid, d, my_font, page)
    }
}
