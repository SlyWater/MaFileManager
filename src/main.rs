#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Language {
    #[default]
    English,
    Russian,
}

impl Language {
    fn from_system_locale() -> Self {
        let locale = sys_locale::get_locale().unwrap_or_default();
        if locale.starts_with("ru") {
            Language::Russian
        } else {
            Language::English
        }
    }

    fn label(self, key: &str) -> String {
        match (self, key) {
            // UI labels
            (_, "source_folder") => if self == Language::Russian { "Папка с файлами:".to_string() } else { "Source folder:".to_string() },
            (_, "output_folder") => if self == Language::Russian { "Папка вывода:".to_string() } else { "Output folder:".to_string() },
            (_, "browse") => if self == Language::Russian { "Обзор".to_string() } else { "Browse".to_string() },
            (_, "trim_mafile") => if self == Language::Russian { "Урезать maFile".to_string() } else { "Trim maFile".to_string() },
            (_, "add_identity") => if self == Language::Russian { "Добавить identity_secret".to_string() } else { "Add identity_secret".to_string() },
            (_, "mode") => if self == Language::Russian { "Режим:".to_string() } else { "Mode:".to_string() },
            (_, "mode_0") => if self == Language::Russian { "Без переименования".to_string() } else { "No renaming".to_string() },
            (_, "mode_1") => if self == Language::Russian { "Переименовать в SteamID".to_string() } else { "Rename to SteamID".to_string() },
            (_, "mode_2") => if self == Language::Russian { "Переименовать в login".to_string() } else { "Rename to login".to_string() },
            (_, "process") => if self == Language::Russian { "Обработать".to_string() } else { "Process".to_string() },
            (_, "specify_source") => if self == Language::Russian { "Укажите папку источника!".to_string() } else { "Please specify source folder!".to_string() },
            (_, "language") => if self == Language::Russian { "Язык:".to_string() } else { "Language:".to_string() },
            (_, "lang_en") => "English".to_string(),
            (_, "lang_ru") => "Русский".to_string(),
            
            // Log messages
            (_, "error_creating_folder") => if self == Language::Russian { "Ошибка создания папки: ".to_string() } else { "Error creating folder: ".to_string() },
            (_, "skipped_no_data") => if self == Language::Russian { "Пропущен файл ".to_string() } else { "Skipped file ".to_string() },
            (_, "no_data") => if self == Language::Russian { ": нет данных".to_string() } else { ": no data".to_string() },
            (_, "skipped_no_steamid") => if self == Language::Russian { ": нет SteamID для переименования".to_string() } else { ": no SteamID for renaming".to_string() },
            (_, "skipped_no_login") => if self == Language::Russian { ": нет login для переименования".to_string() } else { ": no login for renaming".to_string() },
            (_, "failed_create_accounts") => if self == Language::Russian { "Не удалось создать _accounts.txt: ".to_string() } else { "Failed to create _accounts.txt: ".to_string() },
            (_, "error_write_accounts") => if self == Language::Russian { "Ошибка записи в _accounts.txt: ".to_string() } else { "Error writing to _accounts.txt: ".to_string() },
            (_, "file_processed") => if self == Language::Russian { "Файл обработан: ".to_string() } else { "File processed: ".to_string() },
            (_, "error_write_file") => if self == Language::Russian { "Ошибка записи файла ".to_string() } else { "Error writing file ".to_string() },
            (_, "json_parse_error") => if self == Language::Russian { "Ошибка парсинга JSON ".to_string() } else { "JSON parsing error ".to_string() },
            (_, "file_read_error") => if self == Language::Russian { "Ошибка чтения файла ".to_string() } else { "File read error ".to_string() },
            (_, "folder_read_error") => if self == Language::Russian { "Ошибка чтения папки: ".to_string() } else { "Folder read error: ".to_string() },
            (_, "processing_complete_created") => if self == Language::Russian { "Обработка завершена! _accounts.txt создан в ".to_string() } else { "Processing complete! _accounts.txt created in ".to_string() },
            (_, "processing_complete_no_files") => if self == Language::Russian { "Обработка завершена! Файлы не найдены или не обработаны.".to_string() } else { "Processing complete! No files found or processed.".to_string() },
            
            _ => key.to_string(),
        }
    }
}

fn main() -> eframe::Result<()> {
    // Загружаем иконку для панели задач
    let icon = load_icon();

    // Получаем путь к папке с exe-файлом для диалога
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();

    // Определяем язык системы
    let system_language = Language::from_system_locale();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_icon(icon)
            .with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "MaFileManager",
        options,
        Box::new(move |_cc| Box::new(MaFileManagerApp::new(exe_dir, system_language))),
    )
}

