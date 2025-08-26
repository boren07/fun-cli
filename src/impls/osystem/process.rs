use std::fmt::Display;
use std::io::Stdout;
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, SetForegroundColor};
use sysinfo::{Process, System};
use crate::ui::Coordinate;
use crate::ui::theme::Theme;
use crate::ui::widget::{List, Widget};
use crate::utils::consts;

pub struct ProcessWidget {
    coordinate: Coordinate,
    width: u16,
    height: u16,
    theme: Theme,
    process_list: Vec<ProcessInfo>,
}
#[derive(Debug)]
struct ProcessInfo {
    name: String,
    pid: u32,
    cpu: f32,
    mem: u64,
}
impl Display for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //<25 左对齐 ，不足25个字符，用空格填充
        // <: 左对齐（默认用于字符串）
        // >: 右对齐（默认用于数字）
        // ^: 居中对齐
        write!(f, "{:<25}{:<10}{:>8.2}%{:>10}MB", self.name, self.pid, self.cpu, self.mem)
    }
}


impl ProcessWidget {
    pub fn new(left_top: Coordinate, right_bottom: Coordinate, theme: Theme, sys: &System) -> Self {
        Self {
            width: (right_bottom.x - left_top.x) + 1,
            height: (right_bottom.y - left_top.y) + 1,
            coordinate: left_top,
            theme,
            process_list: {
                let mut process_list = sys.processes().iter().map(|(pid, process)| ProcessInfo {
                    name: process.name().to_str().unwrap().to_string(),
                    pid: pid.as_u32(),
                    cpu: process.cpu_usage()/ sys.physical_core_count().unwrap() as f32,
                    mem: process.memory() / consts::SIZE_MB,
                }).collect::<Vec<ProcessInfo>>();
                // 按照 CPU 使用率降序排序
                process_list.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
                process_list
            },

        }
    }
}
impl Widget for ProcessWidget {
    fn coordinate(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn render(&self, stdout: &mut Stdout) -> std::io::Result<()> {
        queue!(stdout, SetForegroundColor(self.theme.primary_text_color()))?;
        let (x, y) = (self.coordinate().x + 2, self.coordinate().y);
        let mut list = List::new(Coordinate::new(x, y), Coordinate::new(x + self.width - 2, y + self.height - 2), self.theme.clone());
        self.process_list.iter().for_each(|process| {
            list.add_item(process);
        });
        list.render(stdout)?;
        Ok(())
    }
}