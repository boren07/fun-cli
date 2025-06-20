use std::cmp::min;
use crate::error::CliError;
use crate::impls::handlers::CommandHandler;
use clap::Parser;
use reqwest::blocking::Client;
use rodio::{Decoder, OutputStream, Sink, Source};
use rustfft::num_complex::{Complex, ComplexFloat};
use rustfft::FftPlanner;
use serde::Deserialize;
use std::io::{BufReader, Cursor, Read, Seek};
use std::ops::Deref;
use std::time::{Duration, Instant};
use colored::Colorize;
use url::form_urlencoded;
use crate::utils::chars;

#[derive(Debug,Parser)]
pub struct MusicHandler {
    #[arg(required = true, help = "输入你想查询的音乐名称")]
    name: String,

    #[arg(short,long, default_value_t = true, help = "是否需要播放音乐")]
    play: bool,
    //使用action为标记参数，出现则为true
    #[arg(short,long, default_value_t = false, help = "是否循环播放音乐",action=clap::ArgAction::SetTrue)]
    loop_play: bool,
}

#[derive(Debug,Default,Deserialize)]
struct NetCloudMusic {

    #[serde(default)]
    title: String,
    #[serde(default)]
    artist: String,
    #[serde(default)]
    album: String,
    #[serde(default)]
    lyric: String,

    #[serde(default)]
    link: String,
}
const BARS: [char; 8] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇'];

impl MusicHandler {

    const MUSIC_API: &'static str = "https://api.bakaomg.cn/v1/music/netease/search?keyword={keyword}";


    pub fn new(name: String,  play: bool, loop_play: bool)->  Self {
        Self {
            name,
            play,
            loop_play,
        }
    }

    pub fn get_internet_music(&self,client: &Client) -> Result<NetCloudMusic, Box<dyn std::error::Error>>
    {
        let name = form_urlencoded::byte_serialize((&self.name).as_bytes()).collect::<String>();
        let response = client.get(Self::MUSIC_API.replace("{keyword}",&name)).send()?.error_for_status()?;
        let res: serde_json::Value = response.json()?;
        if let Some(data) = res["data"].as_object() {
            if let Some(list) = data["list"].as_array() {
                if let Some(item) = list.first() {
                    let music: NetCloudMusic  = serde_json::from_value(item.clone())?;
                    return Ok(music)
                }

            }
        }
        Err("获取音乐信息失败！".into())
    }
    fn get_music_binary(url: &str, client: &Client) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut response = client.get(url).send()?.error_for_status()?;
        let mut audio_data = Vec::new();
        response.copy_to(&mut audio_data)?;
        Ok(audio_data)
    }
    fn create_decoder(data: &Vec<u8>)-> Result<Decoder<BufReader<Cursor<Vec<u8>>>>, Box<dyn std::error::Error>> {
        // 使用Cursor将Vec<u8>转换为可读流
        let cursor = Cursor::new(data.clone());
        let decoder = Decoder::new(BufReader::new(cursor))?;
        Ok(decoder)
    }
    // 修正后的真实频谱计算函数签名
    fn compute_real_spectrum(pcm: &[f32], fft: &dyn rustfft::Fft<f32>, window_size: usize) -> Vec<f32> {
        let mut buffer = pcm.iter()
            .enumerate()
            .map(|(i, &x)| {
                let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / window_size as f32).cos());
                Complex::new(x * window, 0.0)
            })
            .collect::<Vec<_>>();
        if pcm.len() < window_size {
            return vec![0.0; window_size];
        }
        fft.process(&mut buffer);

        buffer[..window_size/2]
            .iter()
            .map(|c| 10.0 * c.norm().log10().max(0.0))
            .collect()
    }

    // 渲染函数保持不变
    fn render_ascii(spectrum: &[f32], width: usize) -> String {
        let chunk_size = spectrum.len() / width;
        let height_chars = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', ' ', ' ', ' '];

        (0..width).map(|i| {
            let start = i * chunk_size;
            let end = (i + 1) * chunk_size;
            let avg = spectrum[start..end].iter().sum::<f32>() / chunk_size as f32;
            let height = (avg.clamp(0.0, 1.0) * (height_chars.len() - 1) as f32).round() as usize;
            height_chars[height]
        }).collect()
    }

}

