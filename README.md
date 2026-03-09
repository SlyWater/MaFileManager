# MaFileManager

**English version** | [Русская версия](#ru)

---

## 📋 Description

MaFileManager is a graphical utility for managing Steam Guard `.mafile` files. It allows you to process, rename, and organize your Steam account authentication files.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

---

## ✨ Features

- 📁 **Browse and select folders** - Easy folder selection for source and output
- 🔄 **Multiple rename modes**:
  - No renaming (keep original names)
  - Rename to SteamID
  - Rename to login (account name)
- ✂️ **Trim maFile** - Reduce file size by removing unnecessary data
- 🔐 **Add identity_secret** - Optionally add identity_secret to trimmed files
- 📝 **Auto-generate accounts.txt** - Creates a list of processed accounts
- 🌐 **Bilingual interface** - English and Russian with auto-detection


---

## 🚀 Installation

### Pre-built Binary

Download the latest release from the [Releases](https://github.com/yourusername/MaFileManager/releases) page.

### Build from Source

**Requirements:**
- Rust 1.70 or later
- Windows 10/11

**Steps:**

```bash
# Clone the repository
git clone https://github.com/SlyWater/MaFileManager.git
cd MaFileManager

# Build release version
cargo build --release
```

The executable will be located at `target\release\MaFileManager.exe`.

---

## 📖 Usage

1. **Launch** `MaFileManager.exe`
2. **Select source folder** - Click "Browse" and choose the folder containing `.mafile` files
3. **Select output folder** (optional) - Leave empty to create output in source folder's `output` subfolder
4. **Choose options**:
   - ☑️ Trim maFile - Remove unnecessary data from files
   - ☑️ Add identity_secret - Include identity_secret in trimmed files
5. **Select rename mode**:
   - No renaming - Keep original filenames
   - Rename to SteamID - Rename files to `<SteamID>.mafile`
   - Rename to login - Rename files to `<account_name>.mafile`
6. **Click "Process"** to start

### Output

- Processed `.mafile` files in the output folder
- `accounts.txt` - List of accounts in format: `<account_name> <steamid>`

---

## 🌐 Language Support

The application automatically detects your system language and uses:
- **Russian** (Русский) - for Russian locale
- **English** - for all other locales

You can manually switch languages using the dropdown at the top of the window.

---

## 📁 File Structure

```
MaFileManager/
├── src/
│   └── main.rs          # Main application code
├── resources/
│   ├── icon.ico         # Application icon (Windows)
│   └── icon.png         # Icon for taskbar
├── Cargo.toml           # Rust dependencies
├── build.rs             # Windows resources builder
├── LICENSE              # MIT License
└── README.md            # This file
```

---

## 🛠️ Technical Details

- **Framework**: eframe/egui (immediate mode GUI)
- **Language**: Rust
- **Platform**: Windows
- **Dependencies**:
  - `eframe` - GUI framework
  - `rfd` - Native file dialogs
  - `serde` + `serde_json` - JSON serialization
  - `sys-locale` - System locale detection
  - `png` - Icon loading
  - `winres` - Windows resource compiler

---

## 📝 License

MIT License - See LICENSE file for details.

---

## ⚠️ Disclaimer

This tool is intended for personal use only. Make sure you have the right to access and modify the `.mafile` files. The author is not responsible for any misuse or account losses.

---

<a name="ru"></a>
# MaFileManager

[English version](#english) | **Русская версия**

---

## 📋 Описание

MaFileManager — это графическая утилита для управления файлами Steam Guard `.mafile`. Позволяет обрабатывать, переименовывать и организовывать файлы аутентификации Steam-аккаунтов.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

---

## ✨ Возможности

- 📁 **Выбор папок** - Удобный выбор папок источника и вывода
- 🔄 **Несколько режимов переименования**:
  - Без переименования (сохранить оригинальные имена)
  - Переименовать в SteamID
  - Переименовать в login (имя аккаунта)
- ✂️ **Уменьшение maFile** - Удаление лишних данных из файлов
- 🔐 **Добавить identity_secret** - Опциональное добавление identity_secret
- 📝 **Автогенерация accounts.txt** - Создание списка обработанных аккаунтов
- 🌐 **Двуязычный интерфейс** - Английский и русский с автоопределением

---

## 🚀 Установка

### Готовый бинарник

Скачайте последнюю версию со страницы [Releases](https://github.com/yourusername/MaFileManager/releases).

### Сборка из исходников

**Требования:**
- Rust 1.70 или новее
- Windows 10/11

**Шаги:**

```bash
# Клонировать репозиторий
git clone https://github.com/SlyWater/MaFileManager.git
cd MaFileManager

# Собрать релизную версию
cargo build --release
```

Исполняемый файл будет находиться в `target\release\MaFileManager.exe`.

---

## 📖 Использование

1. **Запустите** `MaFileManager.exe`
2. **Выберите папку источника** - Нажмите "Обзор" и выберите папку с `.mafile` файлами
3. **Выберите папку вывода** (опционально) - Оставьте пустой для создания в подпапке `output`
4. **Настройте опции**:
  - ☑️ Урезать maFile - Удалить лишние данные из файлов
  - ☑️ Добавить identity_secret - Включить identity_secret в обрезанные файлы
5. **Выберите режим переименования**:
  - Без переименования - Сохранить оригинальные имена
  - Переименовать в SteamID - Переименовать файлы в `<SteamID>.mafile`
  - Переименовать в login - Переименовать файлы в `<account_name>.mafile`
6. **Нажмите "Обработать"** для начала обработки

### Результат

- Обработанные `.mafile` файлы в папке вывода
- `accounts.txt` - Список аккаунтов в формате: `<account_name> <steamid>`

---

## 🌐 Поддержка языков

Приложение автоматически определяет язык вашей системы и использует:
- **Русский** - для русской локали
- **English** - для всех остальных

Вы можете вручную переключить язык через выпадающий список в верхней части окна.

---

## 📁 Структура файлов

```
MaFileManager/
├── src/
│   └── main.rs          # Основной код приложения
├── resources/
│   ├── icon.ico         # Иконка приложения (Windows)
│   └── icon.png         # Иконка для панели задач
├── Cargo.toml           # Зависимости Rust
├── build.rs             # Билдер Windows ресурсов
├── LICENSE              # MIT License
└── README.md            # Этот файл
```

---

## 🛠️ Технические детали

- **Фреймворк**: eframe/egui (immediate mode GUI)
- **Язык**: Rust
- **Платформа**: Windows
- **Зависимости**:
  - `eframe` - GUI фреймворк
  - `rfd` - Нативные диалоги файлов
  - `serde` + `serde_json` - Сериализация JSON
  - `sys-locale` - Определение локали системы
  - `png` - Загрузка иконки
  - `winres` - Компилятор ресурсов Windows

---

## 📝 Лицензия

MIT License - См. файл LICENSE для деталей.

---

## ⚠️ Отказ от ответственности

Этот инструмент предназначен только для личного использования. Убедитесь, что вы имеете право на доступ и модификацию файлов `.mafile`. Автор не несёт ответственности за неправильное использование или потерю аккаунтов.
