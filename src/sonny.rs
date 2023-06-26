use async_std::{self, task};
use inputbot::KeybdKey::*;
use inputbot::{KeybdKey, MouseButton, MouseCursor};
use std::{
    fmt::Debug,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

pub struct Macro {
    state: MacroState,
    win_size: WindowSize,
}
#[derive(Debug, Clone, Copy)]
pub enum MacroState {
    Neutral,
    Record,
    CornerMove,
    WinSizeSelect,
    WindowSizePicker,
    BattleAbilitySelect {
        character: i32,
    },
    BattleChar,
    Inventory {
        x: i32,
        y: i32,
        selected_col: Option<i32>,
    },
    Menu,
    Overworld,
    Manual,
}
use MacroState::*;
#[derive(Debug, Clone, Copy, Default)]
pub struct WindowSize {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
const SIDE: [(i32, i32); 9] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const INFINITY_OVERWORLD: [(f32, f32); 6] = [
    (0.553, 0.58), //b
    (0.75, 0.63),  //t
    (0.89, 0.63),  //sp
    (0.11, 0.6),   //shop
    (0.11, 0.416),
    (0.11, 0.215),
];
const SKIP_TURN_POS: (f32, f32) = (0.5, 0.875);

impl Macro {
    pub fn new() -> Self {
        let mut this = Self {
            win_size: Default::default(),
            state: MacroState::Neutral,
        };
        this.set_state(MacroState::Neutral);
        this
    }
    pub fn key_pressed(&mut self, key: KeybdKey) {
        match key {
            Numpad0Key | Numrow0Key => {
                self.set_state(MacroState::Neutral);
                self.center_mouse();
                return;
            }
            ZKey => {
                if MouseButton::LeftButton.is_pressed() {
                    MouseButton::LeftButton.release();
                    return;
                }
                MouseButton::LeftButton.press();
                if KeybdKey::is_alt() {
                    return;
                }
                end_left_click();
                return;
            }
            XKey => {
                let (x, y) = self.to_screen_coords((0.939, 0.076));
                MouseCursor::move_abs(x, y);
                left_click();
                return;
            }
            _ => (),
        }
        match self.state {
            Neutral => match key {
                RKey => self.set_state(Record),
                EKey => {
                    self.set_state(WindowSizePicker);

                    let pos = MouseCursor::pos();
                    self.win_size.left = pos.0;
                    self.win_size.top = pos.1;
                    println!("Top left corner: {:?}", pos);
                }
                CKey => self.set_state(CornerMove),
                BKey => self.set_state(BattleChar),
                IKey => self.set_state(Inventory {
                    x: 1,
                    y: 1,
                    selected_col: None,
                }),
                OKey => self.set_state(Overworld),
                MKey => {
                    let (x, y) = self.to_screen_coords(SKIP_TURN_POS);
                    MouseCursor::move_abs(x, y);
                    self.set_state(MacroState::Menu);
                }

                _ => (),
            },
            Record => {
                println!(
                    "Screen coord:{:?}, Window coord:{:?}",
                    MouseCursor::pos(),
                    self.to_window_coords(MouseCursor::pos())
                )
            }
            WinSizeSelect => {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf);
                match buf
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    [left, top, right, bottom, ..] => {
                        self.win_size = WindowSize {
                            left,
                            top,
                            right,
                            bottom,
                        }
                    }
                    _ => (),
                }
                self.set_state(MacroState::Neutral);
            }
            WindowSizePicker => {
                if key != EKey {
                    return;
                }
                let pos = MouseCursor::pos();
                self.win_size.right = pos.0;
                self.win_size.bottom = pos.1;
                println!("Bottom right corner: {:?}", pos);
                self.center_mouse();
                self.set_state(Neutral);
            }
            CornerMove => {
                let Some(num) = key.get_num() else {
                    return;
                };
                let (x, y) = self.window_center();
                let (width, heigh) = self.window_size();
                let (width, heigh) = (width / 2, heigh / 2);
                let (width, heigh) = (
                    width * SIDE[(num - 1) as usize].0,
                    heigh * SIDE[(num - 1) as usize].1,
                );
                let (x, y) = (x + width, y + heigh);
                println!("Corner: {:?}", SIDE[(num - 1) as usize]);
                MouseCursor::move_abs(x, y);
            }
            BattleAbilitySelect { .. } => {
                if key == KeybdKey::BKey {
                    self.set_state(BattleChar);
                }
                let (x, y) = match key.get_num() {
                    Some(1) => (-0.045, 0.065),
                    Some(2) => (0., 0.091),
                    Some(3) => (0.045, 0.065),
                    Some(4) => (-0.0665, 0.),
                    Some(6) => (0.0665, 0.),
                    Some(7) => (-0.045, -0.065),
                    Some(8) => (0., -0.091),
                    Some(9) => (0.045, -0.065),
                    _ => return,
                };
                let (x, y) = self.to_screen_coords((x, y));
                let (x, y) = (x - self.win_size.left, y - self.win_size.top);
                MouseCursor::move_rel(x, y);
                left_click();
                if !KeybdKey::is_alt() {
                    task::spawn(async move {
                        left_click();
                        task::sleep(Duration::from_millis(16)).await;
                        left_click();
                    });
                }
            }
            BattleChar => {
                let Some(get_num) = key.get_num() else{
                    return;
                };
                let (x, y) = match get_num {
                    1 => (0.277, 0.64),
                    3 => (0.72, 0.64),
                    4 => (0.2, 0.515),
                    5 => {
                        self.skip_turn();
                        return;
                    }
                    6 => (0.8, 0.515),
                    7 => (0.277, 0.378),
                    9 => (0.72, 0.378),
                    _ => return,
                };
                let state = BattleAbilitySelect { character: get_num };
                let (x, y) = self.to_screen_coords((x, y));
                MouseCursor::move_abs(x, y);
                self.set_state(state);
            }
            Inventory {
                mut x,
                mut y,
                selected_col,
            } => {
                let is_arrow_key: bool;
                match key {
                    RightKey => {
                        x += 1;
                        is_arrow_key = true;
                    }
                    LeftKey => {
                        x -= 1;
                        is_arrow_key = true;
                    }
                    UpKey => {
                        y -= 1;
                        is_arrow_key = true;
                    }
                    DownKey => {
                        y += 1;
                        is_arrow_key = true;
                    }
                    _ => is_arrow_key = false,
                }
                if is_arrow_key {
                    self.inventory_set_mouse(x, y);
                    return;
                }
                let Some(num) = key.get_num() else {
                    return;
                };
                match selected_col {
                    Some(selected_col) => {
                        self.set_state(Inventory {
                            x: selected_col,
                            y: num,
                            selected_col: None,
                        });
                        self.inventory_set_mouse(selected_col, num);
                    }
                    None => self.set_state(Inventory {
                        x,
                        y,
                        selected_col: Some(num),
                    }),
                }
                //                 let Some(num) = key.get_num() else {
                //                     self.set_state(MacroState::Inventory(None));
                //                     return;
                //                 };
                //                 let (x, y) = (
                //                     0.0472 * (row - 1) as f32 + 0.665,
                //                     0.0648 * (num - 1) as f32 + 0.2135,
                //                 );
                //                 let (x, y) = self.to_screen_coords((x, y));
                //                 MouseCursor::move_abs(x, y);
                //
                //                 println!("{}", num);
                //                 std::io::stdout().flush();
                //                 self.set_state(MacroState::Inventory(None));
            }
            Menu => {
                let Some(tab) = key.get_num()else{
                    return;
                };
                match tab {
                    6 => {
                        let (x, y) = self.to_screen_coords(SKIP_TURN_POS);
                        MouseCursor::move_abs(x, y);
                    }
                    1..=5 => {
                        let offset = 0.072 * (tab - 1) as f32;
                        let (x, y) = self.to_screen_coords((offset + 0.068, 0.85));
                        MouseCursor::move_abs(x, y);
                    }
                    _ => return,
                }
            }
            Overworld => {
                let Some(num)=key.get_num() else {
                    return;
                };
                if num > 6 {
                    return;
                }
                let (x, y) = self.to_screen_coords(INFINITY_OVERWORLD[(num - 1) as usize]);
                MouseCursor::move_abs(x, y);
                if KeybdKey::is_alt() {
                    return;
                }
                left_click();
            }
            Manual => {}
        }
    }
    pub fn set_state(&mut self, mode: MacroState) {
        self.state = mode;
        println!("{:?} Mode", mode);
    }
    pub fn window_size(&self) -> (i32, i32) {
        (
            self.win_size.right - self.win_size.left,
            self.win_size.bottom - self.win_size.top,
        )
    }
    pub fn window_center(&self) -> (i32, i32) {
        (
            (self.win_size.right + self.win_size.left) / 2,
            (self.win_size.top + self.win_size.bottom) / 2,
        )
    }
    pub fn to_screen_coords(&self, coord: (f32, f32)) -> (i32, i32) {
        let (x, y) = (self.win_size.left, self.win_size.top);
        let (width, height) = self.window_size();
        (
            x + (width as f32 * coord.0) as i32,
            y + (height as f32 * coord.1) as i32,
        )
    }
    pub fn to_window_coords(&self, coord: (i32, i32)) -> (f32, f32) {
        let (x, y) = (self.win_size.left, self.win_size.top);
        let (x, y) = ((coord.0 - x) as f32, (coord.1 - y) as f32);
        let size = self.window_size();
        (x / size.0 as f32, y / size.1 as f32)
    }
    fn center_mouse(&self) {
        let (x, y) = self.to_screen_coords((0.5, 0.5));
        MouseCursor::move_abs(x, y);
    }
    fn skip_turn(&mut self) {
        self.set_state(MacroState::Neutral);
        let (x, y) = self.to_screen_coords(SKIP_TURN_POS);
        MouseCursor::move_abs(x, y);
        left_click()
    }
    fn inventory_set_mouse(&mut self, x: i32, y: i32) {
        match (x, y) {
            (1..=6, 1..=6) => {
                let (x, y) = (
                    0.0472 * (x - 1) as f32 + 0.665,
                    0.0648 * (y - 1) as f32 + 0.2135,
                );
                let (x, y) = self.to_screen_coords((x, y));
                MouseCursor::move_abs(x, y);
            }
            _ => (),
        }
    }
}
fn left_click() {
    MouseButton::LeftButton.press();
    end_left_click();
}
fn end_left_click() {
    task::spawn(async move {
        task::sleep(Duration::from_millis(16)).await;
        MouseButton::LeftButton.release();
    });
}