fn load_icon() -> egui::IconData {
    // Иконка встроена в exe через winres, но для egui нужно загрузить отдельно
    let rgba = include_bytes!("../resources/icon.png");
    let decoder = png::Decoder::new(std::io::Cursor::new(rgba));
    let mut reader = decoder.read_info().expect("Failed to read PNG");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("Failed to decode PNG");

    egui::IconData {
        rgba: buf,
        width: info.width,
        height: info.height,
    }
}

#[derive(Default)]
struct MaFileManagerApp {
    source_folder: String,
    output_folder: String,
    mode_index: usize,
    trim_mafile: bool,
    add_identity_secret: bool,
    log: Vec<String>,
    exe_dir: std::path::PathBuf,
    language: Language,
}

impl MaFileManagerApp {
    fn new(exe_dir: std::path::PathBuf, language: Language) -> Self {
        Self {
            exe_dir,
            language,
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize)]
struct SessionData {
    steamid: Option<u64>,
}

#[derive(Deserialize)]
struct FileData {
    account_name: Option<String>,
    shared_secret: Option<String>,
    steamid: Option<u64>,
    session: Option<SessionData>,
    identity_secret: Option<String>,
}

#[derive(Serialize)]
struct TrimmedFileData {
    account_name: String,
    shared_secret: String,
    steamid: u64,
    session: SessionData,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_secret: Option<String>,
}

impl eframe::App for MaFileManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Language selector
                ui.horizontal(|ui| {
                    ui.label(self.language.label("language"));
                    egui::ComboBox::from_id_source("language_selector")
                        .selected_text(match self.language {
                            Language::English => self.language.label("lang_en"),
                            Language::Russian => self.language.label("lang_ru"),
                        })
                        .show_ui(ui, |ui| {
                            if ui.selectable_label(self.language == Language::English, self.language.label("lang_en")).clicked() {
                                self.language = Language::English;
                            }
                            if ui.selectable_label(self.language == Language::Russian, self.language.label("lang_ru")).clicked() {
                                self.language = Language::Russian;
                            }
                        });
                });

                ui.add_space(5.0);

                // Source folder
                ui.horizontal(|ui| {
                    ui.label(self.language.label("source_folder"));
                    if ui.button(self.language.label("browse")).clicked() {
                        if let Some(path) = FileDialog::new()
                            .set_directory(&self.exe_dir)
                            .pick_folder()
                        {
                            self.source_folder = path.display().to_string();
                        }
                    }
                });
                ui.add(
                    egui::TextEdit::singleline(&mut self.source_folder)
                        .desired_width(f32::INFINITY),
                );

                ui.add_space(5.0);

                // Output folder
                ui.horizontal(|ui| {
                    ui.label(self.language.label("output_folder"));
                    if ui.button(self.language.label("browse")).clicked() {
                        if let Some(path) = FileDialog::new()
                            .set_directory(&self.exe_dir)
                            .pick_folder()
                        {
                            self.output_folder = path.display().to_string();
                        }
                    }
                });
                ui.add(
                    egui::TextEdit::singleline(&mut self.output_folder)
                        .desired_width(f32::INFINITY),
                );

                ui.add_space(10.0);

                // Checkboxes
                ui.checkbox(&mut self.trim_mafile, self.language.label("trim_mafile"));
                ui.checkbox(&mut self.add_identity_secret, self.language.label("add_identity"));

                ui.add_space(10.0);

