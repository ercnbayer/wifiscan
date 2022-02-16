
use druid::WidgetExt;
use druid::widget::Flex;
use druid::{widget::Label, Widget};
use druid::widget::Button;

use crate::data::AppState;
pub fn build_ui() -> impl  Widget<AppState> {
  
    
    let label=Label::new(|data: &AppState, _env: &_|
        
        format! ("{}", data.name));
      
    let button = Button::new("Scan").on_click(|_ctx, data, _env|{
    
    AppState::wifi_scan(data);
    
    }).padding(5.0);

    
    Flex::column().with_child(button).with_child(label)
}    
    
    
      
        
