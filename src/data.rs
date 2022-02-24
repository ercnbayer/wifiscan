//use std::thread;
use druid::Data;
#[derive(Clone, Data)]
 pub struct AppState  {
    pub name: String,
}
impl AppState{
pub fn wifi_scan(data:&mut Self)->&mut Self{
    
   
    let data_wifi=wifiscanner::scan().unwrap();
    let mut string_data:String="".to_string();
    for hotspot in data_wifi{
       string_data.push_str(&hotspot.ssid);
       string_data.push_str(" ");
       string_data.push_str(&hotspot.signal_level);
       string_data.push_str(" \n");
     
    }*data =AppState{name:string_data};
    data
}

 
 
} 
pub fn wifi_scan_to_string()->String
{
    
       let data_wifi=wifiscanner::scan().unwrap();
    let mut string_data:String="".to_string();
    for hotspot in data_wifi{
       string_data.push_str(&hotspot.ssid);
       string_data.push_str(" ");
       string_data.push_str(&hotspot.signal_level);
       string_data.push_str(" \n");

   }string_data}