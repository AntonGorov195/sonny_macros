use async_std::{self, task};
use inputbot::KeybdKey::*;
use inputbot::{KeybdKey, MouseButton, MouseCursor};
use std::{fmt::Debug, time::Duration};

pub struct Macro {
    state: MacroState,
    window_bounds: WindowBound,
    saved_pos: (f32, f32),
    auto_left_click: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum MacroState {
    Neutral,
    DebugState,
    // WinSizeSelect,
    // WindowSizePicker,
    BattleAbilitySelect { character: i32 },
    BattleCharacterSelection,
    Inventory { x: i32, y: i32 },
    InventoryEquip { x: i32, y: i32 },
    InventoryProfile(i32),
    InventoryDrop,
    BottomTabs(i32),
    Infinity,
    Manual,
    BuffView,
    Victory,
    ShopInventory { x: i32, y: i32 },
    ShopEquip { x: i32, y: i32 },
    ShopBuy { x: i32, y: i32 },
    ShopDropSell { x: i32 },
    AbilityScreen { x: i32, y: i32 },
    AbilityWheel,
    AbilityPointTable(i32),
}
use MacroState::*;
#[derive(Debug, Clone, Copy, Default)]
pub struct WindowBound {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
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
const SENSITIVITY: (f32, f32) = (0.01, 0.01);
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
const SHOP_SELL_BOX: (f32, f32) = (0.855, 0.666);
const SHOP_BUY_POS: (f32, f32) = (0.38125, 0.5);
impl Macro {
    pub fn new() -> Self {
        println!("Neutral State");
        Self {
            window_bounds: Default::default(),
            state: MacroState::Neutral,
            saved_pos: (0.5, 0.5),
            auto_left_click: false,
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
            // XKey => {
            //     let (x, y) = self.to_screen_coords((0.939, 0.076));
            //     MouseCursor::move_abs(x, y);
            //     left_click();
            //     return;
            // }
            _ => (),
        }
        match self.state {
            Neutral => match key {
                SKey => {
                    // self.inventory_set_mouse(1, 1);
                    self.set_state(ShopInventory { x: 1, y: 1 })
                }
                EKey => {
                    //                     if KeybdKey::is_alt() {
                    //                         self.win_bounds = crate::window::get_window_size();
                    //                         println!("{:?}", self.win_bounds);
                    //                         return;
                    //                     }
                    //                     self.set_state(WindowSizePicker);
                    //
                    //                     let pos = MouseCursor::pos();
                    //                     self.win_bounds.left = pos.0;
                    //                     self.win_bounds.top = pos.1;
                    //                     println!("Top left corner: {:?}", pos);
                }
                RBracketKey => self.set_state(DebugState),
                BKey => self.set_state(BattleCharacterSelection),
                IKey => {
                    {
                        // self.inventory_set_mouse(1, 1);
                        self.set_state(Inventory { x: 1, y: 1 })
                    };
                }
                OKey => self.set_state(Infinity),
                VKey => self.set_state(Victory),
                TKey => {
                    // self.move_window_coord(MAP_TAB_POS);
                    self.set_state(BottomTabs(6));
                }
                UKey => {
                    self.set_state(Manual);
                }
                AKey => {
                    self.set_state(AbilityScreen { x: 1, y: 1 });
                    // self.move_window_coord(ABILITY_TREE_POS);
                }
                _ => (),
            },
            DebugState => {
                if let Some(num) = key.to_num() {
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
                    return;
                };
                let pos = self.to_window_coords(MouseCursor::pos());
                println!("{:?}", self.to_window_coords(MouseCursor::pos()));
                self.saved_pos = pos;
            }
            // WinSizeSelect => {
            //     let mut buf = String::new();
            //     io::stdin().read_line(&mut buf);
            //     match buf
            //         .split(" ")
            //         .filter(|x| !x.is_empty())
            //         .map(|x| x.parse::<i32>().unwrap())
            //         .collect::<Vec<_>>()[..]
            //     {
            //         [left, top, right, bottom, ..] => {
            //             self.win_bounds = WindowBound {
            //                 left,
            //                 top,
            //                 right,
            //                 bottom,
            //             }
            //         }
            //         _ => (),
            //     }
            //     self.set_state(MacroState::Neutral);
            // }
            // WindowSizePicker => {
            //     if key != EKey {
            //         return;
            //     }
            //     let pos = MouseCursor::pos();
            //     self.win_bounds.right = pos.0;
            //     self.win_bounds.bottom = pos.1;
            //     println!("Bottom right corner: {:?}", pos);
            //     self.center_mouse();
            //     self.set_state(Neutral);
            // }
            BattleAbilitySelect { character } => {
                match key {
                    BKey => {
                        // self.center_mouse();
                        self.set_state(BattleCharacterSelection);
                        return;
                    }
                    VKey => {
                        self.set_state(Victory);
                        // self.move_window_coord(VICTORY_SCREEN_ITEMS);
                        return;
                    }
                    _ => (),
                }
                // self.move_window_coord(BATTLE_CHARCTER_SELECT[character as usize - 1]);
                // ability select.
                let Some(num)=key.to_num() else {
                    return;
                };
                self.set_state(BattleAbilitySelect { character });
                self.ability_wheel(num);
                // let mut times = 1;
                // if self.should_double_click() {
                //     times += 1;
                // }
                // left_click_repeat(times);
            }
            BattleCharacterSelection => {
                if key == VKey {
                    self.set_state(Victory);
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
                        self.set_state(BuffView);
                    }
                    1..=9 => {
                        self.set_state(BattleAbilitySelect { character: num });
                    }
                    _ => return,
                }
            }
            Inventory { mut x, mut y } => {
                if key == DKey {
                    self.set_state(InventoryDrop);
                }
                if key == VKey {
                    self.set_state(Victory);
                    return;
                }
                if key == BackspaceKey {
                    self.window_bounds.move_window_coord(CLOSE_MENU_WINDOW);
                    left_click();
                    return;
                }
                if key == PKey {
                    self.set_state(InventoryProfile(1));
                    return;
                }
                if let Some(arrow) = KeybdKey::get_arrow() {
                    x += arrow.0;
                    y += arrow.1;

                    match (x, y) {
                        (_, ..=0) => return,
                        (1..=6, 1..=6) => {
                            self.set_state(Inventory { x, y });
                        }
                        (_, 7) => {
                            self.set_state(InventoryDrop);
                            return;
                        }
                        (..=0, _) => {
                            let y = y.min(3).max(1);
                            self.set_state(InventoryEquip { x: 3, y: y });
                            return;
                        }
                        (7.., _) => {
                            let y = y.min(3).max(1);
                            self.set_state(InventoryEquip { x: 1, y: y });
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
                        self.set_state(InventoryEquip { x, y });
                    }
                    2 => {
                        self.set_state(InventoryEquip { x, y });
                    }
                    3 => {
                        self.set_state(InventoryEquip { x, y });
                    }
                    ..=0 => {
                        self.set_state(Inventory { x: 6, y: y });
                    }
                    4.. => {
                        self.set_state(Inventory { x: 1, y: y });
                    }
                }
            }
            InventoryProfile(mut x) => {
                let Some(arrow) = KeybdKey::get_arrow() else {
                    return;
                };
                if arrow.1 < 0 {
                    let (x, y) = (x, 6);
                    self.set_state(Inventory { x, y });
                    return;
                }
                x += arrow.0;
                if x == 7 || x == 0 {
                    self.set_state(InventoryDrop);
                    return;
                }
                self.set_state(InventoryProfile(x));
            }
            InventoryDrop => {
                let Some(arrow)=KeybdKey::get_arrow()else {
                    return;
                };
                match arrow {
                    (_, -1) => {
                        self.set_state(Inventory { x: 6, y: 6 });
                    }
                    (1, _) => {
                        self.set_state(InventoryProfile(1));
                    }
                    (-1, _) => {
                        self.set_state(InventoryProfile(6));
                    }
                    _ => (),
                }
                return;
            }
            BottomTabs(x) => {
                let Some(tab) = key.to_num() else{
                    return;
                };
                self.set_state(BottomTabs(tab));
                //                 match tab {
                //                     7 => {
                //                         self.move_window_coord(EXIT_GAME_X);
                //                     }
                //                     6 => {
                //                         self.move_window_coord(SKIP_TURN_POS);
                //                     }
                //                     1..=5 => {
                //                         let offset = 0.072 * (tab - 1) as f32;
                //                         let (x, y) = self.to_screen_coords((offset + 0.068, 0.85));
                //                         MouseCursor::move_abs(x, y);
                //
                //                         if self.should_double_click() {
                //                             left_click();
                //                         }
                //                     }
                //                     _ => return,
                //                 }
            }
            //             Infinity => {
            //                 let Some(num)=key.to_num() else {
            //                     return;
            //                 };
            //                 if num > 6 {
            //                     return;
            //                 }
            //                 let (x, y) = self.to_screen_coords(INFINITY_OVERWORLD[(num - 1) as usize]);
            //                 MouseCursor::move_abs(x, y);
            //                 if KeybdKey::is_alt() {
            //                     return;
            //                 }
            //                 left_click();
            //             }
            //             Manual => {
            //                 let (mut x, mut y) = (0., 0.);
            //                 if LeftKey.is_pressed() {
            //                     x -= 1.;
            //                 }
            //                 if RightKey.is_pressed() {
            //                     x += 1.;
            //                 }
            //                 if UpKey.is_pressed() {
            //                     y -= 1.;
            //                 }
            //                 if DownKey.is_pressed() {
            //                     y += 1.;
            //                 }
            //                 x *= SENSITIVITY.0;
            //                 y *= SENSITIVITY.1;
            //                 if KeybdKey::is_alt() {
            //                     x *= 3.;
            //                     y *= 3.;
            //                 }
            //                 let (x, y) = self.to_screen_coords((x, y));
            //                 let (x, y) = (x - self.window_bounds.left, y - self.window_bounds.top);
            //                 MouseCursor::move_rel(x, y);
            //             }
            //             BuffView => {
            //                 if self.arrow_move(BATTLE_BUFF_OFFSET) {
            //                     return;
            //                 }
            //                 let Some(num)=key.to_num() else{
            //                     return;
            //                 };
            //                 match num {
            //                     2 => {
            //                         self.center_mouse();
            //                         self.set_state(BattleCharacterSelection);
            //                     }
            //                     8 => return,
            //                     1..=9 => {
            //                         self.move_window_coord(BATTLE_FIRST_BUFF_POS[num as usize - 1]);
            //                     }
            //                     _ => return,
            //                 }
            //             }
            //             Victory => {
            //                 if self.arrow_move(INVENTORY_TILE_OFFSET) {
            //                     return;
            //                 }
            //                 match key {
            //                     VKey => {
            //                         self.move_window_coord(VICTORY_SCREEN_ITEMS);
            //                         return;
            //                     }
            //                     IKey => {
            //                         self.inventory_set_mouse(1, 1);
            //                         self.set_state(Inventory {
            //                             x: 1,
            //                             y: 1,
            //                             selected_col: None,
            //                         });
            //                         return;
            //                     }
            //                     XKey => {
            //                         self.move_window_coord(VICTORY_PROCEED);
            //                         if self.should_double_click() {
            //                             left_click();
            //                             return;
            //                         }
            //                     }
            //                     BKey => {
            //                         self.set_state(BattleCharacterSelection);
            //                         self.center_mouse();
            //                         return;
            //                     }
            //                     _ => return,
            //                 }
            //             }
            //             ShopInventory {
            //                 mut x,
            //                 mut y,
            //                 selected_col,
            //             } => {
            //                 if key == XKey {
            //                     self.move_window_coord(CLOSE_MENU_WINDOW);
            //                     left_click();
            //                     return;
            //                 }
            //                 if let Some(arrow) = KeybdKey::get_arrow() {
            //                     x += arrow.0;
            //                     y += arrow.1;
            //
            //                     // x = wrap(x, 1..10);
            //                     if y < 1 {
            //                         return;
            //                     }
            //                     if self.inventory_set_mouse(x, y) {
            //                         self.set_state(ShopInventory { x, y, selected_col });
            //                         return;
            //                     }
            //                     match (x, y) {
            //                         (6, 7) => {
            //                             self.set_state(ShopDropSell { x: 6 });
            //                             self.move_window_coord(INVENTORY_DROP_BOX);
            //                             return;
            //                         }
            //                         (_, 7) => {
            //                             self.set_state(ShopDropSell { x: x });
            //                             self.move_window_coord(SHOP_SELL_BOX);
            //                             return;
            //                         }
            //                         (..=0, _) if y > 3 => {
            //                             self.set_state(ShopBuy { x: 5, y: y - 3 });
            //                             self.move_window_coord((
            //                                 SHOP_BUY_POS.0 + INVENTORY_TILE_OFFSET.1 * 3. as f32,
            //                                 SHOP_BUY_POS.1 + INVENTORY_TILE_OFFSET.1 * (y - 4) as f32,
            //                             ));
            //                         }
            //                         (7.., _) if y > 3 => {
            //                             self.set_state(ShopBuy { x: 1, y: y - 3 });
            //                             self.move_window_coord((
            //                                 SHOP_BUY_POS.0,
            //                                 SHOP_BUY_POS.1 + INVENTORY_TILE_OFFSET.1 * (y - 4) as f32,
            //                             ));
            //                         }
            //                         (..=0, _) => {
            //                             let y = y.min(3).max(1);
            //                             self.set_state(ShopEquip { x: 3, y: y });
            //                             self.move_window_coord(SHOP_EQUIPMENT_POS[(y + 3) as usize]);
            //                             return;
            //                         }
            //                         (7.., _) => {
            //                             let y = y.min(3).max(1);
            //                             self.set_state(ShopEquip { x: 1, y: y });
            //                             self.move_window_coord(SHOP_EQUIPMENT_POS[(y - 1) as usize]);
            //                             return;
            //                         }
            //                         _ => return,
            //                     }
            //                     return;
            //                 }
            //             }
            //             ShopEquip { x, y } => {
            //                 let Some(arrow) = KeybdKey::get_arrow() else {
            //                     return;
            //                 };
            //                 let (x, y) = (arrow.0 + x, (arrow.1 + y).min(3).max(1));
            //
            //                 match x {
            //                     1 => {
            //                         self.set_state(ShopEquip { x, y });
            //                         self.move_window_coord(SHOP_EQUIPMENT_POS[(y - 1) as usize]);
            //                     }
            //                     2 => {
            //                         self.set_state(ShopEquip { x, y });
            //                         self.move_window_coord(SHOP_EQUIPMENT_POS[3]);
            //                     }
            //                     3 => {
            //                         self.set_state(ShopEquip { x, y });
            //                         self.move_window_coord(SHOP_EQUIPMENT_POS[(y + 3) as usize]);
            //                     }
            //                     ..=0 => {
            //                         self.set_state(ShopInventory {
            //                             x: 6,
            //                             y: y,
            //                             selected_col: None,
            //                         });
            //                         self.inventory_set_mouse(6, y);
            //                     }
            //                     4.. => {
            //                         self.set_state(ShopInventory {
            //                             x: 1,
            //                             y: y,
            //                             selected_col: None,
            //                         });
            //                         self.inventory_set_mouse(1, y);
            //                     }
            //                 }
            //             }
            //             ShopBuy { x, y } => {
            //                 let Some(arrow)=KeybdKey::get_arrow() else{
            //                     return;
            //                 };
            //                 let (x, y) = (x + arrow.0, y + arrow.1);
            //                 match (x, y) {
            //                     (x, _) if x < 1 => {
            //                         self.set_state(ShopInventory {
            //                             x: 6,
            //                             y: y + 3,
            //                             selected_col: None,
            //                         });
            //                         self.inventory_set_mouse(6, y + 3);
            //                     }
            //                     (x, _) if x > 5 => {
            //                         self.set_state(ShopInventory {
            //                             x: 1,
            //                             y: y + 3,
            //                             selected_col: None,
            //                         });
            //                         self.inventory_set_mouse(1, y + 3);
            //                     }
            //                     _ => {
            //                         self.set_state(ShopBuy { x, y });
            //                         self.move_window_coord((
            //                             SHOP_BUY_POS.0 + INVENTORY_TILE_OFFSET.0 * (x - 1) as f32,
            //                             SHOP_BUY_POS.1 + INVENTORY_TILE_OFFSET.1 * (y - 1) as f32,
            //                         ));
            //                     }
            //                 }
            //             }
            //             ShopDropSell { x } => {
            //                 let Some(arrow) = KeybdKey::get_arrow() else {
            //                     return;
            //                 };
            //                 match (x, arrow) {
            //                     (_, (1.., _)) => {
            //                         self.set_state(ShopDropSell { x: 6 });
            //                         self.move_window_coord(INVENTORY_DROP_BOX);
            //                     }
            //                     (x, (_, ..=-1)) => {
            //                         self.set_state(ShopInventory {
            //                             x: x,
            //                             y: 6,
            //                             selected_col: None,
            //                         });
            //                         self.inventory_set_mouse(x, 6);
            //                     }
            //                     (_, (..=-1, _)) => {
            //                         self.set_state(ShopDropSell { x: 5 });
            //                         self.move_window_coord(SHOP_SELL_BOX);
            //                     }
            //                     _ => (),
            //                 }
            //             }
            //             AbilityScreen { x, y } => {
            //                 if key == WKey {
            //                     self.set_state(AbilityWheel);
            //                     self.move_window_coord(ABILITY_WHEEL_CENTER);
            //                     return;
            //                 }
            //                 let Some(arrow)=KeybdKey::get_arrow() else{
            //                     return;
            //                 };
            //                 let (x, y) = (x + arrow.0, y + arrow.1);
            //                 match (x, y) {
            //                     (..=0, _) | (5.., _) | (_, ..=0) | (_, 8..) => {
            //                         return;
            //                     }
            //                     _ => (),
            //                 }
            //                 let offset = (
            //                     ABILITY_TREE_OFFSET.0 * (x - 1) as f32,
            //                     ABILITY_TREE_OFFSET.1 * (y - 1) as f32,
            //                 );
            //                 let pos = (ABILITY_TREE_POS.0 + offset.0, ABILITY_TREE_POS.1 + offset.1);
            //                 self.move_window_coord(pos);
            //                 self.set_state(AbilityScreen { x: x, y: y });
            //             }
            //             AbilityWheel => {
            //                 if key == WKey {
            //                     self.set_state(AbilityPointTable(1));
            //                     return;
            //                 }
            //                 self.move_window_coord(ABILITY_WHEEL_CENTER);
            //                 // ability select.
            //                 let Some(num)=key.to_num() else {
            //                     return;
            //                 };
            //                 let (x, y) = ABILITY_WHEEL_OFFSETS[num as usize - 1];
            //
            //                 let (x, y) = self.to_screen_coords((x, y));
            //                 let (x, y) = (x - self.window_bounds.left, y - self.window_bounds.top);
            //                 MouseCursor::move_rel(x, y);
            //             }
            //             AbilityPointTable(x) => {
            //                 let Some(num) = key.to_num() else {
            //                     return;
            //                 };
            //                 self.move_window_coord((
            //                     ABILITY_POINTS_POS.0,
            //                     ABILITY_POINTS_POS.1 + (num - 1) as f32 * ABILITY_POINTS_OFFSET.1,
            //                 ));
            //             }
            _ => (),
        }
    }

