use chrono::Local;
use env_logger::Builder;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::env;

pub fn init_logger() {
    // 读取环境变量 LogCheck 是否为 "true"
    let log_to_file = env::var("LogCheck").unwrap_or_else(|_| "false".to_string()) == "true";

    let mut log_client = Builder::new();
    log_client
        .format(move |buf, record| {
            writeln!(
                buf,
                "[{}] [{}:{}] {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // 时间
                record.file().unwrap_or("unknown"),       // 文件名
                record.line().unwrap_or(0),               // 行号
                record.level(),                           // 日志级别
                record.args()                             // 日志内容
            )
        })
        .write_style(env_logger::WriteStyle::Always) // 始终启用颜色
        .filter_level(log::LevelFilter::Info); // 设置日志级别

    if log_to_file {
        let log_dir = "logs";
        let date_str = Local::now().format("%Y-%m-%d").to_string(); // 获取当前日期（YYYY-MM-DD）
        let log_file_path = format!("{}/{}.log", log_dir, date_str); // 使用日期作为文件名
                                                                     // 确保日志目录存在
        fs::create_dir_all(log_dir).expect("Failed to create log directory");
        // 日志文件的最大大小 50MB
        let max_file_size = 50 * 1024 * 1024; // 50 MB
        if Path::new(&log_file_path).exists() {
            let metadata = fs::metadata(&log_file_path).expect("Failed to read metadata");
            if metadata.len() > max_file_size {
                // 如果文件超过大小限制，进行文件轮转
                let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
                let rotated_file_path = format!("{}/server_{}.log", log_dir, timestamp);
                fs::rename(&log_file_path, rotated_file_path).expect("Failed to rotate log file");
            }
        }
        // 打开日志文件（如果日志文件不存在则创建）
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)
            .expect("Failed to open log file");
        log_client.target(env_logger::Target::Pipe(Box::new(log_file)));
    } else {
        log_client.target(env_logger::Target::Stdout);
    }
    log_client.init()
}
