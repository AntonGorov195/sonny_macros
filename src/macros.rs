use std::time::Duration;

use async_std::{self, task};
use inputbot::KeybdKey::*;
use inputbot::{KeybdKey, MouseButton, MouseCursor};

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
const BATTLE_CHARCTER_SELECT: [(f32, f32); 9] = [
    (0.277, 0.64),
    (-1., -1.),
    (0.72, 0.64),
    (0.2, 0.515),
    (-1., -1.),
    (0.8, 0.515),
    (0.277, 0.378),
    (-1., -1.),
    (0.72, 0.378),
];
const INVENTORY_EQUIPMENT_POS: [(f32, f32); 7] = [
    (0.1, 0.294),
    (0.1, 0.366),
    (0.1, 0.43),
    (0.15, 0.43),
    (0.33, 0.294),
    (0.33, 0.366),
    (0.33, 0.43),
];
const SHOP_EQUIPMENT_POS: [(f32, f32); 7] = [
    (0.375, 0.2),
    (0.375, 0.26),
    (0.375, 0.32),
    (0.4175, 0.32),
    (0.57, 0.2),
    (0.57, 0.26),
    (0.57, 0.32),
];
const SKIP_TURN_POS: (f32, f32) = (0.5, 0.875);
const MAP_TAB_POS: (f32, f32) = SKIP_TURN_POS;
const SENSITIVITY: (f32, f32) = (0.01, 0.01);
const INVENTORY_POS: (f32, f32) = (0.665, 0.2135);
const INVENTORY_TILE_OFFSET: (f32, f32) = (0.0472, 0.0648);
const INVENTORY_PROFILE_POS: (f32, f32) = (0.0925, 0.669);
const INVENTORY_PROFILE_OFFSET: (f32, f32) = (0.048, 0.);
const CLOSE_MENU_WINDOW: (f32, f32) = (0.939, 0.076);
const EXIT_GAME_X: (f32, f32) = (0.555, 0.8184874);
const VICTORY_SCREEN_ITEMS: (f32, f32) = (0.379, 0.2488);
const BATTLE_FIRST_BUFF_POS: [(f32, f32); 9] = [
    (0.31875, 0.1714),
    (-1., -1.),
    (0.68125, 0.1714),
    (0.31875, 0.1092),
    BATTLE_BUFF_CENTER_POS,
    (0.68125, 0.1092),
    (0.31875, 0.047),
    (-1., -1.),
    (0.68125, 0.047),
];
const BATTLE_BUFF_OFFSET: (f32, f32) = (0.0225, 0.0622);
const BATTLE_BUFF_CENTER_POS: (f32, f32) = (0.5, 0.1092);
const VICTORY_PROCEED: (f32, f32) = (0.5, 0.675);
const INVENTORY_DROP_BOX: (f32, f32) = (0.904, 0.666);
const ABILITY_TREE_POS: (f32, f32) = (0.1075, 0.21176);
const ABILITY_TREE_OFFSET: (f32, f32) = (0.065, 0.07223);
const ABILITY_WHEEL_CENTER: (f32, f32) = (0.7878788, 0.30253354);
const ABILITY_WHEEL_OFFSETS: [(f32, f32); 9] = [
    (-0.045, 0.065),
    (0., 0.091),
    (0.045, 0.065),
    (-0.0665, 0.),
    (0., 0.),
    (0.0665, 0.),
    (-0.045, -0.065),
    (0., -0.091),
    (0.045, -0.065),
];
const ABILITY_POINTS_POS: (f32, f32) = (0.415, 0.6);
const ABILITY_POINTS_OFFSET: (f32, f32) = (0., 0.03);
const SHOP_SELL_BOX: (f32, f32) = (0.855, 0.666);
const SHOP_BUY_POS: (f32, f32) = (0.38125, 0.5);

