# Ultralight Code - Архитектура и Руководство по Разработке

## Обзор Проекта

Ultralight Code — это легковесный аналог VS Code, построенный на связке Rust + Ultralight с поддержкой плагинов из экосистемы VS Code.

## Ключевые Принципы

1. **Легковесность** — минимальное потребление памяти (<100MB в простое)
2. **Быстрый запуск** — <1 секунды до готовности к работе
3. **Совместимость** — поддержка VS Code extensions без модификации
4. **Расширяемость** — модульная архитектура для легкого добавления функций

## Архитектура

### Слои Приложения

```
┌─────────────────────────────────────────────────────┐
│                    UI Layer                         │
│  (HTML/CSS/JS rendered by Ultralight)              │
├─────────────────────────────────────────────────────┤
│              VS Code Extension API                  │
│  (Compatibility layer for VS Code plugins)         │
├─────────────────────────────────────────────────────┤
│               Plugin Runtime                        │
│  (QuickJS - isolated JS execution)                 │
├─────────────────────────────────────────────────────┤
│                Rust Core                            │
│  - Window Management (winit)                       │
│  - File System                                      │
│  - IPC Bridge                                       │
│  - Ultralight Integration                          │
├─────────────────────────────────────────────────────┤
│            System Integration                       │
│  - Native file dialogs                             │
│  - System notifications                            │
│  - Clipboard                                        │
└─────────────────────────────────────────────────────┘
```

## Структура Проекта

```
ultralight_code/
├── src/
│   ├── main.rs          # Точка входа
│   ├── app.rs           # Управление приложением
│   ├── window.rs        # Управление окнами
│   ├── renderer.rs      # Ultralight рендерер
│   ├── plugin.rs        # Система плагинов
│   ├── fs.rs            # Файловая система
│   └── config.rs        # Конфигурация
├── web/
│   └── ui/
│       ├── index.html   # Основной HTML
│       ├── styles/
│       │   └── main.css # Стили
│       └── js/
│           └── app.js   # Frontend логика
├── docs/                # Документация
├── Cargo.toml           # Rust зависимости
└── README.md            # Этот файл
```

## Поддержка Плагинов VS Code

### Как Это Работает

1. **Парсинг .vsix файлов**
   - VSIX — это ZIP архив с package.json и кодом расширения
   - Извлекаем манифест и метаданные

2. **Совместимость API**
   - Реализуем ключевые части VS Code Extension API
   - `vscode` module shim для совместимости

3. **Изолированное Выполнение**
   - Каждый плагин работает в отдельном QuickJS контексте
   - Безопасное взаимодействие через IPC

### Конвертация Плагинов

Плагины VS Code **не требуют конвертации в WebAssembly**. Они работают напрямую:

```javascript
// Плагин VS Code работает без изменений
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

Наш runtime предоставляет совместимый API через QuickJS.

### Ограничения Совместимости

| Функция | Статус | Примечания |
|---------|--------|------------|
| Commands | ✅ | Полная поддержка |
| Menus | ✅ | Полная поддержка |
| Languages | ✅ | Синтаксис, completion |
| Debuggers | 🟡 | Частично (DAP protocol) |
| Webviews | 🟡 | Через Ultralight |
| Native modules | ❌ | Требуют пересборки |

## Сборка и Запуск

### Подробные инструкции по установке

См. основной [README.md](../README.md) для полных инструкций по установке.

### Краткая инструкция

```bash
# 1. Установите Rust (если ещё не установлен)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Установите системные зависимости
# Ubuntu/Debian:
sudo apt-get install cmake libgtk-3-dev libwebkit2gtk-4.0-dev libssl-dev pkg-config build-essential

# macOS:
brew install cmake

# 3. Клонируйте и соберите
git clone <repository-url> ultralight_code
cd ultralight_code
cargo build --release

# 4. Запустите
cargo run --release
```

### Примечание про Ultralight SDK

Для полноценной работы рендерера требуется Ultralight SDK. 
- Скачайте с https://ultralight.dev/
- Следуйте инструкциям по установке для вашей ОС
- В режиме разработки можно использовать заглушку (feature flag)

## Производительность

### Целевые Показатели

| Метрика | Цель | Текущее |
|---------|------|---------|
| Время запуска | <1s | TBD |
| Потребление памяти (idle) | <100MB | TBD |
| Потребление памяти (с плагинами) | <256MB | TBD |
| Время до первого рендера | <500ms | TBD |

### Оптимизации

- LTO (Link Time Optimization) включено в release
- Strip символов для уменьшения размера бинарника
- Lazy loading плагинов
- Виртуализация больших файлов

## Roadmap

### Phase 1: MVP (Q1 2025)
- [x] Базовая структура проекта
- [ ] Интеграция Ultralight
- [ ] Открытие/сохранение файлов
- [ ] Базовая подсветка синтаксиса

### Phase 2: Editor Features (Q2 2025)
- [ ] Multiple cursors
- [ ] Find & Replace
- [ ] Minimap
- [ ] Code folding

### Phase 3: Plugin System (Q3 2025)
- [ ] QuickJS integration
- [ ] VS Code API compatibility layer
- [ ] Plugin marketplace integration
- [ ] Hot reload плагинов

### Phase 4: Advanced Features (Q4 2025)
- [ ] Integrated terminal
- [ ] Git integration
- [ ] Debug adapter protocol
- [ ] Remote development (SSH, WSL)

## Вклад в Проект

### Как Помочь

1. **Тестирование** — сообщайте о багах
2. **Документация** — улучшайте docs/
3. **Плагины** — тестируйте совместимость
4. **Код** — PR приветствуются!

### Стиль Кода

- Rust: rustfmt (стандартные настройки)
- JavaScript: ESLint (конфиг в разработке)
- Коммиты: Conventional Commits

## Лицензия

MIT License — см. LICENSE файл

## Контакты

- GitHub Issues: для багов и фич
- Discussions: для вопросов и обсуждений

---

**Ultralight Code** — создаётся с ❤️ для сообщества разработчиков