    fn ability_wheel(&mut self, num: i32) {
        let (x, y) = ABILITY_WHEEL_OFFSETS[num as usize - 1];
        let (x, y) = self.to_screen_coords((x, y));
        let (x, y) = (x - self.window_bounds.left, y - self.window_bounds.top);
        MouseCursor::move_rel(x, y);
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
            self.rel_move((x, y));
            return true;
        }
        false
    }
    fn should_double_click(&self) -> bool {
        KeybdKey::is_alt() != self.auto_left_click
    }
    // fn move_window_coord(&mut self, coord: (f32, f32)) {
    //     let (x, y) = self.to_screen_coords(coord);
    //     MouseCursor::move_abs(x, y);
    // }
    fn set_state(&mut self, state: MacroState) {
        state.move_mouse(self.window_bounds);
        self.state = state;
        println!("{:?} State", state);
    }
    fn skip_turn(&mut self) {
        self.window_bounds.move_window_coord(SKIP_TURN_POS);
        if !self.should_double_click() {
            return;
        }
        left_click()
    }
    fn rel_move(&mut self, (x, y): (f32, f32)) {
        let (x, y) = self.to_screen_coords((x, y));
        let (x, y) = (x - self.window_bounds.left, y - self.window_bounds.top);
        MouseCursor::move_rel(x, y);
    }

