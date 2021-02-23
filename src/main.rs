use std::env;
use wry::{Application, Attributes, Result};

fn main() -> Result<()> {
    let url = if let Some(u) = env::args().nth(1) {
        u
    } else {
        println!(
            "Usage: {} <url>",
            env::args().nth(0).unwrap_or("cargo run --".to_string())
        );
        return Ok(());
    };

    let mut app = Application::new()?;
    let attributes = Attributes {
        url: Some(url.to_string()),
        ..Default::default()
    };
    app.add_window(attributes, None)?;
    app.run();
    Ok(())
}
