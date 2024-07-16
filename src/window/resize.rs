use std::{sync::{Arc, Mutex}, time::{Duration, Instant}};
use slint::{ComponentHandle, PhysicalPosition, PhysicalSize};
use lazy_static::lazy_static;
use paste::paste;
#[derive(Debug, Clone, Copy)]
enum Side {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight
}

impl Side {
    pub fn inverse(&self) -> Side {
        match self {
            Self::TopLeft => Self::BottomRight,
            Self::Top => Self::Bottom,
            Self::TopRight => Self::BottomLeft,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::BottomLeft => Self::TopRight,
            Self::Bottom => Self::Top,
            Self::BottomRight => Self::TopLeft
        }
    }
}
impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
            Self::TopLeft => String::from("Side::TopLeft"),
            Self::Top => String::from("Side::Top"),
            Self::TopRight => String::from("Side::TopRight"),
            Self::Left => String::from("Side::Left"),
            Self::Right => String::from("Side::Right"),
            Self::BottomLeft => String::from("Side::BottomLeft"),
            Self::Bottom => String::from("Side::Bottom"),
            Self::BottomRight => String::from("Side::BottomRight")
        }
    }
}

struct Resizer {
    target_side: Side,
    anchor: Side,

    last_tick: Instant,
    last_mouse_xy: (i32, i32),
    interval: Duration,

}
impl Resizer {
    pub fn of_side(side: Side) -> Resizer {
        Resizer {
            target_side: side,
            anchor: side.inverse(),
            last_tick: Instant::now() - Duration::from_secs(1),
            last_mouse_xy: (0,0),
            interval: Duration::from_millis(25)
        }
    }

    pub fn pointer_down(&mut self, _ui: impl ComponentHandle, xy: (i32, i32)) {
        log::debug!(target: "window::resize", "Pointer down on side {}!", self.target_side.to_string());

        self.last_mouse_xy = xy;


    }
    pub fn pointer_up(&mut self, _ui: impl ComponentHandle, _xy: (i32, i32)) {
        log::debug!(target: "window::resize", "Pointer up on side {}!", self.target_side.to_string());

        self.last_mouse_xy = (0,0);
    }
    // pub fn pointer_move(&mut self, ui: impl ComponentHandle, xy: (i32, i32)) {
    //     log::trace!(target: "window::resize", "Pointer move on side {}!", self.target_side.to_string());

    //     if self.last_tick.elapsed() < self.interval {
    //         return;
    //     }
    //     if self.last_mouse_xy == xy {
    //         log::debug!(target: "window::resize", "Not resizing, no diff!");
    //         return
    //     }

    //     let window = ui.window();
    //     let current_window_position = window.position();
    //     let current_window_size = window.size();
    //     let anchor_position = {
    //         use Side::*;
    //         match self.anchor {
    //             TopLeft | Top | Left => current_window_position,
    //             TopRight => {
    //                 PhysicalPosition::new(
    //                     current_window_position.x + current_window_size.width as i32,
    //                     current_window_position.y
    //                 )
    //             },
    //             BottomRight | Bottom | Right => {
    //                 PhysicalPosition::new(
    //                     current_window_position.x + current_window_size.width as i32,
    //                     current_window_position.y + current_window_size.height as i32,
    //                 )
    //             },
    //             BottomLeft => {
    //                 PhysicalPosition::new(
    //                     current_window_position.x,
    //                     current_window_position.y + current_window_size.height as i32
    //                 )
    //             }
    //         }
    //     };

    //     let dist_relative_to_target_side = {
    //         use Side::*;
    //         match self.target_side {
    //             TopLeft | TopRight => (xy.0 - self.last_mouse_xy.0, xy.1 + self.last_mouse_xy.1),
    //             BottomLeft | BottomRight => (xy.0 + self.last_mouse_xy.0, xy.1 - self.last_mouse_xy.1),
    //             Top => (0, xy.1 + self.last_mouse_xy.1),
    //             Bottom => (0, xy.1 - self.last_mouse_xy.1),
    //             Left | Right => (xy.0 - self.last_mouse_xy.0, 0),
    //         }
    //     };

    //     log::debug!(target: "window::resize", "dist_relative_to_target_side: {:?}", dist_relative_to_target_side);

    //     let new_window_size = {
    //         use Side::*;
    //         match self.target_side {
    //             Top | TopRight | TopLeft => {
    //                 PhysicalSize::new(
    //                     (current_window_size.width as i32 - dist_relative_to_target_side.0) as u32,
    //                     (current_window_size.height as i32 - dist_relative_to_target_side.1) as u32
    //                 )
    //             }
    //             _ => {
    //                 PhysicalSize::new(
    //                     (current_window_size.width as i32 + dist_relative_to_target_side.0) as u32,
    //                     (current_window_size.height as i32 + dist_relative_to_target_side.1) as u32
    //                 )
    //             }
    //             // Top => {
    //             //     PhysicalSize::new(
    //             //         , height)
    //             // }
    //         }
    //     };

    //     // anchor point & new_window_size to determine position of TL corner

    //     let new_window_position = {
    //         use Side::*;
    //         match self.target_side {
    //             TopLeft | Left | Top => {
    //                 PhysicalPosition::new(
    //                     current_window_position.x + ((new_window_size.width as i32 - current_window_size.width as i32)),
    //                     current_window_position.y + ((new_window_size.height as i32 - current_window_size.height as i32))
    //                 )
    //             },
    //             TopRight => {
    //                 PhysicalPosition::new(
    //                     anchor_position.x - new_window_size.width as i32,
    //                     anchor_position.y + (new_window_size.height as i32 - current_window_size.height as i32)
    //                 )
    //             },
    //             BottomRight | Bottom | Right => current_window_position,
    //             BottomLeft => {
    //                 PhysicalPosition::new(
    //                     current_window_position.x,
    //                     current_window_position.y - new_window_size.height as i32
    //                 )
    //             }
    //         }
    //     };

