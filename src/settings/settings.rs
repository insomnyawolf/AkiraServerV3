/// # Config Manager
extern crate config;
// extern crate notify;
extern crate serde;
extern crate serde_derive;

use crate::settings::debug::*;
use crate::settings::server::*;
use crate::settings::timeouts::*;

use crate::utils;
use std::result::Result;
use termcolor::Color;

#[derive(Debug, Deserialize)]
/// Contains all the settings fragments
pub struct Settings {
    pub server: Server,
    pub timeout: Timeouts,
    pub debug: Debug,
}

impl Settings {
    /// Load Config From The Specified File
    ///
    /// Currently ```"Settings.toml"```
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(config::File::with_name("Settings.toml"))?;

        // You may also programmatically change settings
        //s.set("database.url", "postgres://")?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }

    /// Prints Current Config to stdout
    pub fn show(&self) {
        utils::log::log(self, Color::Magenta);
    }

    // ToDo funciona con la implementacion actual, WIP
    /*
    pub fn watch(&self) {
        // Watch for settings change
        thread::spawn(|| {
            // Create a channel to receive the events.
            let (tx, rx) = channel();

            // Automatically select the best implementation for your platform.
            // You can also access each implementation directly e.g. INotifyWatcher.
            let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            watcher
                .watch("Settings.toml", RecursiveMode::NonRecursive)
                .unwrap();

            // This is a simple loop, but you may want to use more complex logic here,
            // for example to handle I/O.
            loop {
                match rx.recv() {
                    Ok(DebouncedEvent::Write(_)) => {
                        println!(" * Settings.toml written; consider restarting the program...");
                        //Self = Settings::new();
                        //refresh();
                    }

                    Err(e) => println!("watch error: {:?}", e),

                    _ => {
                        // Ignore event
                    }
                }
            }
        });
    }*/
}
