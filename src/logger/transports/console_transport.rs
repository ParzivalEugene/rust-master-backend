use crate::logger::{
    enums::LogLevel,
    types::{RawLogModel, RequestLog, RequestLogModel, ResponseLog, ResponseLogModel},
    Logger,
};
use axum::http::StatusCode;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct ConsoleLogger {}

impl Logger for ConsoleLogger {
    fn log_raw(&self, message: Option<String>, service: String, level: LogLevel) {
        let message = RawLogModel {
            level,
            message,
            timestamp: chrono::Utc::now().naive_utc(),
            service,
        }
        .format();
        match level {
            LogLevel::ERROR => eprintln!("{}", message),
            _ => println!("{}", message),
        }
    }

    fn log_request(&self, request: RequestLog) {
        let message = RequestLogModel {
            raw: RawLogModel {
                level: LogLevel::INFO,
                message: None,
                timestamp: chrono::Utc::now().naive_utc(),
                service: "request".to_string(),
            },
            method: request.method,
            url: request.url,
            headers: request.headers,
        }
        .format();
        println!("{}", message);
    }

    fn log_reponse(&self, response: ResponseLog) {
        let message = ResponseLogModel {
            raw: RawLogModel {
                level: LogLevel::INFO,
                message: None,
                timestamp: chrono::Utc::now().naive_utc(),
                service: "response".to_string(),
            },
            status: StatusCode::from_u16(response.status)
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            headers: response.headers,
        }
        .format();
        println!("{}", message);
    }
}

trait Format {
    fn format(&self) -> String;
}

impl Format for RawLogModel {
    fn format(&self) -> String {
        format!(
            "[{}] [{}] [{}] - {}",
            self.timestamp.time(),
            self.level.to_color(),
            self.service.to_uppercase().blue(),
            self.message.as_ref().unwrap_or(&String::new())
        )
    }
}

impl Format for RequestLogModel {
    fn format(&self) -> String {
        let method = match self.method.as_str() {
            "GET" => self.method.green(),
            "POST" => self.method.yellow(),
            "PUT" => self.method.blue(),
            "DELETE" => self.method.red(),
            _ => self.method.purple(),
        };
        format!(
            "[{}] [{}] [{}] - {} {} {:?}",
            self.raw.timestamp.time(),
            self.raw.level.to_color(),
            self.raw.service.to_uppercase().purple(),
            method,
            self.url,
            self.headers
        )
    }
}

impl Format for ResponseLogModel {
    fn format(&self) -> String {
        let status = match self.status.as_u16() {
            200..=299 => self.status.to_string().green(),
            300..=399 => self.status.to_string().yellow(),
            400..=499 => self.status.to_string().bright_red(),
            500..=599 => self.status.to_string().red(),
            _ => self.status.to_string().purple(),
        };
        format!(
            "[{}] [{}] [{}] - {} {:?}",
            self.raw.timestamp.time(),
            self.raw.level.to_color(),
            self.raw.service.to_uppercase().purple(),
            status,
            self.headers
        )
    }
}

pub trait Color {
    fn to_color(self) -> colored::ColoredString;
}

impl Color for LogLevel {
    fn to_color(self) -> colored::ColoredString {
        match self {
            LogLevel::INFO => "INFO".green(),
            LogLevel::ERROR => "ERROR".red(),
            LogLevel::DEBUG => "DEBUG".blue(),
            LogLevel::WARN => "WARN".yellow(),
            LogLevel::TRACE => "TRACE".purple(),
        }
    }
}
