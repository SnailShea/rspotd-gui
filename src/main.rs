use slint::include_modules;

include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let ui = PotDGUI::new()?;

    ui.run()
}