    //     log::info!(target: "window::resize", "Window is resizing from {:?} to {:?}, and moving from {:?} to {:?}",
    //         (current_window_size.width, current_window_size.height),
    //         (new_window_size.width, new_window_size.height),
    //         (current_window_position.x, current_window_position.y),
    //         (new_window_position.x, new_window_position.y)
    //     );

    //     log::info!(target: "window::resize", "diff of window_size: {:?}, diff of window position: {:?}",
    //         (current_window_size.width as i32 - new_window_size.width as i32, current_window_size.height as i32 - new_window_size.height as i32),
    //         (current_window_position.x - new_window_position.x, current_window_position.y - new_window_position.y)
    //     );
    //     window.set_position(new_window_position);
    //     window.set_size(new_window_size);
    //     {
    //         use Side::*;
    //         match self.anchor {

    //             Bottom | BottomRight | Right => {}
    //             _ => {}
    //         }
    //     };

    //     // use Side::*;
    //     // match self.target_side {
    //     //     Bottom | BottomRight | Right => {
    //     //         window.set_size(new_window_size);
    //     //     }
    //     // }
    // }
    //
    pub fn pointer_move(&mut self, ui: impl ComponentHandle, xy: (i32, i32)) {
        log::trace!(target: "window::resize", "Pointer move on side {}!", self.target_side.to_string());

        if self.last_tick.elapsed() < self.interval {
            return;
        }
        if self.last_mouse_xy == xy {
            log::debug!(target: "window::resize", "Not resizing, no diff!");
            return
        }

        let window = ui.window();

        use Side::*;
        match self.target_side {
            Top => {
                let change_y = self.last_mouse_xy.1 - xy.1;
                window.set_size({
                    let current_size = window.size();
                    let new_size = PhysicalSize::new(
                        current_size.width,
                        current_size.height + (change_y as u32).min(current_size.height - 100)
                    );
                    log::trace!(
                        target: "window::resize",
                        "[SIZ] ({},{}) -> ({},{}) (diff ({}, {}))",
                        current_size.width,
                        current_size.height,
                        new_size.width,
                        new_size.height,
                        current_size.width - new_size.width,

                        current_size.height as i32 - new_size.height as i32
                    );
                    new_size
                });
                window.set_position({
                    let current_position = window.position();
                    let new_position = PhysicalPosition::new(
                        current_position.x,
                        current_position.y - change_y.max(0)
                    );
                    log::trace!(
                        target: "window::resize",
                        "[POS] ({},{}) -> ({},{}) (diff ({}, {}))",
                        current_position.x,
                        current_position.y,
                        new_position.x,
                        new_position.y,
                        current_position.x - new_position.x,

                        current_position.y - new_position.y
                    );
                    new_position
                });
                self.last_mouse_xy = xy;
            }
            _ => { unimplemented!("hi") }
        }
    }


}

lazy_static! {
    static ref TL: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::TopLeft)));
    static ref TT: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::Top)));
    static ref TR: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::TopRight)));
    static ref LL: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::Left)));
    static ref RR: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::Right)));
    static ref BL: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::BottomLeft)));
    static ref BB: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::Bottom)));
    static ref BR: Arc<Mutex<Resizer>> = Arc::new(Mutex::new(Resizer::of_side(Side::BottomRight)));
}

macro_rules! resize_handlers {
    ($($resizer: expr),*) => {
        $(
            paste! {
                pub fn [<pointer_ $resizer:lower _up>](ui: &impl ComponentHandle) -> impl FnMut(f32, f32) {
                    let _handle = ui.as_weak();
                    let resizer = $resizer.clone();

                    move |x, y| {
                        match resizer.lock() {
                            Ok(mut it) => {
                                it.pointer_up(_handle.upgrade().unwrap(), (x as i32, y as i32))
                            }
                            Err(_it) => {
                                log::error!(target: "window::resize", "Poison {}!", stringify!($resizer));
                                resizer.clear_poison();
                                log::info!(target: "window::resize", "{} Poison cleared.", stringify!($resizer));
                            }
                        }
                    }
                }
                pub fn [<pointer_ $resizer:lower _down>](ui: &impl ComponentHandle) -> impl FnMut(f32, f32) {
                    let _handle = ui.as_weak();
                    let resizer = $resizer.clone();

                    move |x, y| {
                        match resizer.lock() {
                            Ok(mut it) => {
                                it.pointer_down(_handle.upgrade().unwrap(), (x as i32, y as i32))
                            }
                            Err(_it) => {
                                log::error!(target: "window::resize", "Poison {}!", stringify!($resizer));
                                resizer.clear_poison();
                                log::info!(target: "window::resize", "{} Poison cleared.", stringify!($resizer));
                            }
                        }
                    }
                }
                pub fn [<pointer_ $resizer:lower _move>](ui: &impl ComponentHandle) -> impl FnMut(f32, f32) {
                    let _handle = ui.as_weak();
                    let resizer = $resizer.clone();

                    move |x, y| {
                        match resizer.lock() {
                            Ok(mut it) => {
                                it.pointer_move(_handle.upgrade().unwrap(), (x as i32, y as i32))
                            }
                            Err(_it) => {
                                log::error!(target: "window::resize", "Poison {}!", stringify!($resizer));
                                resizer.clear_poison();
                                log::info!(target: "window::resize", "{} Poison cleared.", stringify!($resizer));
                            }
                        }
                    }
                }
            }
        )*
    };
}
resize_handlers!(TL, TT, TR, LL, RR, BL, BB, BR);
