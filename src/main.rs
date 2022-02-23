
use core::time::Duration;
use crate::data::wifi_scan_to_string;

use druid::{AppLauncher, WindowDesc,PlatformError};
mod data;
mod view;
use data::AppState;
use view::build_ui;
use std::thread;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui());
    let initial_state:AppState=AppState{ name:wifi_scan_to_string().into()};
    let launcher=AppLauncher::with_window(main_window);
    
    let event_sink = launcher.get_external_handle();
    thread::spawn(move || wifi_update(event_sink));

    launcher.log_to_console()
        
        .launch(initial_state)
}


fn wifi_update(event_sink: druid::ExtEventSink) {
    
    loop {
       
     
        let  data_wifi = wifi_scan_to_string();

   
      

        let data_wifi_clone = data_wifi.clone();
        
        event_sink.add_idle_callback(move |data: &mut AppState| {
           
            *data =AppState { name: data_wifi_clone };
          
        });
        thread::sleep(Duration::from_millis(20));
    }
}








   


    