pub struct Macro {
    state: MacroState,
    window_bounds: WindowBound,
    saved_pos: (f32, f32),
}
#[derive(Debug, Clone, Copy, Default)]
pub struct WindowBound {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[derive(Debug, Clone, Copy)]
enum MacroState {
    Neutral,
    DebugState,
    Manual,
    BattleCharacterSelection,
}
use MacroState::*;
impl Macro {
    pub fn new() -> Self {
        Self {
            saved_pos: Default::default(),
            state: Neutral,
            window_bounds: Default::default(),
        }
    }
    pub fn key_pressed(&mut self, key: KeybdKey) {
        self.window_bounds = crate::window::get_window_size();
        match key {
            Numpad0Key | Numrow0Key => {
                self.set_state(MacroState::Neutral);
                return;
            }
            ZKey => {
                if MouseButton::LeftButton.is_pressed() {
                    MouseButton::LeftButton.release();
                    return;
                }
                MouseButton::LeftButton.press();
                if KeybdKey::CapsLockKey.is_toggled() {
                    return;
                }
                end_left_click();
                return;
            }
            RKey => {
                if KeybdKey::is_alt() {
                    let pos = self.window_bounds.to_screen_coords(self.saved_pos);
                    println!(
                        "Loaded: Screen coord:{:?}, Window coord:{:.2?}",
                        pos, self.saved_pos
                    );
                    self.window_bounds.move_window_coord(self.saved_pos);
                    return;
                }
                let pos = self.window_bounds.to_window_coords(MouseCursor::pos());
                println!(
                    "Saved: Screen coord:{:?}, Window coord:{:.2?}",
                    MouseCursor::pos(),
                    self.window_bounds.to_window_coords(MouseCursor::pos())
                );
                self.saved_pos = pos;
                return;
            }
            _ => (),
        }
        match self.state {
            Neutral => match key {
                RBracketKey => self.set_state(DebugState),
                UKey => self.set_state(Manual),
                _ => return,
            },
            DebugState => {
                if let Some(num) = key.to_num() {
                    let (x, y) = self.window_bounds.window_center();
                    let (width, heigh) = self.window_bounds.window_size();
                    let (width, heigh) = (width / 2, heigh / 2);
                    let (width, heigh) = (
                        width * SIDE[(num - 1) as usize].0,
                        heigh * SIDE[(num - 1) as usize].1,
                    );
                    let (x, y) = (x + width, y + heigh);
                    println!("Corner: {:?}", SIDE[(num - 1) as usize]);
                    MouseCursor::move_abs(x, y);
                    return;
                };
                let pos = self.window_bounds.to_window_coords(MouseCursor::pos());
                println!("{:?}", pos);
            }
            Manual => {
                let Some(arrow)=KeybdKey::get_arrow()else{
                    return;
                };
                let mut offset = (
                    SENSITIVITY.0 * arrow.0 as f32,
                    SENSITIVITY.1 * arrow.1 as f32,
                );
                if KeybdKey::is_alt() {
                    offset = (offset.0 * 3., offset.1 * 3.);
                }
                self.window_bounds.move_rel(offset);
            }
            BattleCharacterSelection => {
                
            }
        }
    }
    fn set_state(&mut self, state: MacroState) {
        println!("{:?}", state);
        self.state = state;
    }
}
impl WindowBound {
    pub fn move_window_coord(self, coord: (f32, f32)) {
        let (x, y) = self.to_screen_coords(coord);
        MouseCursor::move_abs(x, y);
    }
    pub fn to_screen_coords(self, coord: (f32, f32)) -> (i32, i32) {
        let (x, y) = (self.left, self.top);
        let (width, height) = self.window_size();
        (
            x + (width as f32 * coord.0) as i32,
            y + (height as f32 * coord.1) as i32,
        )
    }
    pub fn to_window_coords(self, coord: (i32, i32)) -> (f32, f32) {
        let (x, y) = (self.left, self.top);
        let (x, y) = ((coord.0 - x) as f32, (coord.1 - y) as f32);
        let size = self.window_size();
        (x / size.0 as f32, y / size.1 as f32)
    }
    pub fn window_size(self) -> (i32, i32) {
        (self.right - self.left, self.bottom - self.top)
    }
    pub fn window_center(self) -> (i32, i32) {
        ((self.right + self.left) / 2, (self.top + self.bottom) / 2)
    }
    pub fn center_mouse(self) {
        let (x, y) = self.to_screen_coords((0.5, 0.5));
        MouseCursor::move_abs(x, y);
    }
    pub fn move_rel(self, (x, y): (f32, f32)) {
        let (x, y) = self.to_screen_coords((x, y));
        let (x, y) = (x - self.left, y - self.top);
        MouseCursor::move_rel(x, y);
    }
}
impl MacroState {
    pub fn move_mouse(self, window: WindowBound) {
        match self {
            Neutral => window.center_mouse(),
            BattleCharacterSelection => window.center_mouse(),
            _ => (),
        }
    }
}
fn left_click() {
    MouseButton::LeftButton.press();
    end_left_click();
}
fn left_click_repeat(times: i32) {
    task::spawn(async move {
        for _ in 0..times {
            MouseButton::LeftButton.press();
            wait().await;
            MouseButton::LeftButton.release();
        }
    });
}
fn end_left_click() {
    _ = task::spawn(async move {
        wait().await;
        MouseButton::LeftButton.release();
    });
}
async fn wait() {
    task::sleep(Duration::from_millis(1)).await
}
pub trait KeybdKeyExt {
    fn is_shift() -> bool;
    fn is_ctrl() -> bool;
    fn is_alt() -> bool;
    fn get_arrow() -> Option<(i32, i32)>;
    fn to_num(self) -> Option<i32>;
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
    fn get_arrow() -> Option<(i32, i32)> {
        let mut is_arrow_key = false;
        let mut arrow = (0, 0);
        if KeybdKey::UpKey.is_pressed() {
            arrow.1 -= 1;
            is_arrow_key = true;
        }
        if KeybdKey::DownKey.is_pressed() {
            arrow.1 += 1;
            is_arrow_key = true;
        }
        if KeybdKey::LeftKey.is_pressed() {
            arrow.0 -= 1;
            is_arrow_key = true;
        }
        if KeybdKey::RightKey.is_pressed() {
            arrow.0 += 1;
            is_arrow_key = true;
        }
        if is_arrow_key {
            return Some(arrow);
        }
        return None;
    }
    fn to_num(self) -> Option<i32> {
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
