use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io;
use crossterm::style::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorConfig {
    pub tab_size: usize,
    pub auto_indent: bool,
    pub line_numbers: bool,
    pub max_undo: usize,
    pub max_clipboard: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub background: String,
    pub foreground: String,
    pub selection: String,
    pub status_bar_bg: String,
    pub status_bar_fg: String,
    pub line_numbers: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    pub save: String,
    pub quit: String,
    pub copy: String,
    pub cut: String,
    pub paste: String,
    pub undo: String,
    pub redo: String,
    pub search: String,
    pub replace: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: EditorConfig::default(),
            theme: ThemeConfig::default(),
            keybindings: KeybindingsConfig::default(),
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        EditorConfig {
            tab_size: 4,
            auto_indent: true,
            line_numbers: true,
            max_undo: 100,
            max_clipboard: 10,
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            background: "#1E1E1E".to_string(),
            foreground: "#D4D4D4".to_string(),
            selection: "#264F78".to_string(),
            status_bar_bg: "#007ACC".to_string(),
            status_bar_fg: "#FFFFFF".to_string(),
            line_numbers: "#858585".to_string(),
        }
    }
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        KeybindingsConfig {
            save: "ctrl-s".to_string(),
            quit: "ctrl-q".to_string(),
            copy: "ctrl-c".to_string(),
            cut: "ctrl-x".to_string(),
            paste: "alt-v".to_string(),
            undo: "ctrl-z".to_string(),
            redo: "ctrl-y".to_string(),
            search: "ctrl-f".to_string(),
            replace: "ctrl-r".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(config_path)?;
        match toml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(_) => Ok(Config::default()),
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let config_path = Self::config_path()?;
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(config_path, content)
    }

    fn config_path() -> io::Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::NotFound,
                "Could not find config directory"
            ))?;
        path.push("text_editor");
        path.push("config.toml");
        Ok(path)
    }

    pub fn parse_color(&self, color_str: &str) -> Color {
        if let Some(hex) = color_str.strip_prefix('#') {
            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                let r = ((rgb >> 16) & 0xFF) as u8;
                let g = ((rgb >> 8) & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;
                return Color::Rgb { r, g, b };
            }
        }
        Color::Reset
    }
} 