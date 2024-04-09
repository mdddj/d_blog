use chrono::Local;

pub fn get_current_time_as_string() -> String {
    let now = Local::now();
    // 自定义时间格式，可以根据需要调整
    let time_string = now.format("%Y-%m-%d %H:%M:%S").to_string();
    time_string
}
