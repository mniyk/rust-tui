mod app;

use color_eyre::Result;

use app::app::App;
use app::google::authentication::Authentication;

fn main() -> Result<()> {
    let authentication = Authentication::new();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new(&authentication.token_info).run(terminal);
    ratatui::restore();
    app_result
}
