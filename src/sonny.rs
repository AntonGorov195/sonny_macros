use async_std::{self, task};
use inputbot::KeybdKey::*;
use inputbot::{KeybdKey, MouseButton, MouseCursor};
use std::array;
use std::{fmt::Debug, time::Duration};

pub struct Macro {
    mode: MacroMode,
    state: MacroState,
    window_bounds: WindowBound,
    saved_pos: (f32, f32),
}
#[derive(Debug, Clone, Copy)]
pub enum MacroMode {
    Neutral,
    DebugState,
    Manual,
    LanguagePicker(i32),
    StartMenu(i32),
    LoadType(i32),
    LoadCharacterSelecter(i32),
    NewCharacterSelect(i32),
    EnterName(i32),
    Map(i32),
    WhiteNovember(i32),
    GhostBeach(i32),
    Infinity(i32),
    // WinSizeSelect,
    // WindowSizePicker,
    BattleAbilitySelect { character: i32, ability: i32 },
    BattleCharacterSelection,
    BuffView,
    Inventory { x: i32, y: i32 },
    InventoryEquip { x: i32, y: i32 },
    InventoryProfile(i32),
    InventoryDrop(i32),
    BottomTabs(i32),
    VictoryLoot { x: i32, y: i32 },
    VictoryProceed(i32),
    VictoryInventory { x: i32, y: i32 },
    VictoryDrop(i32),
    ShopInventory { x: i32, y: i32 },
    ShopEquip { x: i32, y: i32 },
    ShopBuy { x: i32, y: i32 },
    ShopDropSell { x: i32 },
    AbilityTree { x: i32, y: i32 },
    AbilityWheel(i32),
    AbilityPointTable(i32),
    AbilitySelect { x: i32 },
    AbilitySelectSlider(i32),
}
use MacroMode::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MacroState {
    Normal,
    Paused,
    Console,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct WindowBound {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
use KeybdKey::BKey as Back;
use KeybdKey::NKey as Next;
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
const LANGUAGE_POS: [(f32, f32); 2] = [(0.5, 0.422), (0.5, 0.576)];
const START_MENU: [(f32, f32); 3] = [
    (0.48625, 0.6184874),
    (0.48375, 0.7277311),
    (0.485, 0.77983195),
];
const BACK_BUTTON: (f32, f32) = (0.95, 0.95);
const GAME_LOAD_TYPE: [(f32, f32); 3] = [(0.5, 0.6), (0.5, 0.7), BACK_BUTTON];
const SAVE_SELECT: [(f32, f32); 5] = [
    (0.35, 0.366),
    (0.35, 0.46),
    (0.35, 0.56),
    (0.35, 0.65),
    BACK_BUTTON,
];
const ENTER_NAME_HERE: [(f32, f32); 3] = [(0.5, 0.475), (0.5, 0.617), BACK_BUTTON];
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
const TABS_POS: (f32, f32) = (0.068, 0.85);
const TABS_OFFSET_X: f32 = 0.072;
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
const SENSITIVITY: (f32, f32) = (0.02, 0.02);
const INVENTORY_POS: (f32, f32) = (0.665, 0.2135);
const INVENTORY_TILE_OFFSET: (f32, f32) = (0.0472, 0.0648);
const INVENTORY_PROFILE_POS: (f32, f32) = (0.0925, 0.669);
const INVENTORY_PROFILE_OFFSET_X: f32 = 0.048;
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
const ABILITY_SELECT_POS: (f32, f32) = (0.696, 0.5277);
const ABILITY_SELECT_OFFSET: (f32, f32) = (0.09875, 0.062);
const ABILITY_SELECT_SLIDER_POS: (f32, f32) = (0.905, 0.5411765);
const ABILITY_SELECT_SLIDER_OFFSET: (f32, f32) = (0., 0.0975);
const SHOP_SELL_BOX: (f32, f32) = (0.855, 0.666);
const SHOP_BUY_POS: (f32, f32) = (0.38125, 0.5);
const MAP: [(f32, f32); 4] = [
    (0.16875, 0.74118),
    (0.47, 0.7311),
    (0.5975, 0.281),
    (0.8938, 0.739),
];
impl Macro {
    pub fn new() -> Self {
        println!("Neutral State");
        Self {
            window_bounds: Default::default(),
            mode: MacroMode::Neutral,
            saved_pos: (0.5, 0.5),
            state: MacroState::Normal,
        }
    }
    pub fn key_pressed(&mut self, key: KeybdKey) {
        macro_rules! select_menu {
            ($name:ident,$num:ident,$key:ident,$range:expr,$next:expr,$back:expr) => {
                if $key == Next {
                    $next
                    return;
                }
                if key == Back {
                    $back
                    return;
                }
                if let Some($num) = $key.to_num() {
                    if (1..=2).contains(&$num) == false {
                        return;
                    }
                    self.set_mode($name($num));
                    return;
                }
                let Some(arrow)=KeybdKey::get_arrow() else {
                                return;
                            };
                let $num = $num + arrow.1;
                self.set_mode($name($num.min(*$range.end()).max(*$range.start())));
            };
        }

        self.window_bounds = crate::window::get_window_size();

        match key {
            F2Key => {
                self.state = MacroState::Paused;
            }
            F1Key => {
                self.state = MacroState::Normal;
            }
            _ if self.state == MacroState::Paused => {
                return;
            }
            Numrow0Key | Numpad0Key => {
                self.set_mode(MacroMode::Neutral);
                return;
            }
            ZKey => {
                if MouseButton::LeftButton.is_pressed() {
                    MouseButton::LeftButton.release();
                    return;
                }
                MouseButton::LeftButton.press();
                if KeybdKey::XKey.is_pressed() {
                    return;
                }
                end_left_click();
                return;
            }
            QKey => {
                if KeybdKey::is_alt() {
                    let pos = self.to_screen_coords(self.saved_pos);
                    println!(
                        "Loaded: Screen coord:{:?}, Window coord:{:.2?}",
                        pos, self.saved_pos
                    );
                    self.window_bounds.move_window_coord(self.saved_pos);
                    return;
                }
                let pos = self.to_window_coords(MouseCursor::pos());
                println!(
                    "Saved: Screen coord:{:?}, Window coord:{:.2?}",
                    MouseCursor::pos(),
                    self.to_window_coords(MouseCursor::pos())
                );
                self.saved_pos = pos;
                return;
            }
            _ => (),
        }
        match self.mode {
            Neutral => match key {
                RBracketKey => self.set_mode(DebugState),
                TKey => {
                    self.set_mode(BottomTabs(6));
                }
                UKey => {
                    self.set_mode(Manual);
                }
                LKey => self.set_mode(LanguagePicker(1)),
                _ => (),
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
                let pos = self.to_window_coords(MouseCursor::pos());
                println!("{:?}", self.to_window_coords(MouseCursor::pos()));
                self.saved_pos = pos;
            }
            Manual => {
                let Some((x, y)) = KeybdKey::get_arrow() else {
                    return;
                };
                let (mut x, mut y) = (x as f32 * SENSITIVITY.0, y as f32 * SENSITIVITY.1);
                if KeybdKey::is_alt() {
                    x *= 4.;
                    y *= 4.;
                }
                self.window_bounds.rel_move((x, y));
            }
            LanguagePicker(num) => {
                select_menu!(
                    LanguagePicker,
                    num,
                    key,
                    (1..=2),
                    {
                        self.set_mode(StartMenu(1));
                    },
                    {}
                );
                // if key == Next {
                //     self.set_mode(StartMenu(1));
                //     return;
                // }
                // if let Some(num) = key.to_num() {
                //     if (1..=2).contains(&num) == false {
                //         return;
                //     }
                //     self.set_mode(LanguagePicker(num));
                //     return;
                // }
                // let Some(arrow)=KeybdKey::get_arrow() else {
                //     return;
                // };
                // let num = num + arrow.1;
                // self.set_mode(LanguagePicker(num.min(2).max(1)));
            }
            StartMenu(num) => {
                if key == Next {
                    match num {
                        1 => {
                            self.set_mode(LoadType(1));
                        }
                        _ => todo!(),
                    }
                    return;
                }
                match key.to_num() {
                    Some(num) if (1..=3).contains(&num) => {
                        self.set_mode(StartMenu(num));
                        return;
                    }
                    _ => (),
                }
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let num = num + arrow.1;
                self.set_mode(StartMenu(num.min(3).max(1)));
            }
            LoadType(num) => {
                if key == Next {
                    match num {
                        1 => {
                            self.set_mode(NewCharacterSelect(1));
                        }
                        2 => {
                            self.set_mode(LoadCharacterSelecter(1));
                        }
                        3 => {
                            self.set_mode(StartMenu(1));
                        }
                        _ => todo!(),
                    }
                    return;
                }
                if key == Back {
                    self.set_mode(StartMenu(1));
                    return;
                }
                match key.to_num() {
                    Some(num) if (1..=3).contains(&num) => {
                        self.set_mode(LoadType(num));
                        return;
                    }
                    _ => (),
                }
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let num = num + arrow.1;
                self.set_mode(LoadType(num.min(3).max(1)));
            }
            LoadCharacterSelecter(num) => {
                if key == Next {
                    match num {
                        1..=4 => {
                            todo!()
                        }
                        5 => {
                            self.set_mode(StartMenu(1));
                        }
                        _ => panic!(
                            "Unexpected state: LoadCharacterSelecter must be in range 1 to 5"
                        ),
                    }
                    return;
                }
                if key == Back {
                    self.set_mode(StartMenu(1));
                    return;
                }
                match key.to_num() {
                    Some(num) if (1..=5).contains(&num) => {
                        self.set_mode(LoadCharacterSelecter(num));
                        return;
                    }
                    _ => (),
                }
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let num = num + arrow.1;
                self.set_mode(LoadCharacterSelecter(num.min(5).max(1)));
            }
            NewCharacterSelect(num) => {
                if key == Next {
                    match num {
                        1..=4 => {
                            todo!()
                        }
                        5 => {
                            self.set_mode(StartMenu(1));
                        }
                        _ => panic!("Unexpected state: NewCharacterSelect must be in range 1 to 5"),
                    }
                    return;
                }
                if key == Back {
                    self.set_mode(StartMenu(1));
                    return;
                }
                match key.to_num() {
                    Some(num) if (1..=5).contains(&num) => {
                        self.set_mode(NewCharacterSelect(num));
                        return;
                    }
                    _ => (),
                }
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let num = num + arrow.1;
                self.set_mode(NewCharacterSelect(num.min(5).max(1)));
            }
            EnterName(_) => todo!(),
            BattleAbilitySelect { character, .. } => {
                match key {
                    BKey => {
                        self.set_mode(BattleCharacterSelection);
                        return;
                    }
                    VKey => {
                        self.set_mode(VictoryLoot { x: 1, y: 1 });
                        return;
                    }
                    _ => (),
                }
                // ability select.
                let Some(num)=key.to_num() else {
                    return;
                };
                self.set_mode(BattleAbilitySelect {
                    character,
                    ability: num,
                });
            }
            BattleCharacterSelection => {
                if key == VKey {
                    self.set_mode(VictoryLoot { x: 1, y: 1 });
                    // self.move_window_coord(VICTORY_SCREEN_ITEMS);
                    return;
                }
                let Some(num) = key.to_num() else{
                    return;
                };
                match num {
                    5 => {
                        self.skip_turn();
                        return;
                    }
                    8 => {
                        self.set_mode(BuffView);
                    }
                    1..=9 => {
                        self.set_mode(BattleAbilitySelect {
                            character: num,
                            ability: 5,
                        });
                    }
                    _ => return,
                }
            }
            Inventory { mut x, mut y } => {
                if key == DKey {
                    self.set_mode(InventoryDrop(x));
                }
                if key == VKey {
                    self.set_mode(VictoryLoot { x: 1, y: 1 });
                    return;
                }
                if key == BackspaceKey {
                    self.window_bounds.move_window_coord(CLOSE_MENU_WINDOW);
                    left_click();
                    return;
                }
                if key == PKey {
                    self.set_mode(InventoryProfile(1));
                    return;
                }
                if let Some(arrow) = KeybdKey::get_arrow() {
                    x += arrow.0;
                    y += arrow.1;

                    match (x, y) {
                        (_, ..=0) => return,
                        (1..=6, 1..=6) => {
                            self.set_mode(Inventory { x, y });
                        }
                        (_, 7) => {
                            self.set_mode(InventoryDrop(x));
                            return;
                        }
                        (..=0, _) => {
                            let y = y.min(3).max(1);
                            self.set_mode(InventoryEquip { x: 3, y: y });
                            return;
                        }
                        (7.., _) => {
                            let y = y.min(3).max(1);
                            self.set_mode(InventoryEquip { x: 1, y: y });
                            return;
                        }
                        _ => return,
                    }
                }
            }
            InventoryEquip { x, y } => {
                let Some(arrow) = KeybdKey::get_arrow() else{
                                return;
                            };
                let (x, y) = (arrow.0 + x, (arrow.1 + y).min(3).max(1));

                match x {
                    1 => {
                        self.set_mode(InventoryEquip { x, y });
                    }
                    2 => {
                        self.set_mode(InventoryEquip { x, y });
                    }
                    3 => {
                        self.set_mode(InventoryEquip { x, y });
                    }
                    ..=0 => {
                        self.set_mode(Inventory { x: 6, y: y });
                    }
                    4.. => {
                        self.set_mode(Inventory { x: 1, y: y });
                    }
                }
            }
            InventoryProfile(mut x) => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                if arrow.1 < 0 {
                    let (x, y) = (x, 6);
                    self.set_mode(Inventory { x, y });
                    return;
                }
                x += arrow.0;
                if x == 7 || x == 0 {
                    self.set_mode(InventoryDrop(6));
                    return;
                }
                self.set_mode(InventoryProfile(x));
            }
            InventoryDrop(x) => {
                let Some(arrow)=KeybdKey::get_arrow()else {
                    return;
                };
                match arrow {
                    (_, -1) => {
                        self.set_mode(Inventory { x: x, y: 6 });
                    }
                    (1, _) => {
                        self.set_mode(InventoryProfile(1));
                    }
                    (-1, _) => {
                        self.set_mode(InventoryProfile(6));
                    }
                    _ => (),
                }
                return;
            }
            BottomTabs(x) => {
                if key == Next {
                    match x {
                        1 => {
                            self.set_mode(Inventory { x: 1, y: 1 });
                        }
                        2 => {
                            self.set_mode(AbilityTree { x: 1, y: 1 });
                        }
                        6 => {
                            self.set_mode(Map(1));
                        }
                        7 => {
                            todo!()
                        }
                        _ => return,
                    }
                }
                if let Some(tab) = key.to_num() {
                    self.set_mode(BottomTabs(tab));
                    return;
                };
                let Some(arrow) = KeybdKey::get_arrow() else{
                    return;
                };
                let x = arrow.0 + x;
                if (1..=7).contains(&x) {
                    self.set_mode(BottomTabs(x));
                }
            }
            Infinity(_) => {
                let Some(num) = key.to_num() else {
                    return;
                };
                if num > 6 {
                    return;
                }
                self.set_mode(Infinity(num));
            }
            BuffView => {
                if self.arrow_move(BATTLE_BUFF_OFFSET) {
                    return;
                }
                let Some(num)=key.to_num() else{
                    return;
                };
                match num {
                    2 => {
                        self.set_mode(BattleCharacterSelection);
                    }
                    8 => return,
                    1..=9 => {
                        self.window_bounds
                            .move_window_coord(BATTLE_FIRST_BUFF_POS[num as usize - 1]);
                    }
                    _ => return,
                }
            }
            VictoryLoot { x, y } => {
                match key {
                    VKey => {
                        self.set_mode(VictoryLoot { x: 1, y: 1 });
                        return;
                    }
                    IKey => {
                        self.set_mode(Inventory { x: 1, y: 1 });
                        return;
                    }
                    XKey => {
                        self.set_mode(VictoryProceed(x));
                    }
                    BKey => {
                        self.set_mode(BattleCharacterSelection);
                        return;
                    }
                    _ => (),
                }

                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let (x, y) = (arrow.0 + x, arrow.1 + y);
                if x < 1 || y < 1 {
                    return;
                }
                match (x, y) {
                    (1..=5, _) | (_, 1..=3) => self.set_mode(VictoryLoot { x: x, y: y }),
                    (6.., _) => self.set_mode(VictoryInventory { x: 1, y: y }),
                    (_, 4..) => self.set_mode(VictoryProceed(x)),
                    _ => return,
                }
            }
            VictoryProceed(x) => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                if arrow.1 < 0 {
                    self.set_mode(VictoryLoot { x: x, y: 3 });
                    return;
                }
            }
            VictoryInventory { x, y } => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let (x, y) = (arrow.0 + x, arrow.1 + y);
                match (x, y) {
                    (..=0, _) => {
                        self.set_mode(VictoryLoot { x: 5, y: y.min(3) });
                    }
                    (_, 7..) => {
                        self.set_mode(VictoryDrop(x));
                    }
                    _ => (),
                }
            }
            VictoryDrop(x) => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                match arrow {
                    (_, -1) => {
                        self.set_mode(VictoryInventory { x, y: 6 });
                    }
                    (-1, _) => {
                        self.set_mode(VictoryProceed(5));
                    }
                    _ => return,
                }
            }
            ShopInventory { mut x, mut y } => {
                if key == XKey {
                    self.window_bounds.move_window_coord(CLOSE_MENU_WINDOW);
                    left_click();
                    return;
                }
                if let Some(arrow) = KeybdKey::get_arrow() {
                    x += arrow.0;
                    y += arrow.1;

                    match (x, y) {
                        (_, ..=0) => return,
                        (1..=6, 1..=6) => {
                            self.set_mode(ShopInventory { x, y });
                        }
                        (_, 7..) => {
                            self.set_mode(ShopDropSell { x });
                            return;
                        }
                        (..=0, 1..=3) => {
                            self.set_mode(ShopEquip { x: 3, y: y });
                            return;
                        }
                        (..=0, 4..=6) => {
                            self.set_mode(ShopBuy { x: 5, y: y - 3 });
                            return;
                        }
                        (7.., 1..=3) => {
                            self.set_mode(ShopEquip { x: 1, y: y });
                            return;
                        }
                        (7.., 4..=6) => {
                            self.set_mode(ShopBuy { x: 1, y: y - 3 });
                            return;
                        }
                    }
                }
            }
            ShopEquip { x, y } => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let (x, y) = (arrow.0 + x, (arrow.1 + y).min(3).max(1));

                match x {
                    1 => {
                        self.set_mode(ShopEquip { x, y });
                    }
                    2 => {
                        self.set_mode(ShopEquip { x, y });
                    }
                    3 => {
                        self.set_mode(ShopEquip { x, y });
                    }
                    ..=0 => {
                        self.set_mode(ShopInventory { x: 6, y: y });
                    }
                    4.. => {
                        self.set_mode(ShopInventory { x: 1, y: y });
                    }
                }
            }
            ShopBuy { x, y } => {
                let Some(arrow)=KeybdKey::get_arrow() else{
                    return;
                };
                let (x, y) = (x + arrow.0, y + arrow.1);
                match (x, y) {
                    (_, 4..) | (_, ..=0) => return,
                    (x, _) if x < 1 => {
                        self.set_mode(ShopInventory { x: 6, y: y + 3 });
                    }
                    (x, _) if x > 5 => {
                        self.set_mode(ShopInventory { x: 1, y: y + 3 });
                    }
                    _ => {
                        self.set_mode(ShopBuy { x, y });
                    }
                }
            }
            ShopDropSell { x } => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                match arrow {
                    (1.., _) => {
                        self.set_mode(ShopDropSell { x: 6 });
                    }
                    (_, ..=-1) => {
                        self.set_mode(ShopInventory { x: x, y: 6 });
                    }
                    (..=-1, _) => {
                        self.set_mode(ShopDropSell { x: 5 });
                    }
                    _ => (),
                }
            }
            AbilityTree { x, y } => {
                if key == WKey {
                    self.set_mode(AbilityWheel(5));
                    return;
                }
                if key == SKey {
                    self.window_bounds.move_window_coord(ABILITY_SELECT_POS);
                    self.set_mode(AbilitySelect { x: 1 });
                    return;
                }
                if key == YKey {
                    self.set_mode(AbilityPointTable(1));
                    return;
                }
                let Some(arrow)=KeybdKey::get_arrow() else{
                    return;
                };
                let (x, y) = (x + arrow.0, y + arrow.1);
                match (x, y) {
                    (..=0, _) | (5.., _) | (_, ..=0) | (_, 8..) => {
                        return;
                    }
                    _ => (),
                }
                self.set_mode(AbilityTree { x: x, y: y });
            }
            AbilityWheel(_) => {
                if key == AKey {
                    self.set_mode(AbilityTree { x: 1, y: 1 });
                    return;
                }
                if key == SKey {
                    self.window_bounds.move_window_coord(ABILITY_SELECT_POS);
                    self.set_mode(AbilitySelect { x: 1 });
                    return;
                }
                if key == YKey {
                    self.set_mode(AbilityPointTable(1));
                    return;
                }

                self.window_bounds.move_window_coord(ABILITY_WHEEL_CENTER);
                // ability select.
                let Some(num)=key.to_num() else {
                    return;
                };
                self.set_mode(AbilityWheel(num));
            }
            AbilityPointTable(num) => {
                if key == SKey {
                    self.window_bounds.move_window_coord(ABILITY_SELECT_POS);
                    self.set_mode(AbilitySelect { x: 1 });
                    return;
                }
                if key == AKey {
                    self.set_mode(AbilityTree { x: 1, y: 1 });
                    return;
                }
                if key == WKey {
                    self.set_mode(AbilityWheel(5));
                    return;
                }

                match (key.to_num(), KeybdKey::get_arrow()) {
                    (_, Some((_, y))) if (1..5).contains(&(y + num)) => {
                        self.set_mode(AbilityPointTable(y + num));
                    }
                    (Some(num), _) if num < 5 => self.set_mode(AbilityPointTable(num)),
                    _ => return,
                }
            }
            AbilitySelect { x } => {
                if key == AKey {
                    self.set_mode(AbilityTree { x: 1, y: 1 });
                    return;
                }
                if key == WKey {
                    self.set_mode(AbilityWheel(5));
                    return;
                }
                if key == YKey {
                    self.set_mode(AbilityPointTable(1));
                    return;
                }

                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let mut y = arrow.1 as f32 * ABILITY_SELECT_OFFSET.1;
                if KeybdKey::is_alt() {
                    y /= 3.;
                }
                self.window_bounds.rel_move((0., y));
                let x = x + arrow.0;
                match x {
                    3.. => {
                        self.set_mode(AbilitySelectSlider(1));
                    }
                    1 | 2 => {
                        self.set_mode(AbilitySelect { x: x });
                    }
                    _ => return,
                }
            }
            AbilitySelectSlider(y) => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                let y = y + arrow.1;
                match y {
                    1 => self.set_mode(AbilitySelectSlider(1)),
                    2 => self.set_mode(AbilitySelectSlider(2)),
                    _ => return,
                }
            }
            WhiteNovember(_) => {
                let Some(num)=key.to_num() else {
                    return;
                };
                if (1..=2).contains(&num) {
                    self.set_mode(WhiteNovember(num));
                }
            }
            GhostBeach(_) => {
                let Some(num)=key.to_num() else {
                    return;
                };
                if (1..=2).contains(&num) {
                    self.set_mode(WhiteNovember(num));
                }
            }
            Map(_) => match key.to_num() {
                Some(num) if (1..=4).contains(&num) => self.set_mode(Map(num)),
                _ => return,
            },
        }
    }
    // fn move_to_shop_sell(&mut self, x: i32) {
    //     if x == 6 {
    //         self.move_window_coord(INVENTORY_DROP_BOX);
    //         return;
    //     }
    //     self.move_window_coord(SHOP_SELL_BOX);
    // }
    fn arrow_move(&mut self, offset: (f32, f32)) -> bool {
        if let Some(arrow) = KeybdKey::get_arrow() {
            let (x, y) = (arrow.0 as f32 * offset.0, arrow.1 as f32 * offset.1);
            self.window_bounds.rel_move((x, y));
            return true;
        }
        false
    }
    // fn move_window_coord(&mut self, coord: (f32, f32)) {
    //     let (x, y) = self.to_screen_coords(coord);
    //     MouseCursor::move_abs(x, y);
    // }
    fn set_mode(&mut self, state: MacroMode) {
        state.move_mouse(self.window_bounds);
        self.mode = state;
        println!("{:?} State", state);
    }
    fn skip_turn(&mut self) {
        self.window_bounds.move_window_coord(SKIP_TURN_POS);
        left_click()
    }
    fn to_screen_coords(&self, coord: (f32, f32)) -> (i32, i32) {
        self.window_bounds.to_screen_coords(coord)
    }
    fn to_window_coords(&self, coord: (i32, i32)) -> (f32, f32) {
        self.window_bounds.to_window_coords(coord)
    }
    // fn center_mouse(&self) {
    //     self.win_bounds.center_mouse()
    // }
}
impl WindowBound {
    pub fn rel_move(&self, (x, y): (f32, f32)) {
        let (x, y) = self.to_screen_coords((x, y));
        let (x, y) = (x - self.left, y - self.top);
        MouseCursor::move_rel(x, y);
    }
    pub fn move_window_coord(&self, coord: (f32, f32)) {
        let (x, y) = self.to_screen_coords(coord);
        MouseCursor::move_abs(x, y);
    }
    pub fn to_screen_coords(&self, coord: (f32, f32)) -> (i32, i32) {
        let (x, y) = (self.left, self.top);
        let (width, height) = self.window_size();
        (
            x + (width as f32 * coord.0) as i32,
            y + (height as f32 * coord.1) as i32,
        )
    }
    pub fn to_window_coords(&self, coord: (i32, i32)) -> (f32, f32) {
        let (x, y) = (self.left, self.top);
        let (x, y) = ((coord.0 - x) as f32, (coord.1 - y) as f32);
        let size = self.window_size();
        (x / size.0 as f32, y / size.1 as f32)
    }
    pub fn window_size(&self) -> (i32, i32) {
        (self.right - self.left, self.bottom - self.top)
    }
    pub fn window_center(&self) -> (i32, i32) {
        ((self.right + self.left) / 2, (self.top + self.bottom) / 2)
    }
    pub fn center_mouse(&self) {
        let (x, y) = self.to_screen_coords((0.5, 0.5));
        MouseCursor::move_abs(x, y);
    }
}
impl MacroMode {
    pub fn move_mouse(&self, window: WindowBound) {
        match *self {
            Neutral => window.center_mouse(),
            LanguagePicker(num) => {
                select_move(window, num, LANGUAGE_POS);
                // window.move_window_coord(LANGUAGE_POS[(num - 1) as usize]);
            }
            StartMenu(num) => {
                window.move_window_coord(START_MENU[(num - 1) as usize]);
            }
            LoadType(num) => {
                window.move_window_coord(GAME_LOAD_TYPE[(num - 1) as usize]);
            }
            LoadCharacterSelecter(num) => {
                select_move(window, num, SAVE_SELECT);
            }
            NewCharacterSelect(num) => {
                select_move(window, num, SAVE_SELECT);
            }
            EnterName(num) => select_move(window, num, ENTER_NAME_HERE),
            Infinity(num) => {
                window.move_window_coord(INFINITY_OVERWORLD[(num - 1) as usize]);
            }
            BuffView => {
                window.move_window_coord(BATTLE_BUFF_CENTER_POS);
            }
            BattleAbilitySelect { character, ability } => {
                window.move_window_coord(BATTLE_CHARCTER_SELECT[character as usize - 1]);
                window.rel_move(ABILITY_WHEEL_OFFSETS[ability as usize - 1]);
            }
            VictoryLoot { x, y } => {
                window.move_window_coord(VICTORY_SCREEN_ITEMS);
                window.rel_move(offset((x, y), INVENTORY_TILE_OFFSET));
            }
            VictoryProceed(_) => {
                window.move_window_coord(VICTORY_PROCEED);
            }
            VictoryInventory { x, y } => {
                window.move_window_coord(INVENTORY_POS);
                window.rel_move(offset((x, y), INVENTORY_TILE_OFFSET));
            }
            VictoryDrop(_) => {
                window.move_window_coord(INVENTORY_DROP_BOX);
            }
            BattleCharacterSelection => window.center_mouse(),
            BottomTabs(x) => match x {
                7 => {
                    window.move_window_coord(EXIT_GAME_X);
                }
                6 => {
                    window.move_window_coord(SKIP_TURN_POS);
                }
                1..=5 => {
                    let offset = TABS_OFFSET_X * (x - 1) as f32;
                    window.move_window_coord((TABS_POS.0 + offset, TABS_POS.1))
                }
                _ => return,
            },
            Inventory { x, y } => {
                window.move_window_coord(INVENTORY_POS);
                window.rel_move(offset((x, y), INVENTORY_TILE_OFFSET));
            }
            InventoryEquip { x, y } => match x {
                1 => {
                    window.move_window_coord(INVENTORY_EQUIPMENT_POS[(y - 1) as usize]);
                }
                2 => {
                    window.move_window_coord(INVENTORY_EQUIPMENT_POS[3]);
                }
                3 => {
                    window.move_window_coord(INVENTORY_EQUIPMENT_POS[(y + 3) as usize]);
                }
                ..=0 => {}
                _ => return,
            },
            InventoryProfile(x) => {
                window.move_window_coord(INVENTORY_PROFILE_POS);
                window.rel_move(offset((x, 0), (INVENTORY_PROFILE_OFFSET_X, 0.)));
            }
            InventoryDrop(_) => {
                window.move_window_coord(INVENTORY_DROP_BOX);
            }
            ShopInventory { x, y } => {
                window.move_window_coord(INVENTORY_POS);
                window.rel_move(offset((x, y), INVENTORY_TILE_OFFSET));
            }
            ShopBuy { x, y } => {
                window.move_window_coord((
                    SHOP_BUY_POS.0 + INVENTORY_TILE_OFFSET.0 * (x - 1) as f32,
                    SHOP_BUY_POS.1 + INVENTORY_TILE_OFFSET.1 * (y - 1) as f32,
                ));
            }
            ShopEquip { x, y } => match x {
                1 => {
                    window.move_window_coord(SHOP_EQUIPMENT_POS[(y - 1) as usize]);
                }
                2 => {
                    window.move_window_coord(SHOP_EQUIPMENT_POS[3]);
                }
                3 => {
                    window.move_window_coord(SHOP_EQUIPMENT_POS[(y + 3) as usize]);
                }
                _ => return,
            },
            ShopDropSell { x } => {
                if x == 6 {
                    window.move_window_coord(INVENTORY_DROP_BOX);
                    return;
                }
                window.move_window_coord(SHOP_SELL_BOX);
            }
            AbilityTree { x, y } => {
                window.move_window_coord(ABILITY_TREE_POS);
                window.rel_move(offset((x, y), ABILITY_TREE_OFFSET));
            }
            AbilityWheel(ability) => {
                window.move_window_coord(ABILITY_WHEEL_CENTER);
                window.rel_move(ABILITY_WHEEL_OFFSETS[ability as usize - 1]);
            }
            AbilityPointTable(y) => {
                window.move_window_coord(ABILITY_POINTS_POS);
                let y = ABILITY_POINTS_OFFSET.1 * (y as f32 - 1.);
                window.rel_move((0., y));
            }
            AbilitySelect { x } => {
                let (_, y) = window.to_window_coords(MouseCursor::pos());
                let x = ABILITY_SELECT_POS.0 + ABILITY_SELECT_OFFSET.0 * (x as f32 - 1.);
                window.move_window_coord((x, y));
            }
            AbilitySelectSlider(y) => {
                window.move_window_coord(ABILITY_SELECT_SLIDER_POS);
                let y = ABILITY_SELECT_SLIDER_OFFSET.1 * (y as f32 - 1.);
                window.rel_move((0., y));
            }
            DebugState | Manual => (),
            WhiteNovember(_) => todo!(),
            GhostBeach(_) => todo!(),
            Map(num) => {
                window.move_window_coord(MAP[(num - 1) as usize]);
            }
        }
    }
}
fn select_move<const N: usize>(window: WindowBound, index: i32, positon: [(f32, f32); N]) {
    window.move_window_coord(positon[(index - 1) as usize]);
}
fn offset(steps: (i32, i32), offset: (f32, f32)) -> (f32, f32) {
    (
        offset.0 * (steps.0 as f32 - 1.),
        offset.1 * (steps.1 as f32 - 1.),
    )
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
pub async fn wait() {
    task::sleep(Duration::from_millis(1)).await
}
fn wrap(mut x: i32, range: std::ops::Range<i32>) -> i32 {
    if x < range.start {
        x = range.end - x - 1;
    }
    (x - range.start) % (range.end - range.start) + range.start
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
