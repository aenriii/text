use crate::Application;

mod drag;
mod act;
mod resize;

pub fn apply_handlers(ui: Application) {
    ui.on_window_close(act::close(&ui));
    ui.on_window_maximize(act::maximize(&ui));
    ui.on_window_minimize(act::minimize(&ui));

    ui.on_window_drag_start(drag::drag_start(&ui));
    ui.on_window_drag_move(drag::drag_move(&ui));
    ui.on_window_drag_end(drag::drag_end(&ui));

    ui.on_window_pointer_tl_down(resize::pointer_tl_down(&ui));
    ui.on_window_pointer_tl_up(resize::pointer_tl_up(&ui));
    ui.on_window_pointer_tl_move(resize::pointer_tl_move(&ui));
    ui.on_window_pointer_tt_down(resize::pointer_tt_down(&ui));
    ui.on_window_pointer_tt_up(resize::pointer_tt_up(&ui));
    ui.on_window_pointer_tt_move(resize::pointer_tt_move(&ui));
    ui.on_window_pointer_tr_down(resize::pointer_tr_down(&ui));
    ui.on_window_pointer_tr_up(resize::pointer_tr_up(&ui));
    ui.on_window_pointer_tr_move(resize::pointer_tr_move(&ui));
    ui.on_window_pointer_ll_down(resize::pointer_ll_down(&ui));
    ui.on_window_pointer_ll_up(resize::pointer_ll_up(&ui));
    ui.on_window_pointer_ll_move(resize::pointer_ll_move(&ui));
    ui.on_window_pointer_rr_down(resize::pointer_rr_down(&ui));
    ui.on_window_pointer_rr_up(resize::pointer_rr_up(&ui));
    ui.on_window_pointer_rr_move(resize::pointer_rr_move(&ui));
    ui.on_window_pointer_bl_down(resize::pointer_bl_down(&ui));
    ui.on_window_pointer_bl_up(resize::pointer_bl_up(&ui));
    ui.on_window_pointer_bl_move(resize::pointer_bl_move(&ui));
    ui.on_window_pointer_bb_down(resize::pointer_bb_down(&ui));
    ui.on_window_pointer_bb_up(resize::pointer_bb_up(&ui));
    ui.on_window_pointer_bb_move(resize::pointer_bb_move(&ui));
    ui.on_window_pointer_br_down(resize::pointer_br_down(&ui));
    ui.on_window_pointer_br_up(resize::pointer_br_up(&ui));
    ui.on_window_pointer_br_move(resize::pointer_br_move(&ui));
}
