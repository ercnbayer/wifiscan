use druid::Env;
use druid::WidgetExt;

use druid::{AppLauncher, WindowDesc,PlatformError};



use druid::widget::Flex;
use druid::{widget::Label, Widget};
use druid::widget::Button;
use druid::Data;

extern crate wifiscanner;
#[derive(Clone, Data)]
struct AppState {
    name: String,
}


 fn build_ui() -> impl  Widget<AppState> {
  
    let label = Label::new(|data: &AppState, _env: &Env|{      format!( "{}", data.name)});
    let button = Button::new("Scan").on_click(|_ctx, data, _env|{

let data_wifi=wifiscanner::scan().unwrap();
let mut string_data:String="".to_string();
for hotspot in data_wifi{
   string_data.push_str(&hotspot.ssid);
   string_data.push_str(" ");
   string_data.push_str(&hotspot.signal_level);
   string_data.push_str(" \n");
 
}*data =AppState{name:string_data};}).padding(5.0);  
Flex::column().with_child(label).with_child(button)}   

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui);
    let initial_state:AppState=AppState{ name:"".into()};
    AppLauncher::with_window(main_window).use_simple_logger()
        
        .launch(initial_state)
}