                // Rename mode
                ui.horizontal(|ui| {
                    ui.label(self.language.label("mode"));
                    egui::ComboBox::from_id_source("mode_selector")
                        .selected_text(match self.mode_index {
                            0 => self.language.label("mode_0"),
                            1 => self.language.label("mode_1"),
                            2 => self.language.label("mode_2"),
                            _ => "".to_string(),
                        })
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(self.mode_index == 0, self.language.label("mode_0"))
                                .clicked()
                            {
                                self.mode_index = 0;
                            }
                            if ui
                                .selectable_label(self.mode_index == 1, self.language.label("mode_1"))
                                .clicked()
                            {
                                self.mode_index = 1;
                            }
                            if ui
                                .selectable_label(self.mode_index == 2, self.language.label("mode_2"))
                                .clicked()
                            {
                                self.mode_index = 2;
                            }
                        });
                });

                ui.add_space(20.0);

                // Process button
                // Если source_folder пустой, используем exe_dir
                let src_path = if self.source_folder.is_empty() {
                    self.exe_dir.clone()
                } else {
                    Path::new(&self.source_folder).to_path_buf()
                };
                
                if ui
                    .button(self.language.label("process"))
                    .clicked()
                {
                    let src = src_path.as_path();
                    let out = if self.output_folder.is_empty() {
                        src.join("output")
                    } else {
                        Path::new(&self.output_folder).to_path_buf()
                    };

                    if !out.exists() {
                        if let Err(e) = fs::create_dir_all(&out) {
                            self.log.push(format!("{}{}", self.language.label("error_creating_folder"), e));
                            return;
                        }
                    }

                    let accounts_file_path = out.join("_accounts.txt");
                    let mut accounts_file: Option<File> = None;

                    match fs::read_dir(src) {
                        Ok(entries) => {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_file() {
                                    // Filter only .mafile files (case-insensitive)
                                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                                        if ext.to_lowercase() != "mafile" {
                                            continue;
                                        }
                                    } else {
                                        continue;
                                    }

                                    match fs::read_to_string(&path) {
                                        Ok(content) => {
                                            match serde_json::from_str::<FileData>(&content) {
                                                Ok(data) => {
                                                    let account_name = data.account_name.clone().unwrap_or_default();
                                                    let shared_secret = data.shared_secret.clone().unwrap_or_default();
                                                    let steam_id = match data.steamid {
                                                        Some(id) if id != 0 => id,
                                                        _ => data.session.as_ref().and_then(|s| s.steamid).unwrap_or(0),
                                                    };

                                                    // Skip files without steamid and account_name
                                                    if steam_id == 0 && account_name.is_empty() {
                                                        self.log.push(format!("{}{:?}{}", self.language.label("skipped_no_data"), path.file_name().unwrap(), self.language.label("no_data")));
                                                        continue;
                                                    }

                                                    // Generate new filename
                                                    let new_name = match self.mode_index {
                                                        1 => {
                                                            if steam_id == 0 {
                                                                self.log.push(format!("{}{:?}{}", self.language.label("skipped_no_data"), path.file_name().unwrap(), self.language.label("skipped_no_steamid")));
                                                                continue;
                                                            }
                                                            format!("{}.mafile", steam_id)
                                                        }
                                                        2 => {
                                                            if account_name.is_empty() {
                                                                self.log.push(format!("{}{:?}{}", self.language.label("skipped_no_data"), path.file_name().unwrap(), self.language.label("skipped_no_login")));
                                                                continue;
                                                            }
                                                            format!("{}.mafile", account_name)
                                                        }
                                                        _ => path.file_name().unwrap().to_string_lossy().to_string(),
                                                    };

                                                    // Create _accounts.txt only on first successful file
                                                    if accounts_file.is_none() {
                                                        match File::create(&accounts_file_path) {
                                                            Ok(f) => accounts_file = Some(f),
                                                            Err(e) => {
                                                                self.log.push(format!("{}{}", self.language.label("failed_create_accounts"), e));
                                                                return;
                                                            }
                                                        }
                                                    }

                                                    // Generate file content
                                                    let new_content = if self.trim_mafile {
                                                        let trimmed = TrimmedFileData {
                                                            account_name: account_name.clone(),
                                                            shared_secret,
                                                            steamid: steam_id,
                                                            session: SessionData { steamid: Some(steam_id) },
                                                            identity_secret: if self.add_identity_secret {
                                                                data.identity_secret.clone()
                                                            } else {
                                                                None
                                                            },
                                                        };
                                                        serde_json::to_string_pretty(&trimmed).unwrap_or_default()
                                                    } else {
                                                        content.clone()
                                                    };

                                                    let new_path = out.join(new_name);

                                                    // Save file
                                                    match File::create(&new_path)
                                                        .and_then(|mut f| f.write_all(new_content.as_bytes()))
                                                    {
                                                        Ok(_) => {
                                                            if let Some(ref mut f) = accounts_file {
                                                                if let Err(e) = writeln!(f, "{} {}", account_name, steam_id) {
                                                                    self.log.push(format!("{}{}", self.language.label("error_write_accounts"), e));
                                                                } else {
                                                                    self.log.push(format!("{}{:?}", self.language.label("file_processed"), path.file_name().unwrap()));
                                                                }
                                                            }
                                                        }
                                                        Err(e) => {
                                                            self.log.push(format!("{}{:?}{}", self.language.label("error_write_file"), new_path.file_name().unwrap(), e));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    self.log.push(format!("{}{:?}: {}", self.language.label("json_parse_error"), path.file_name().unwrap(), e));
                                                }
                                            }
                                        }
                                        Err(e) => self.log.push(format!("{}{:?}: {}", self.language.label("file_read_error"), path.file_name().unwrap(), e)),
                                    }
                                }
                            }
                        }
                        Err(e) => self.log.push(format!("{}{}", self.language.label("folder_read_error"), e)),
                    }

                    if accounts_file.is_some() {
                        self.log.push(format!("{}{:?}", self.language.label("processing_complete_created"), out));
                    } else {
                        self.log.push(self.language.label("processing_complete_no_files").to_string());
                    }
                }

                ui.add_space(10.0);

                // Log
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for line in &self.log {
                                ui.label(line);
                            }

                            // Scroll to bottom
                            if !self.log.is_empty() {
                                ui.scroll_to_cursor(Option::from(egui::Align::BOTTOM));
                            }
                        });
                    });


            });
        });
    }
}
