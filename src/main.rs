/// Entrypoint for mc-backup-tui

/*
 Copyright (c) 2026 RayZh

 Permission is hereby granted, free of charge, to any person obtaining a copy of
 this software and associated documentation files (the "Software"), to deal in
 the Software without restriction, including without limitation the rights to
 use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 the Software, and to permit persons to whom the Software is furnished to do so,
 subject to the following conditions:

 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

mod app;
mod config;
mod typealias;

use std::{panic::panic_any};

use typealias::Result;
use clap::{Arg, Command};
use app::App;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_CONFIG_PATH: &str = "config.toml";

fn main() -> Result<()> {
    // Obtain the configuration from file.
    let matches = Command::new("mc-backup-tui")
        .version(VERSION)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .required(false)
        )
        .get_matches();
    let config_path = matches.get_one::<String>("config")
        .map(String::as_str)
        .unwrap_or(DEFAULT_CONFIG_PATH);
    let config = config::Config::from_file(config_path).unwrap_or_else(|_| {
        // Check if the error is due to the file not existing. If not, panic.
        if std::fs::metadata(config_path).is_ok() {
            panic_any(format!("Failed to read config file at {}: {}", config_path, std::io::Error::last_os_error()));
        }
        eprintln!("Warning: No config file found at {}, creating a default one.", config_path);
        let default_config = config::Config::default();
        if let Err(e) = std::fs::write(config_path, toml::to_string_pretty(&default_config).unwrap()) {
            panic_any(format!("Failed to create default config file: {}", e));
        }
        default_config
    });

    // Start the main application loop.
    ratatui::run(|t| {
        let mut app = App::new(config);
        app.run(t)
    })
}