pub trait KeybdKeyExt {
    fn is_shift() -> bool;
    fn is_ctrl() -> bool;
    fn is_alt() -> bool;
    fn get_num(self) -> Option<i32>;
}
impl KeybdKeyExt for inputbot::KeybdKey {
    fn is_shift() -> bool {
        RShiftKey.is_pressed() || LShiftKey.is_pressed()
    }
    fn is_ctrl() -> bool {
        RControlKey.is_pressed() || LControlKey.is_pressed()
    }
    fn is_alt() -> bool {
        KeybdKey::RAltKey.is_pressed() || KeybdKey::LAltKey.is_pressed()
    }
    fn get_num(self) -> Option<i32> {
        match self {
            Numpad0Key | Numrow0Key => Some(0),
            Numpad1Key | Numrow1Key => Some(1),
            Numpad2Key | Numrow2Key => Some(2),
            Numpad3Key | Numrow3Key => Some(3),
            Numpad4Key | Numrow4Key => Some(4),
            Numpad5Key | Numrow5Key => Some(5),
            Numpad6Key | Numrow6Key => Some(6),
            Numpad7Key | Numrow7Key => Some(7),
            Numpad8Key | Numrow8Key => Some(8),
            Numpad9Key | Numrow9Key => Some(9),
            _ => None,
        }
    }
}
