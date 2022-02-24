
extern crate cron;
extern crate chrono;

use druid::Env;
use druid::WidgetExt;
use druid::widget::Flex;
use druid::{widget::Label, Widget};
use druid::widget::Button;
use crate::data::AppState;





pub fn build_ui() -> impl  Widget<AppState> {
  
    
    let  label=Label::dynamic(|data: &AppState, _env: &Env|format! ("{}", data.name));
 
  
    // Set minutes.
    //cron.minutes("0");
    // Start the cronjob.
        
    

    let button = Button::new("Scan").on_click(|_ctx, data, _env|{
      

    AppState::wifi_scan(data);
        println!("buton");
    }).padding(5.0);
  
    Flex::column().with_child(button).with_child(label)
}


        
