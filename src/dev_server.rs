use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{mpsc::channel, Arc, Mutex},
    time::Instant,
};

#[derive(Deserialize)]
struct ProjectConfig {
    name: String,
    target_platforms: Vec<Platform>,
    #[serde(default)]
    build_command: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BuildStatus {
    pub in_progress: bool,
    pub last_build: Option<Instant>,
    pub error: Option<String>,
}

pub struct DevServer {
    watcher: RecommendedWatcher,
    target_platform: Platform,
    status: Arc<Mutex<BuildStatus>>,
    simulator_windows: Vec<SimulatorWindow>,
}

#[derive(Clone, Debug)]
pub enum Platform {
    Desktop,
    IOS,
    Android,
    Web,
}

#[allow(dead_code)]
pub struct SimulatorWindow {
    platform: Platform,
    process: Option<std::process::Child>,
}

impl SimulatorWindow {
    fn new(platform: Platform) -> Self {
        Self {
            platform,
            process: None,
        }
    }
}

impl DevServer {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                tx.send(event).unwrap();
            }
        })
        .unwrap();

        Self {
            watcher,
            target_platform: Platform::Desktop,
            status: Arc::new(Mutex::new(BuildStatus {
                in_progress: false,
                last_build: None,
                error: None,
            })),
            simulator_windows: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crossterm::{
            event::{self, Event, KeyCode},
            terminal::{disable_raw_mode, enable_raw_mode},
        };

        let config = self.load_project_config()?;
        println!("Starting development server for: {}", config.name);

        enable_raw_mode()?;
        loop {
            self.render_ui()?;

            match event::read()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('1') => {
                        if config.target_platforms.contains(&Platform::Desktop) {
                            self.set_platform(Platform::Desktop);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn load_project_config(&self) -> Result<ProjectConfig, Box<dyn std::error::Error>> {
        let config_path = Path::new("rust-native.toml");
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            // Default config
            Ok(ProjectConfig {
                name: self.detect_project_name()?,
                target_platforms: vec![Platform::Desktop],
                build_command: None,
            })
        }
    }

    fn detect_project_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let cargo_toml = fs::read_to_string("Cargo.toml")?;
        let cargo_config: toml::Value = toml::from_str(&cargo_toml)?;
        Ok(cargo_config["package"]["name"]
            .as_str()
            .unwrap_or("rust-native-project")
            .to_string())
    }

    pub fn watch(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn set_platform(&mut self, platform: Platform) {
        self.target_platform = platform;
        self.rebuild();
    }

    pub fn rebuild(&mut self) {
        let mut status = self.status.lock().unwrap();
        status.in_progress = true;
        status.error = None;
        drop(status);

        let result = match self.target_platform {
            Platform::Desktop => self.start_desktop_build(),
            Platform::IOS => self.start_ios_simulator(),
            Platform::Android => self.start_android_emulator(),
            Platform::Web => self.start_web_server(),
        };

        let mut status = self.status.lock().unwrap();
        status.in_progress = false;
        status.last_build = Some(Instant::now());
        if let Err(e) = result {
            status.error = Some(e.to_string());
        }
    }

    fn start_desktop_build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.load_project_config()?;
        let command = config.build_command.unwrap_or_else(|| String::from("cargo run"));
        
        let args: Vec<&str> = command.split_whitespace().collect();
        let process = Command::new(args[0])
            .args(&args[1..])
            .spawn()?;

        let mut window = SimulatorWindow::new(Platform::Desktop);
        window.process = Some(process);
        self.simulator_windows.push(window);
        Ok(())
    }

    #[allow(unused_variables)]
    fn start_ios_simulator(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder for iOS implementation
        Ok(())
    }

    #[allow(unused_variables)]
    fn start_android_emulator(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder for Android implementation
        Ok(())
    }

    #[allow(unused_variables)]
    fn start_web_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder for Web implementation
        Ok(())
    }

    pub fn get_status(&self) -> BuildStatus {
        self.status.lock().unwrap().clone()
    }
}
