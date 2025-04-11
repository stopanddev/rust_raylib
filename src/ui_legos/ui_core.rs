pub enum UiElementEnum
{
    SmallButton,
    MedButton,
    MedInputBox,
}
pub struct UiElement 
{
    pub label: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub content: String,
    pub visible: bool,
    pub focus: bool,
    pub group: i32,
    pub ui_type: UiElementEnum,
}

impl UiElement
{
    pub fn new(label: String, pos_x: i32, pos_y: i32, content: String, visible: bool, focus: bool, group: i32, ui_type: UiElementEnum) -> Self
    {
        UiElement { label, pos_x, pos_y, content, visible, focus, group, ui_type }
    }
}
