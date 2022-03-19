use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

pub fn init(debug: bool) {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, get_level_filter(debug)) 
        .init();
} 

fn get_level_filter(debug_mode: bool) -> LevelFilter {
    if debug_mode {
        return LevelFilter::Debug;
    }
    LevelFilter::Info
}