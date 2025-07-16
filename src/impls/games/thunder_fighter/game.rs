use std::fmt::{Display, Formatter};
use std::io::{stdout, Stdout, Write};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::{result, thread};
use std::panic::catch_unwind;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;
use crossterm::{cursor, event, execute, queue, terminal};
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::style::{Color, Print, SetBackgroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crate::error::CliError;
use crate::impls::games::entities::{Entity, GameEntity};
use crate::impls::games::games::Game;
use crate::impls::games::thunder_fighter::entity::{Enemy, Player};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub struct ThunderFighterGame;
struct ThunderFighterGameState {
    width: u16,
    height: u16,
    difficulty: u8,
    stdout: Stdout,
    player: Player,
    enemys: Vec<Enemy>,
    score: u16,
    frame_count: u128,
}
pub const PLAYER_DISPLAY: &str = "☮";
pub const PLAYER_BULLET: &str = "♝";
pub const ENEMY_BULLET: &str = "✦";
//"🐱‍🚀" ★☮♝♗ ♕ i⛴⛟✈♞☢♛
// pub const ENEMY_DISPLAY: [&str; 4] = ["🐱‍👤", "‍👓", "🐱‍💻", "🐱‍🐉"];
pub const ENEMY_DISPLAY: [&str; 6] = ["⛴", "‍⛟", "✈", "♞","☢","♛"];

impl ThunderFighterGameState {
    pub fn new(width: u16, height: u16, difficulty: u8) -> Self {
        Self {
            width,
            height,
            difficulty,
            stdout: stdout(),
            player: Player {
                entity: Entity {
                    x: width / 2,
                    y: height / 2,
                    display: PLAYER_DISPLAY.to_string(),
                    width: UnicodeWidthStr::width(PLAYER_DISPLAY) as u16,
                    last_x: width / 2,
                    last_y: height / 2,
                },
                health: 100,
                bullets: vec![],
            },
            enemys: vec![],
            score: 0,
            frame_count: 0
        }
    }
    ///
    /// 游戏实体渲染
    fn render(&mut self) -> Result<(), CliError> {
        self.player.render(&mut self.stdout, self.height)?;
        for enemy in self.enemys.iter_mut() {
            enemy.render(&mut self.stdout, self.height)?;
        }
        //更新敌人数量
        self.enemys.retain(|enemy| !enemy.is_dead());
        self.render_player_score()?;
        // 刷新到终端
        self.stdout.flush()?;
        Ok(())
    }
    ///
    /// 处理玩家输入
    /// 返回true表示退出游戏
    fn handle_input(&mut self, code: KeyCode) -> Result<bool, CliError> {
        match code {
            KeyCode::Up => {
                let mut y = self.player.position().y;
                if y >= 1 {
                    y -= 1;
                }
                self.player.move_to(self.player.position().x, y)
            }
            KeyCode::Down => {
                let mut y = self.player.position().y;
                if y + 1 <= self.height {
                    y += 1;
                }
                self.player.move_to(self.player.position().x, y)
            }
            KeyCode::Left => {
                let width = self.player.width();
                let mut x = self.player.position().x;
                if x >= width {
                    x -= width
                }
                self.player.move_to(x, self.player.position().y)
            }
            KeyCode::Right => {
                let width = self.player.width();
                let mut x = self.player.position().x;
                if x + width <= self.width {
                    x += width
                }
                self.player.move_to(x, self.player.position().y)
            }
            KeyCode::Char(' ') => {
                self.player.attack_bullet();
            }
            KeyCode::Char('q') => {
                return Ok(true);
            }
            _ => {}
        }
        Ok(false)
    }
    /// 更新地图上实体的位置
    fn update(&mut self) -> Result<(), CliError> {
        self.player.update_bullets_by_speed(self.height);
        for enemy in self.enemys.iter_mut() {
            //enemy.update(self.height);
            enemy.update_bullets_by_speed(self.height);

            if self.frame_count%10==0 {
                enemy.attack_bullet(self.height);
            }
            //碰撞检测
            if enemy.deref().coll_detect(&self.player) {
                self.player.health -= 10;
            }
            for bullet in &enemy.bullets {
                if bullet.coll_detect(&self.player) {
                    self.player.health -= 10;
                }
            }
            for bullet in &self.player.bullets {
                if bullet.coll_detect(enemy.deref()) {
                    enemy.health -= 10;
                }
            }
            if enemy.is_dead() {
                self.score += 1;
            }
        }
        let max_enemy_count = self.difficulty * 10;
        if self.frame_count%10==0 && self.enemys.len() < max_enemy_count as usize {
            let random_enemy = Enemy::new_random_enemy(self.width, self.height);
            self.enemys.push(random_enemy);
        }
        Ok(())
    }
    /// 渲染分数
    fn render_player_score(&mut self) -> Result<(), CliError> {
        execute!(
            self.stdout,
            cursor::MoveTo(self.width +1,0),
            Print(format!("得分🥇：{}",self.score.to_string().blue()))
        )?;
        execute!(
            self.stdout,
            cursor::MoveTo(self.width +1,1),
            Print(format!("生命🩸：{}",self.player.health.to_string().blue()))
        )?;
        if self.player.health<0 {
            return Err(CliError::UnknownError("Game Over!!!".to_owned()));
        }
        Ok(())
    }
}

/// 游戏输入轮询
fn poll_input() -> Result<Option<KeyCode>, CliError> {
    // 非阻塞输入轮询
    if event::poll(Duration::from_millis(100))? {
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
        "按上下左右操作战机，空格键发射"
    }

    fn run(&self, width: u16, height: u16, difficulty: u8) -> Result<(), CliError> {
        println!("{}运行中", self.name());
        let mut game = Arc::new(Mutex::new(ThunderFighterGameState::new(width, height, difficulty)));
        // 进入全屏 EnterAlternateScreen
        let mut stdout = stdout();
        execute!(stdout,cursor::Hide)?;
        // terminal::enable_raw_mode()?;
        execute!(stdout,Clear(ClearType::All),cursor::MoveTo(0,0))?;
        let g1 = game.clone();
        {
            let mut game_state = g1.lock().unwrap();
            game_state.render_player_score()?;
        }
        let (tx, rx) = channel();
        thread::spawn(move || {
            execute!(std::io::stdout(),cursor::MoveTo(0,0),Print("update spawn....")).unwrap();
            loop {
                //捕获panic!
                let result = catch_unwind(|| {
                    execute!(std::io::stdout(),cursor::MoveTo(0,0),Print("update loop....")).unwrap();
                    let mut game_state = g1.lock().unwrap();
                    execute!(std::io::stdout(),cursor::MoveTo(0,0),Print("update....")).unwrap();
                    //game_state.update().unwrap()
                });
                if let Err(e) = result {
                    tx.send("game update error!".to_owned()).unwrap();
                } 
                //控制刷新帧率
                sleep(Duration::from_millis(100));
            }
        });
        let g2 = game.clone();
        loop {
            if let Ok(err) = rx.try_recv() {
                execute!(std::io::stdout(),cursor::MoveTo(0,0),Print("update try_recv....")).unwrap();
                return Err(CliError::UnknownError(err));
            }
            let mut game_state = g2.lock().unwrap();
            //接收输入
            if let Some(code) = poll_input()? {
                if game_state.handle_input(code)? {
                    break;
                }
            }
            game_state.update()?;
            // 绘制
            game_state.render()?;
            game_state.frame_count+= 1;
            drop(game_state);
            // 控制刷新帧率
            // sleep(Duration::from_millis(50));
        }
        execute!(stdout,LeaveAlternateScreen,cursor::Show)?;
        Ok(())
    }
}