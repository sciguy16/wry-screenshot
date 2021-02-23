use std::env;
use std::fs::File;
use std::io::Write;
use std::thread;
use url::Url;
use wry::{Application, Attributes, Callback, Result, WindowProxy};

const HTML2CANVAS: &str = include_str!("../dist/html2canvas.min.js");
const TAKE_SCREENSHOT: &str = r#"
window.onload = function() {
  console.log("Starting window.onload");
  html2canvas(document.body, {
  	    useCORS: false
  	})
    .then(function(canvas) {
      console.log("Received canvas");
      var data_url = canvas.toDataURL();
      callback(data_url);
});
}
"#;

fn main() -> Result<()> {
    let url = if let Some(u) = env::args().nth(1) {
        u
    } else {
        println!(
            "Usage: {} <url>",
            env::args()
                .next()
                .unwrap_or_else(|| "cargo run --".to_string())
        );
        return Ok(());
    };

    let mut app = Application::new()?;
    let attributes = Attributes {
        url: Some(url),
        debug: true,
        visible: false,
        ..Default::default()
    };
    let window_proxy: WindowProxy = app.add_window(
        attributes,
        Some(vec![Callback {
            name: "callback".to_string(),
            function: Box::new(callback),
        }]),
    )?;

    let handle = thread::spawn(move || do_the_thing(window_proxy));

    app.run();

    handle.join().unwrap()?;

    Ok(())
}

fn callback(_window_proxy: WindowProxy, _seq: i32, args: Vec<String>) -> i32 {
    const SUCCESS: i32 = 0;
    const FAILURE: i32 = -1;

    println!("Received callback");
    if let Some(u) = args.iter().next() {
        if let Ok(u) = Url::parse(u) {
            // got a uri
            if u.scheme() == "data" {
                if u.path().starts_with("image/png;base64,") {
                    if let Some(encoded) = u.path().split(',').nth(1) {
                        // encoded is the base64-encoded image
                        let decoded = base64::decode(&encoded).unwrap();
                        let mut f = File::create("image.png").unwrap();
                        f.write_all(&decoded).unwrap();
                        println!("Image written to `image.png`");
                        println!("It is now safe to terminate this program");
                        return SUCCESS;
                    }
                }
            }
        }
    }

    FAILURE
}

fn do_the_thing(window_proxy: WindowProxy) -> Result<()> {
    println!("Doing the thing");
    window_proxy.evaluate_script(HTML2CANVAS)?;
    window_proxy.evaluate_script(TAKE_SCREENSHOT)?;

    Ok(())
}