    fn window_size(&self) -> (i32, i32) {
        self.window_bounds.window_size()
    }
    fn window_center(&self) -> (i32, i32) {
        self.window_bounds.window_center()
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
impl MacroState {
    pub fn move_mouse(&self, window: WindowBound) {
        match *self {
            Neutral => window.center_mouse(),
            BuffView => {
                window.move_window_coord(BATTLE_BUFF_CENTER_POS);
            }
            BattleAbilitySelect { character } => {
                window.move_window_coord(BATTLE_CHARCTER_SELECT[character as usize - 1]);
            }
            Victory => window.move_window_coord(VICTORY_SCREEN_ITEMS),
            BattleCharacterSelection => window.center_mouse(),
            AbilityScreen { x, y } => window.move_window_coord(ABILITY_TREE_POS),
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
            Inventory { x, y } => Self::inventory_set_mouse(x, y, window),
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
                let (x, y) = (
                    INVENTORY_PROFILE_POS.0 + INVENTORY_PROFILE_OFFSET_X * (x - 1) as f32,
                    INVENTORY_PROFILE_POS.1,
                );
                window.move_window_coord((x, y));
            }
            InventoryDrop => {
                window.move_window_coord(INVENTORY_DROP_BOX);
            }
            ShopInventory { x, y } => Self::inventory_set_mouse(x, y, window),
            DebugState | Manual => (),
            _ => (),
        }
    }
    pub fn inventory_set_mouse(x: i32, y: i32, win: WindowBound) {
        let (x, y) = (
            INVENTORY_TILE_OFFSET.0 * (x - 1) as f32 + INVENTORY_POS.0,
            INVENTORY_TILE_OFFSET.1 * (y - 1) as f32 + INVENTORY_POS.1,
        );
        win.move_window_coord((x, y));
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
