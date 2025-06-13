use crate::error::CliError;
use crate::impls::handlers::CommandHandler;
use clap::Parser;
use rodio::{Decoder, OutputStream, Sink, Source};
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::fs::File;
use std::io::{sink, Read, Seek};
use std::ops::Deref;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug,Parser)]
pub struct MusicHandler {
    #[arg(short, long, help = "输入你想查询的音乐名称")]
    name: String,

    #[arg(long, default_value_t = true, help = "是否需要播放音乐")]
    play: bool,
}
const BARS: [char; 8] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇'];
// 蓝色频谱的ANSI颜色码
const BLUE_START: &str = "\x1B[34m";
const BLUE_END: &str = "\x1B[0m";
impl MusicHandler {

    pub fn new(name: String,  play: bool)->  Self {
        Self {
            name,
            play,
        }
    }

    pub fn get_internet_source<R>(name: &str) -> R
        where R: Read + Seek + Send + Sync + 'static
    {

    }

    /// 结合 rodio 的播放控制 + 定时频谱刷新
    pub fn play_with_visualization(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = &self.name;
        let play = self.play;
        // 1. 初始化显示
        println!("🎵 加载音频文件: {}...", path);

        // 2. 加载音频并获取准确时长
        let file = File::open(path)?;
        let source = Decoder::new(file)?;
        let total_duration = source.total_duration()
            .ok_or("无法获取音频时长")?
            .as_secs_f32();
        let sample_rate = source.sample_rate() as usize;

        // 重新加载文件用于处理
        let file = File::open(path)?;
        let source_for_process = Decoder::new(file)?.convert_samples::<f32>();
        let samples: Vec<f32> = source_for_process.collect();

        println!("✅ 音频加载完成 (时长: {:.2}秒, 采样率: {}Hz)", total_duration, sample_rate);

        // 3. 创建播放器(根据参数决定是否播放)
        let (sink, _stream) = if play {
            println!("🔊 正在初始化音频设备...");
            let (stream, handle) = OutputStream::try_default()?;
            let sink = Sink::try_new(&handle)?;
            sink.append(Decoder::new(File::open(path)?)?);
            (Some(sink), Some(stream))
        } else {
            println!("⚠️ 静默模式: 仅显示频谱不播放声音");
            (None, None)
        };

        println!("🎶 {} (按 Ctrl+C 停止)\n", if !play { "频谱分析中" } else { "开始播放" });
        print!("\x1B[2J\x1B[H"); // 清屏并重置光标

        // 蓝色频谱的ANSI颜色码
        const BLUE_START: &str = "\x1B[34m";
        const BLUE_END: &str = "\x1B[0m";

        // 4. 初始化FFT
        let mut planner = FftPlanner::<f32>::new();
        let window_size = 1024;
        let fft = planner.plan_fft_forward(window_size);

        // 显示布局
        print!("\x1B[1;1H");
        println!("{}实时频谱:{}", BLUE_START, BLUE_END);
        print!("\x1B[3;1H");
        println!("播放进度:");

        // 5. 主循环
        let start_time = Instant::now();
        let mut is_playing = true;

        while is_playing {
            let elapsed = start_time.elapsed().as_secs_f32();
            let progress = (elapsed / total_duration).min(1.0);

            // 检查是否结束
            if progress >= 1.0 || sink.as_ref().map_or(false, |s| s.empty()) {
                is_playing = false;
            }

            // 计算当前音频位置
            let pos = (progress * samples.len() as f32) as usize;
            if pos + window_size >= samples.len() {
                break;
            }

            // 获取当前音频片段并计算频谱
            let chunk = &samples[pos..pos+window_size];
            let spectrum = Self::compute_real_spectrum(chunk, &*fft, window_size);
            let ascii_bars = Self::render_ascii(&spectrum, 50);

            // 更新显示
            print!("\x1B[2;1H\x1B[2K");
            print!("{}{}{}", BLUE_START, ascii_bars, BLUE_END);
            print!("\x1B[4;1H\x1B[2K");
            println!("{:.1}% [{}{}] {:.1}/{:.1}s",
                     progress * 100.0,
                     "=".repeat((progress * 50.0) as usize),
                     " ".repeat(50 - (progress * 50.0) as usize),
                     elapsed,
                     total_duration
            );

            std::thread::sleep(Duration::from_millis(30));
        }

        // 6. 结束显示
        print!("\x1B[2;1H\x1B[2K");
        println!("{}██████████████████████████████████████████████████{}", BLUE_START, BLUE_END);
        print!("\x1B[4;1H\x1B[2K");
        println!("100.0% [==================================================] {:.1}/{:.1}s",
                 total_duration,
                 total_duration
        );
        println!("\n🎉 {}！", if !play { "分析完成" } else { "播放完成" });

        // 确保播放完全停止
        if let Some(sink) = sink {
            sink.sleep_until_end();
        }
        Ok(())
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
        self.play_with_visualization()?;
        Ok(())
    }

}
// 仅测试编译该模块
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_play() {
        let music = MusicHandler::new("D:\\Downloads\\6 (陈)\\陈奕迅 - 六月飞霜.flac".to_string(), true);
        music.play_with_visualization().unwrap();
    }

}

