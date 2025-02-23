use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
    process::Command,
    sync::{mpsc::{channel, Sender}, Arc, Mutex},
    time::{Instant, Duration},
    io::stdout,
    thread,
};
use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};

#[derive(Deserialize)]
struct ProjectConfig {
    name: String,
    target_platforms: Vec<Platform>,
    #[serde(default)]
    build_command: Option<String>,
    #[serde(default)]
    ios_config: IOSConfig,
    #[serde(default)]
    android_config: AndroidConfig,
    #[serde(default)]
    web_config: WebConfig,
}

#[derive(Deserialize, Default)]
struct IOSConfig {
    device_name: Option<String>,
    ios_version: Option<String>,
    #[serde(skip)]
    _runtime: Option<String>, // Changed to _runtime to indicate intentionally unused
}

#[derive(Deserialize, Default)]
struct AndroidConfig {
    avd_name: Option<String>,
    api_level: Option<u32>,
    abi: Option<String>,
}

#[derive(Deserialize, Default)]
struct WebConfig {
    port: Option<u16>,
    browsers: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Platform {
    Desktop,
    IOS,
    Android,
    Web,
}

#[derive(Clone, Debug)]
pub struct BuildStatus {
    pub in_progress: bool,
    pub last_build: Option<Instant>,
    pub error: Option<String>,
    pub progress: Option<(u32, u32)>, // (current, total)
    pub message: Option<String>,
}

pub struct DevServer {
    watcher: Option<RecommendedWatcher>,
    watcher_tx: Option<Sender<notify::Event>>,
    target_platform: Platform,
    status: Arc<Mutex<BuildStatus>>,
    simulator_windows: Vec<SimulatorWindow>,
    file_watcher_thread: Option<std::thread::JoinHandle<()>>,
    ignore_paths: Vec<String>,
}

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

    fn set_process(&mut self, process: std::process::Child) {
        self.process = Some(process);
    }

    fn kill_process(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
        }
    }
}

impl DevServer {
    pub fn new() -> Self {
        Self {
            watcher: None,
            watcher_tx: None,
            target_platform: Platform::Desktop,
            status: Arc::new(Mutex::new(BuildStatus {
                in_progress: false,
                last_build: None,
                error: None,
                progress: None,
                message: None,
            })),
            simulator_windows: Vec::new(),
            file_watcher_thread: None,
            ignore_paths: vec![
                "target".to_string(),
                ".git".to_string(),
            ],
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crossterm::{
            event::{self, Event, KeyCode},
            terminal::{disable_raw_mode, enable_raw_mode},
        };

        enable_raw_mode()?;
        
        // Show welcome screen first
        self.render_welcome_screen()?;
        
        loop {
            self.render_ui()?;

            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('1') => {
                        self.set_platform(Platform::Desktop);
                        self.show_platform_setup("Desktop")?;
                    },
                    KeyCode::Char('2') => {
                        self.set_platform(Platform::IOS);
                        self.show_platform_setup("iOS")?;
                    },
                    KeyCode::Char('3') => {
                        self.set_platform(Platform::Android);
                        self.show_platform_setup("Android")?;
                    },
                    KeyCode::Char('4') => {
                        self.set_platform(Platform::Web);
                        self.show_platform_setup("Web")?;
                    },
                    KeyCode::Char('r') => {
                        if self.is_platform_selected() {
                            self.rebuild();
                        }
                    },
                    KeyCode::Char('s') => {
                        if self.is_platform_selected() {
                            self.restart()?;
                        }
                    },
                    _ => {}
                }
            }
        }

