use clap::{Command, Parser, Subcommand};
use crate::error::CliError;
use crate::impls::curl::CurlHandler;
use crate::impls::music::MusicHandler;
use crate::impls::handlers::{CombineHandler, CommandHandler};
use crate::impls::weather::WeatherHandler;

#[derive(Debug, Parser)]
#[command(name = "fun", author, version, about, long_about)]
pub struct FunCli {

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    //天气系统
    #[command(name = "weather", version, about = "天气系统")]
    Weather(WeatherHandler),
    //音乐系统
    #[command(name = "music", version, about = "音乐系统")]
    Music(MusicHandler),

    #[command(name = "curl", version, about = "curl系统")]
    Curl(CurlHandler),
}

impl Commands {
    /// 执行子命令
    pub fn run(self) {
        let combine_handlers = CombineHandler::new();
        match combine_handlers.matches_handler(self) {
            Ok(handler) => {
                if let Err(cli_err) = handler.run() {
                    eprintln!("error:{}", cli_err);
                }
            }
            Err(cli_err) => {
                eprintln!("error:{}", cli_err)
            }
        }
    }
}
