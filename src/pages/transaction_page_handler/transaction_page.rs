use crate::ui_legos::buttons::*;
use crate::ui_legos::inputs::*;
use crate::ui_legos::ui_core::*;
use crate::pages::page_core::*;
use raylib::prelude::*;
use crate::grid_builder::Grid;


pub fn init_transaction_page() -> StructPage
{
    let id = PageEnum::Transaction;
    let elements = init_ui_elements(); 
    let layers:[i32; 4] = [1,0,0,0]; 
    StructPage { id, elements, layers, instantiated: true }

}


fn init_ui_elements() -> Vec<UiElement>
{
    let element_list: Vec<UiElement> = vec![
       UiElement { label: "Go".to_string(), pos_x: 4, pos_y: 4, content: "".to_string(), group: 0, focus: false, visible: true, ui_type:UiElementEnum::MedButton },
       UiElement { label: "New Transaction".to_string(), pos_x: 9, pos_y: 4, content: "".to_string(), group: 0, focus: false, visible: true,ui_type:UiElementEnum::MedInputBox },
    ];
    element_list

}

pub fn draw_transaction_page(grid: &Grid, d: &mut RaylibDrawHandle, my_font: &Font, page: &StructPage)
{
    for (i, &layer) in page.layers.iter().enumerate()  
    {
        for element in page.elements.iter()
        {
 
            if element.group == i as i32 && layer == 1
            {
                draw_element_tree(grid, d, my_font, element);
            }
        }
    }
}
