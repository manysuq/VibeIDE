# Ultralight Code

Легковесный аналог VS Code на Rust + Ultralight с поддержкой плагинов VS Code.

## 🚀 Быстрый старт

### Шаг 1: Установка Rust

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
1. Скачайте установщик с https://rustup.rs/
2. Запустите `rustup-init.exe`
3. Следуйте инструкциям

Проверьте установку:
```bash
rustc --version
cargo --version
```

### Шаг 2: Установка системных зависимостей

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y cmake libgtk-3-dev libwebkit2gtk-4.0-dev libssl-dev pkg-config build-essential
```

**Fedora/RHEL:**
```bash
sudo dnf install cmake gtk3-devel webkit2gtk3-devel openssl-devel pkg-config gcc
```

**macOS:**
```bash
brew install cmake
```

**Windows:**
1. Установите [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Выберите "C++ build tools"
3. Установите [CMake](https://cmake.org/download/)

### Шаг 3: Клонирование проекта

```bash
git clone <repository-url> ultralight_code
cd ultralight_code
```

### Шаг 4: Сборка и запуск

**Debug режим (для разработки):**
```bash
cargo build
cargo run
```

**Release режим (оптимизированная версия):**
```bash
cargo build --release
./target/release/ultralight_code  # Linux/macOS
.\target\release\ultralight_code.exe  # Windows
```

## 📦 Структура проекта

```
ultralight_code/
├── src/                    # Исходный код на Rust
│   ├── main.rs            # Точка входа
│   ├── app.rs             # Управление приложением
│   ├── window.rs          # Менеджер окон
│   ├── renderer.rs        # Ultralight рендерер
│   ├── plugin.rs          # Система плагинов
│   ├── fs.rs              # Файловая система
│   └── config.rs          # Конфигурация
├── web/ui/                # Frontend часть
│   ├── index.html         # UI разметка
│   ├── styles/main.css    # Стили (тёмная тема VS Code)
│   └── js/app.js          # Логика интерфейса
├── docs/                  # Документация
├── Cargo.toml            # Rust зависимости
└── README.md             # Этот файл
```

## 🔌 Поддержка плагинов VS Code

### Как это работает

Плагины VS Code **НЕ требуют конвертации в WebAssembly**! Они работают напрямую через QuickJS runtime:

1. **Парсинг .vsix файлов** — извлечение package.json и кода
2. **VS Code API совместимость** — shim слой для ключевых API
3. **Изолированное выполнение** — каждый плагин в отдельном контексте

### Пример плагина

```javascript
// Работает без изменений из VS Code
const vscode = require('vscode');

function activate(context) {
    let disposable = vscode.commands.registerCommand(
        'extension.helloWorld', 
        () => {
            vscode.window.showInformationMessage('Hello World!');
        }
    );
    context.subscriptions.push(disposable);
}

module.exports = { activate };
```

### Совместимость

| Функция | Статус | Примечания |
|---------|--------|------------|
| Commands | ✅ | Полная поддержка |
| Menus | ✅ | Полная поддержка |
| Languages | ✅ | Синтаксис, completion |
| Debuggers | 🟡 | Частично (DAP protocol) |
| Webviews | 🟡 | Через Ultralight |
| Native modules | ❌ | Требуют пересборки |

## 🎯 Целевые показатели производительности

| Метрика | Цель |
|---------|------|
| Время запуска | <1 секунды |
| Потребление памяти (idle) | <100 MB |
| Потребление памяти (с плагинами) | <256 MB |
| Время до первого рендера | <500 ms |

## 🛠️ Разработка

### Сборка в режиме разработки

```bash
# Быстрая сборка с дебаг символами
cargo build

# Запуск с логированием
RUST_LOG=debug cargo run

# Запуск с hot-reload (feature dev)
cargo run --features dev
```

### Запуск тестов

```bash
cargo test
```

### Форматирование кода

```bash
cargo fmt
cargo clippy
```

## ⚙️ Конфигурация

Конфигурационный файл создаётся автоматически при первом запуске:

**Linux:** `~/.config/ultralight_code/config.json`
**macOS:** `~/Library/Application Support/ultralight_code/config.json`
**Windows:** `%APPDATA%\ultralight_code\config.json`

Пример конфигурации:
```json
{
  "theme": "dark",
  "editor": {
    "fontSize": 14,
    "fontFamily": "Consolas, 'Courier New', monospace",
    "lineNumbers": true,
    "minimap": true
  },
  "plugins": {
    "enabled": true,
    "autoUpdate": false
  }
}
```

## 🔧 Решение проблем

### Ошибка: "Ultralight SDK not found"

Ultralight SDK требуется для рендеринга. Скачайте с https://ultralight.dev/ и следуйте инструкциям по установке.

### Ошибка компиляции на Linux

Убедитесь, что установлены все зависимости:
```bash
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libssl-dev
```

### Ошибка на Windows

Убедитесь, что установлены Visual Studio Build Tools с поддержкой C++.

### Медленная сборка

Используйте release режим только для финальной сборки:
```bash
cargo build --release
```

Для разработки используйте debug режим — он быстрее компилируется.

## 📚 Документация

- [Архитектура проекта](docs/ARCHITECTURE.md)
- [API плагинов](docs/PLUGIN_API.md) (в разработке)
- [Руководство по внесению вклада](docs/CONTRIBUTING.md) (в разработке)

## 🤝 Вклад в проект

Приветствуются:
- 🐛 Отчёты об ошибках
- 💡 Предложения новых функций
- 📝 Улучшение документации
- 👨‍💻 Pull requests с кодом

См. [ARCHITECTURE.md](docs/ARCHITECTURE.md) для деталей.

## 📄 Лицензия

MIT License

---

**Ultralight Code** — создаётся с ❤️ для сообщества разработчиков
