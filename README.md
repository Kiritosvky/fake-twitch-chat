# fake-twitch-chat

Фейковый русскоязычный Twitch-чат в тёмной теме, окно всегда поверх остальных.

[![Скачать последнюю версию](https://img.shields.io/badge/%E2%AC%87%EF%B8%8F_Скачать-fake--twitch--chat.exe-9147ff?style=for-the-badge&labelColor=18181b)](https://github.com/Kiritosvky/fake-twitch-chat/releases/latest/download/fake-twitch-chat.exe)

**Windows x64, ничего доустанавливать не нужно.** Скачал → запустил.

## Версии

Каждый релиз хранится отдельно, старые не удаляются.

| Версия | Скачать exe | Заметки |
|---|---|---|
| **1.0.0** (latest) | [fake-twitch-chat.exe](https://github.com/Kiritosvky/fake-twitch-chat/releases/download/v1.0.0/fake-twitch-chat.exe) | Первый релиз: 50 сообщений/мин, АФК-детект, CS2 / BeamNG / YouTube / вайбкодинг, 324 фразы из реального твич-чата |

Все версии: [github.com/Kiritosvky/fake-twitch-chat/releases](https://github.com/Kiritosvky/fake-twitch-chat/releases)

- 50 сообщений в минуту в любом контексте
- АФК 60 секунд без ввода мыши/клавиатуры → «стример в АФК», «завис» и т.п.
- Определяет активное окно: CS2, BeamNG.drive, YouTube (по названию видео), редактор кода (вайбкодинг)
- 324 реальные фразы из твич-логов, переведённые на русский (`assets/phrases.txt`)

## Сборка

```
cargo build --release
```

Готовый файл: `target/release/fake-twitch-chat.exe`. Зависимостей нет: чистый Win32 GDI + статический CRT, ничего доустанавливать не нужно.

## Как выпустить новую версию

1. Поменять `version` в `Cargo.toml`.
2. `cargo build --release`
3. `git commit -am "1.1.0" && git tag v1.1.0 && git push --tags`
4. На [Releases → Draft a new release](https://github.com/Kiritosvky/fake-twitch-chat/releases/new) выбрать тег `v1.1.0`, прикрепить `target/release/fake-twitch-chat.exe` (имя файла оставить `fake-twitch-chat.exe`) и опубликовать.

Кнопка «Скачать последнюю версию» в README всегда ведёт на `/releases/latest/download/fake-twitch-chat.exe`, поэтому её менять не нужно.

## Структура

```
Cargo.toml           зависимости (только windows-sys) и профиль релиза
.cargo/config.toml   статическая линковка CRT
src/main.rs          вся программа: окно, GDI-рендер, детект контекста, генератор сообщений
assets/phrases.txt   переведённые фразы реального твич-чата (вшиваются в exe)
tools/               скрипты, которыми собирались фразы (в exe не входят)
LICENSE              MIT
```

## Лицензия

MIT
