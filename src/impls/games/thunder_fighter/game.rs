use std::fmt::{Display, Formatter};
use std::io::{stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{cursor, event, execute, queue};
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen};
use crate::error::CliError;
use crate::impls::games::entities::{Entity, GameEntity};
use crate::impls::games::games::Game;
use crate::impls::games::thunder_fighter::entity::Player;

pub struct ThunderFighterGame;
struct ThunderFighterGameState {
    width: u16,
    height: u16,
    difficulty: u8,
    stdout: Stdout,
    player: Player,
    score: u16,
}


impl ThunderFighterGameState {
    pub fn new(width: u16, height: u16, difficulty: u8)-> Self{
        Self {
            width,
            height,
            difficulty,
            stdout: stdout(),
            player: Player{
                entity: Entity{
                    x: width / 2,
                    y: height,
                    display: "✈️".to_string(),
                },
                health: 100,
            },
            score: 0
        }
    }

    fn render(&mut self)-> Result<(), CliError>{
        queue!(self.stdout,Clear(ClearType::All),cursor::MoveTo(0,0))?;
        self.render_player()?;
        // 刷新到终端
        self.stdout.flush()?;
        Ok(())
    }
    fn render_player(&mut self)-> Result<(), CliError>{
        queue!(self.stdout,cursor::MoveTo(self.player.position().0,self.player.position().1), Print(self.player.display()))?;
        Ok(())
    }
    ///
    /// 处理玩家输入
    /// 返回true表示退出游戏
    fn handle_input(&mut self, code: KeyCode)-> Result<bool, CliError>{
        match code {
            KeyCode::Up => {
                self.player.move_to(self.player.position().0, self.player.position().1 - 1)
            }
            KeyCode::Down => {
                self.player.move_to(self.player.position().0, self.player.position().1 + 1)
            }
            KeyCode::Left => {
                self.player.move_to(self.player.position().0 - 1, self.player.position().1)
            }
            KeyCode::Right => {
                self.player.move_to(self.player.position().0 + 1, self.player.position().1)
            }
            KeyCode::Char('q') => {
                return Ok(true);
            }
            _ => {}
        }
        Ok(false)
    }
    fn update(&mut self)-> Result<(), CliError>{
        Ok(())
    }

}
fn poll_input ()-> Result<Option<KeyCode>, CliError>{
    // 非阻塞输入轮询
    if event::poll(Duration::from_millis(0))? {
        if let event::Event::Key(key_event) = event::read()? {
            // 只处理按下事件
            if key_event.kind == KeyEventKind::Press {
                return Ok(Some(key_event.code));
            }
        }
    }
    Ok(None)
}


impl Game for ThunderFighterGame {
    fn name(&self) -> &'static str {
        "雷霆战机✈️"
    }

    fn help(&self) -> &'static str {
        "按上下左右操作飞行器"
    }

    fn run(&self, width: u16, height: u16, difficulty: u8) -> Result<(), CliError> {
        println!("{}运行中", self.name());
        let mut game = ThunderFighterGameState::new(width, height, difficulty);
        // 进入全屏
        // execute!(stdout, EnterAlternateScreen,cursor::Hide)?;
        loop {
            //接收输入
            if let Some(code) = poll_input()? {
                if game.handle_input(code)? {
                    break;
                }
            }
            // 更新画面
            game.update()?;
            // 绘制
            game.render()?;

            // 控制刷新帧率
            sleep(Duration::from_millis(50));
        }
        Ok(())
    }
}