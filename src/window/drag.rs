use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use lazy_static::lazy_static;
use slint::{ComponentHandle, PhysicalPosition};



lazy_static! {
    static ref INITIAL_MOUSE_POSITION: Arc<Mutex<(i32, i32)>> = Arc::new(Mutex::new((0, 0)));
    static ref DELTA_TIME: Arc<Mutex<u128>> = Arc::new(Mutex::new(0u128));
}

pub fn drag_start(ui: &impl ComponentHandle) -> impl FnMut(f32, f32) {

    let _handle = ui.as_weak();
    let mouse_pos = INITIAL_MOUSE_POSITION.clone();

    move |x, y| {
        match mouse_pos.lock() {
            Ok(mut it) => {
                log::debug!(target: "window::drag", "Starting drag at {:?}!", (x, y));
                *it = (x as i32, y as i32);
            }
            Err(it) => {
                log::error!(target: "window::drag", "Drag Poison! {:?}", it);
                mouse_pos.clear_poison();
                log::info!(target: "window::drag", "Drag Poison cleared.");
            }
        }
    }
}

pub fn drag_move(ui: &impl ComponentHandle) -> impl FnMut(f32, f32) {

    let handle = ui.clone_strong();
    let mouse_pos = INITIAL_MOUSE_POSITION.clone();
    let delta = DELTA_TIME.clone();

    move |x, y| {
        let current_time =
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("father time is rolling in his grave")
                .as_millis();
        let mut time_handle = match delta.lock() {
            Ok(it) => it,
            Err(it) => {
                log::error!(target: "window::drag", "Time Poison! {:?}", it);
                mouse_pos.clear_poison();
                log::warn!(target: "window::drag", "Time Poison cleared, forcing it in.");
                delta.lock().unwrap()
            }
        };
        // move window max of 40 times/s
        if current_time - *time_handle < 25 {
            return;
        }
        match mouse_pos.lock() {
            Ok(it) => {
                // do not
                if *it == (x as i32, y as i32) {
                    log::trace!(target: "window::drag", "Not moving, no diff!");
                    return;
                }

                // edit deltatime to now
                *time_handle = current_time;

                let window = handle.window();
                let new_position = {
                    let current_screen_pos = window.position();
                    let diff = (
                        x as i32 - it.0,
                        y as i32 - it.1
                    );
                    log::trace!(
                        target: "window::drag",
                        "Moving window {:?} (mouse_pos: {:?})",
                        diff, (x as i32, y as i32)
                    );
                    PhysicalPosition::new(
                        current_screen_pos.x + diff.0,
                        current_screen_pos.y + diff.1
                    )
                };

                window.set_position(new_position)

            }
            Err(it) => {
                log::error!(target: "window::drag", "Drag Poison! {:?}", it);
                mouse_pos.clear_poison();
                log::info!(target: "window::drag", "Drag Poison cleared.");
            }
        }
    }
}

pub fn drag_end(ui: &impl ComponentHandle) -> impl FnMut(f32, f32) {

    let _handle = ui.as_weak();
    let mouse_pos = INITIAL_MOUSE_POSITION.clone();

    move |_x, _y| {
        match mouse_pos.lock() {
            Ok(mut it) => {
                *it = (0, 0);
                log::debug!(target: "window::drag", "Ending drag!")
            }
            Err(it) => {
                log::error!(target: "window::drag", "Drag Poison! {:?}", it);
                mouse_pos.clear_poison();
                log::info!(target: "window::drag", "Drag Poison cleared.");
            }
        }
    }
}
