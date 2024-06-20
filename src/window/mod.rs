use crate::Application;

mod drag;
mod act;

pub fn apply_handlers(ui: Application) {
    ui.on_window_drag_start(drag::drag_start(&ui));
    ui.on_window_drag_move(drag::drag_move(&ui));
    ui.on_window_drag_end(drag::drag_end(&ui));

    ui.on_window_close(act::close(&ui));
    ui.on_window_maximize(act::maximize(&ui));
    ui.on_window_minimize(act::minimize(&ui));
}