impl CommandHandler for MusicHandler {
    fn run(&self) -> Result<(),CliError> {
        let client = Client::new();
        let music = self.get_internet_music(&client)?;
        println!("🎶 加载音频地址: {}", &music.link);
        let binary = Self::get_music_binary(&music.link, &client)?;
        let decoder = Self::create_decoder(&binary)?;
        let total_seconds = decoder.total_duration().ok_or_else(|| CliError::UnknownError("无法获取音频时长!!!".to_owned()))?.as_secs();
        let sample_rate = decoder.sample_rate() as usize;

        println!("✅ 音频加载完成 (时长: {}分{}秒, 采样率: {}Hz)", total_seconds /60, total_seconds %60, sample_rate);
        //转换成f32
        let source_for_process = decoder.convert_samples::<f32>();
        let samples: Vec<f32> = source_for_process.collect();
        // 3. 创建播放器(根据参数决定是否播放)
        let (sink, _stream) = if self.play {
            println!("🔊 正在初始化播放器...");
            let (stream, handle) = OutputStream::try_default()?;
            let sink = Sink::try_new(&handle).map_err(|e| CliError::UnknownError("播放音频出现错误!!!".to_owned()))?;
            sink.append(Self::create_decoder(&binary)?);
            (Some(sink), Some(stream))
        } else {
            println!("⚠️ 静默模式: 仅显示频谱不播放声音");
            (None, None)
        };

        println!("🎶 播放中 (按 Ctrl+C 停止)\n");
        print!("{}", chars::CLEAR); // 清屏并重置光标

        // 4. 初始化FFT
        let mut planner = FftPlanner::<f32>::new();
        let window_size = 1024;
        let fft = planner.plan_fft_forward(window_size);

        // 置顶布局
        print!("\x1B[1;1H");
        println!("🎹  实时频谱:{}({}):-{}", &music.title,&music.album,&music.artist);
        print!("\x1B[3;1H");
        println!("播放进度:");

        // 5. 主循环
        let mut start_time = Instant::now();
        let mut is_playing = true;

        while is_playing {
            let elapsed = start_time.elapsed().as_secs_f32();
            let progress = (elapsed / total_seconds as f32).min(1.0);

            // 计算当前音频位置
            let pos = (progress * samples.len() as f32) as usize;
            let end_pos = (pos + window_size).min(samples.len());
            // 获取当前音频片段并计算频谱
            let chunk = &samples[pos..end_pos];
            let spectrum = Self::compute_real_spectrum(chunk, &*fft, window_size);
            let ascii_bars = Self::render_ascii(&spectrum, 50);

            // 更新显示
            print!("\x1B[2;1H\x1B[2K");
            print!("{}",ascii_bars.bright_green().bold());
            print!("\x1B[4;1H\x1B[2K");
            println!("{:.1}% [{}{}] {:.1}/{:.1}s",
                     progress * 100.0,
                     "=".repeat((progress * 50.0) as usize),
                     " ".repeat(50 - (progress * 50.0) as usize),
                     elapsed,
                     total_seconds
            );
            std::thread::sleep(Duration::from_millis(50));

            // 检查是否结束
            if progress >= 1.0 || sink.as_ref().map_or_else(|| false, |sink| sink.empty()) {
                is_playing = false;
                print!("\x1B[2;1H\x1B[2K");
                println!("{}", "██████████████████████████████████████████████████".bright_green().bold());
                print!("\x1B[4;1H\x1B[2K");
                println!("100.0% [==================================================] {:.1}/{:.1}s",
                         total_seconds,
                         total_seconds
                );
                if self.loop_play {
                    // 重置播放器
                    if let Some(sink) = &sink {
                        sink.stop();
                        sink.append(Self::create_decoder(&binary)?);
                    }
                    is_playing = true;
                    start_time = Instant::now();
                    continue;
                } else {
                    is_playing = false;
                }
            }

        }
        println!("\n🎉 {}！", if !self.play { "分析完成" } else { "播放完成" });
        // 确保播放完全停止
        if let Some(sink) = sink {
            sink.sleep_until_end();
        }
        Ok(())
    }

}
// 仅测试编译该模块
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_play() {
        let music = MusicHandler::new("富士山下".to_string(), true,true);
        music.run().unwrap();
    }
    #[test]
    fn test_colored_print() {
        println!("blue:{}-red:{}:on_bright_green:{}", "hello world".blue(), "hello world".red(), "hello world".on_bright_green());
        println!("green bold:{},bright_red italic:{}", "hello world".green().bold(),"italic".bright_red().italic());
    }

}

