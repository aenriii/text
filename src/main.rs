mod window;

slint::include_modules!();
#[allow(unused_assignments)]
fn main() -> Result<(), slint::PlatformError>{

    env_logger::init();
    let ui = Application::new()?;
    window::apply_handlers(ui.clone_strong());

    ui.run()
}