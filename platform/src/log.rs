use crate::makepad_micro_serde::*;

#[macro_export]
macro_rules!log {
    ( $ ( $ t: tt) *) => {
        $crate::log::log_with_level(file!(), line!(), column!()+1, line!(), column!() + 4, &format!( $ ( $ t) *), $ crate::log::LogLevel::Log)
    }
}

#[macro_export]
macro_rules!error {
    ( $ ( $ t: tt) *) => {
        $crate::log::log_with_level(file!(), line!(), column!()+1, line!(), column!() + 4, &format!( $ ( $ t) *), $ crate::log::LogLevel::Error)
    }
}

#[macro_export]
macro_rules!warning {
    ( $ ( $ t: tt) *) => {
        $crate::log::log_with_level(file!(), line!(), column!()+1, line!(), column!() + 4, &format!( $ ( $ t) *), $ crate::log::LogLevel::Warning)
    }
}


#[derive(Clone, PartialEq, Eq, Copy, Debug, SerBin, DeBin)]
pub enum LogLevel{
    Warning,
    Error,
    Log,
    Wait,
    Panic,
}

use crate::cx::Cx;
use crate::studio::AppToStudio;

pub fn log_with_level(file_name:&str, line_start:u32, column_start:u32, line_end:u32, column_end:u32, message:&str, level:LogLevel){
    // lets send out our log message on the studio websocket 
    
    /*if std::env::args().find(|v| v == "--message-format=json").is_some(){
        let out = ty.make_json(file, line_start, column_start, line_end, column_end, message);
        println!("{}", out);
        return
    }*/
    
    let studio_http: Option<&'static str> = std::option_env!("MAKEPAD_STUDIO_HTTP");
    if studio_http.is_none() {
        println!("{}:{}:{} - {}", file_name, line_start, column_start, message);
    }
    else{
        Cx::send_studio_message(AppToStudio::Log{
            file_name: file_name.to_string(),
            line_start,
            column_start,
            line_end,
            column_end,
            message:message.to_string(),
            level
        });
        }
}
// alright let log