        disable_raw_mode()?;
        Ok(())
    }

    fn is_platform_selected(&self) -> bool {
        !matches!(self.get_status().message, None)
    }

    fn show_platform_setup(&mut self, platform: &str) -> Result<(), Box<dyn std::error::Error>> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Cyan),
            Print(format!("🔧 Setting up {} Platform\n\n", platform)),
        )?;

        self.update_progress(0, 1, &format!("Preparing {} environment...", platform));
        thread::sleep(Duration::from_millis(500));

        Ok(())
    }

    fn render_welcome_screen(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Cyan),
            Print("\n\n"),
            Print("    ╔══════════════════════════════════╗\n"),
            Print("    ║      RustUI Development Hub      ║\n"),
            Print("    ╚══════════════════════════════════╝\n\n"),
            SetForegroundColor(Color::White),
            Print("    Choose your development platform:\n\n"),
            Print("    1. 🖥️  Desktop Application\n"),
            Print("    2. 📱 iOS Application\n"),
            Print("    3. 🤖 Android Application\n"),
            Print("    4. 🌐 Web Application\n\n"),
            SetForegroundColor(Color::Yellow),
            Print("    Controls:\n"),
            Print("    • Select platform (1-4)\n"),
            Print("    • r - Rebuild current platform\n"),
            Print("    • s - Restart development server\n"),
            Print("    • q - Quit\n\n"),
            SetForegroundColor(Color::Green),
            Print("    Ready to start development!\n"),
        )?;

        thread::sleep(Duration::from_secs(2));
        Ok(())
    }

    fn render_ui(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Cyan),
            Print("╔════════════════════════════════════════════╗\n"),
            Print("║           RustUI Development Hub           ║\n"),
            Print("╚════════════════════════════════════════════╝\n\n"),
        )?;

        // Show current platform with highlight
        let platforms = vec![
            (Platform::Desktop, "1. Desktop 🖥️ "),
            (Platform::IOS, "2. iOS 📱"),
            (Platform::Android, "3. Android 🤖"),
            (Platform::Web, "4. Web 🌐"),
        ];

        for (platform, label) in platforms {
            let color = if platform == self.target_platform {
                Color::Green
            } else {
                Color::White
            };
            execute!(
                stdout(),
                SetForegroundColor(color),
                Print(format!("{}{}\n", 
                    if platform == self.target_platform { "▶ " } else { "  " },
                    label
                )),
            )?;
        }

        // Show progress and status
        let status = self.get_status();
        if let Some((current, total)) = status.progress {
            let width = 40;
            let progress = (current as f32 / total as f32 * width as f32) as usize;
            let bar = format!(
                "▕{}{}▏",
                "█".repeat(progress),
                "░".repeat(width - progress),
            );

            execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                Print(format!("\n⏳ Progress: {}\n", bar)),
            )?;
        }

        // Show status message with emoji
        if let Some(msg) = &status.message {
            let (emoji, color) = if status.in_progress {
                ("🔄", Color::Yellow)
            } else if status.error.is_some() {
                ("❌", Color::Red)
            } else {
                ("✅", Color::Green)
            };

            execute!(
                stdout(),
                SetForegroundColor(color),
                Print(format!("\n{} Status: {}\n", emoji, msg)),
            )?;
        }

        // Show controls
        execute!(
            stdout(),
            SetForegroundColor(Color::White),
            Print("\n╔════════════════════════════════════════════╗\n"),
            Print("║ Controls:                                  ║\n"),
            Print("║ • 1-4: Switch Platform                    ║\n"),
            Print("║ • r: Rebuild   • s: Restart   • q: Quit   ║\n"),
            Print("╚════════════════════════════════════════════╝\n"),
        )?;

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
                ios_config: IOSConfig::default(),
                android_config: AndroidConfig::default(),
                web_config: WebConfig::default(),
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
        let (tx, rx) = channel();
        let watcher_tx = tx.clone();
        let ignore_paths = self.ignore_paths.clone();
        
        let status = self.status.clone();
        let handle = std::thread::spawn(move || {
            for event in rx {
                // Filter out events from ignored directories
                let notify::Event { paths, .. } = &event;
                if paths.iter().any(|p| {
                    ignore_paths.iter().any(|ignore| p.to_string_lossy().contains(ignore))
                }) {
                    continue;
                }

                if let Ok(mut status) = status.lock() {
                    status.in_progress = true;
                    println!("File changed: {:?}", paths);
                }
            }
        });

        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })?;

        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        self.watcher = Some(watcher);
        self.watcher_tx = Some(watcher_tx);
        self.file_watcher_thread = Some(handle);
        Ok(())
    }

    fn cleanup(&mut self) {
        if let Some(handle) = self.file_watcher_thread.take() {
            drop(self.watcher.take());
            drop(self.watcher_tx.take());
            let _ = handle.join();
        }
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

    fn update_progress(&self, current: u32, total: u32, message: &str) {
        if let Ok(mut status) = self.status.lock() {
            status.progress = Some((current, total));
            status.message = Some(message.to_string());
        }
    }

    fn start_desktop_build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_progress(0, 4, "Initializing desktop build...");
        self.cleanup_platform(Platform::Desktop);
        thread::sleep(Duration::from_millis(500));

        self.update_progress(1, 4, "Building application...");
        
        // Build and run the counter app
        let process = Command::new("cargo")
            .args(["run", "--example", "todo_app"])
            .spawn()?;

        self.update_progress(2, 4, "Starting application...");
        let mut window = SimulatorWindow::new(Platform::Desktop);
        window.set_process(process);
        self.simulator_windows.push(window);

        self.update_progress(4, 4, "Desktop application ready!");
        Ok(())
    }

    fn start_ios_simulator(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_progress(0, 5, "Building for iOS...");
        
        // Run iOS build script
        let status = Command::new("sh")
            .args(["scripts/build-ios.sh", "todo_app"])
            .status()?;

        if !status.success() {
            return Err("iOS build failed".into());
        }

        self.update_progress(2, 5, "Starting iOS simulator...");
        
        // Start simulator
        let simulator = Command::new("open")
            .arg("-a")
            .arg("Simulator")
            .spawn()?;

        let mut window = SimulatorWindow::new(Platform::IOS);
        window.set_process(simulator);
        self.simulator_windows.push(window);

        self.update_progress(5, 5, "iOS simulator ready!");
        Ok(())
    }

    fn start_android_emulator(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_progress(0, 5, "Building for Android...");
        
        // Run Android build script
        let status = Command::new("sh")
            .args(["scripts/build-android.sh", "todo_app"])
            .status()?;

        if !status.success() {
            return Err("Android build failed".into());
        }

        self.update_progress(2, 5, "Starting Android emulator...");
        
        let config = self.load_project_config()?;
        let avd_name = config.android_config.avd_name
            .unwrap_or_else(|| "Pixel_4".to_string());

        // Start emulator
        let emulator = Command::new("emulator")
            .args(["-avd", &avd_name])
            .spawn()?;

        let mut window = SimulatorWindow::new(Platform::Android);
        window.set_process(emulator);
        self.simulator_windows.push(window);

        self.update_progress(5, 5, "Android emulator ready!");
        Ok(())
    }

    fn start_web_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_progress(0, 5, "Building for web...");
        
        // Run web build script
        let status = Command::new("sh")
            .args(["scripts/build-web.sh"])
            .status()?;

        if !status.success() {
            return Err("Web build failed".into());
        }

        self.update_progress(2, 5, "Starting web server...");
        
        let config = self.load_project_config()?;
        let port = config.web_config.port.unwrap_or(8080);

        // Start Python HTTP server
        let server = Command::new("python3")
            .args(["-m", "http.server", &port.to_string()])
            .current_dir("www")
            .spawn()?;

        let mut window = SimulatorWindow::new(Platform::Web);
        window.set_process(server);
        self.simulator_windows.push(window);

        // Open browser
        self.open_default_browser(port)?;

        self.update_progress(5, 5, "Web server ready!");
        Ok(())
    }

    fn open_browser(&self, browser: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://localhost:{}", port);
        
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .args(["-a", browser, &url])
                .spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(&url)
                .spawn()?;
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/C", "start", &url])
                .spawn()?;
        }

        Ok(())
    }

    fn open_default_browser(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("http://localhost:{}", port);
        
        #[cfg(target_os = "macos")]
        Command::new("open").arg(&url).spawn()?;
        #[cfg(target_os = "linux")]
        Command::new("xdg-open").arg(&url).spawn()?;
        #[cfg(target_os = "windows")]
        Command::new("cmd").args(["/C", "start", &url]).spawn()?;

        Ok(())
    }

    fn cleanup_platform(&mut self, platform: Platform) {
        self.simulator_windows.retain_mut(|window| {
            if window.platform == platform {
                window.kill_process();
                false
            } else {
                true
            }
        });
    }

    pub fn get_status(&self) -> BuildStatus {
        self.status.lock().unwrap().clone()
    }

    fn restart(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.watcher = None;
        self.watcher_tx = None;
        self.simulator_windows.clear();
        self.watch(".")?;
        self.rebuild();
        Ok(())
    }

    // Add helper method for checking tool installations
    fn check_tool(&self, tool: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new(tool)
            .args(args)
            .output()
            .map_err(|_| format!("{} not found", tool))?;

        if !output.status.success() {
            return Err(format!("{} check failed", tool).into());
        }

        Ok(())
    }
}

impl Drop for DevServer {
    fn drop(&mut self) {
        for window in &mut self.simulator_windows {
            window.kill_process();
        }
        self.cleanup();
    }
}
