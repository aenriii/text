use slint::ComponentHandle;



pub fn minimize(ui: &impl ComponentHandle) -> impl FnMut() {

    let handle = ui.clone_strong();

    move || {
        log::debug!(target: "window::act", "Minimizing!");
        handle.window().set_minimized(true);
    }
}

pub fn maximize(ui: &impl ComponentHandle) -> impl FnMut() {

    let handle = ui.clone_strong();

    move || {
        log::debug!(target: "window::act", "Maximizing!");
        let window = handle.window();
        window.set_maximized(!window.is_maximized());
        window.request_redraw()
    }
}

pub fn close(ui: &impl ComponentHandle) -> impl FnMut() {

    let handle = ui.clone_strong();

    move || {
        log::debug!(target: "window::act", "Closing window!");
        let _ = handle.window().hide();
    }
}