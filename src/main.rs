use druid::{AppLauncher, WindowDesc,PlatformError};
mod data;
mod view;
use data::AppState;
use view::build_ui;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui);
    let initial_state:AppState=AppState{ name:"".into()};
    AppLauncher::with_window(main_window).use_simple_logger()
        
        .launch(initial_state)
}





 





   


    





