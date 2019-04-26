// Config
extern crate config;
extern crate notify;
extern crate serde;
extern crate serde_derive;

use std::sync::RwLock;

use config::*;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings
            .merge(config::File::with_name("Settings.toml"))
            .unwrap();

        settings
    });
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: String,
    pub ttl: u32,
    pub root_folder: String,
    pub list_directories: bool,
    pub workers_per_thread: usize,
}

#[derive(Debug, Deserialize)]
pub struct Timeouts {
    pub request_seconds: u64,
    pub request_miliseconds: u32,
}

impl Timeouts {
    pub fn get_nanoseconds(&self) -> u32 {
        self.request_miliseconds * 1000000
    }
}

#[derive(Debug, Deserialize)]
pub struct Debug {
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: Debug,
    pub server: Server,
    pub timeout: Timeouts,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = config::Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(config::File::with_name("Settings.toml"))?;

        // You may also programmatically change settings
        //s.set("database.url", "postgres://")?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }

    pub fn new_unwrap() -> Self {
        Self::new().unwrap()
    }

    pub fn show(&self) {
        println!("\n\x1b[31m{:?}\x1b[0m", self);
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
