# Posts API Server

REST API сервер на Rust с JWT авторизацией для управления пользователями и постами.

## Технологии

- Rust + Tokio (асинхронный рантайм)
- Axum (веб-фреймворк)
- Diesel + diesel-async (ORM для PostgreSQL)
- jsonwebtoken (JWT авторизация)
- argon2_async (хэширование паролей)
- Validator (валидация данных)

## Установка и запуск

### Требования

- Rust (nightly или stable)
- PostgreSQL

### Настройка

1. Клонируйте репозиторий:

   git clone <repository-url>
   
   cd posts_api_server

2. Создайте файл .env:

   SERVER_IP=127.0.0.1:3000
   
   DATABASE_URL=postgres://user:password@localhost:5432/posts_api

### JWT

3. AUDIENCE=http://localhost:3000
   
4. JWT_ACCESS_SECRET=your-very-secret-access-key-minimum-32-bytes
   
5. JWT_REFRESH_SECRET=your-very-secret-refresh-key-minimum-32-bytes

6. Запустите миграции:

   diesel migration run

7. Запустите сервер:

   cargo run --release

## Авторизация

API использует JWT токены двух типов:

- Access token (15 минут) — для доступа к защищённым эндпоинтам
- Refresh token (7 дней) — для получения новой пары токенов

Для доступа к защищённым эндпоинтам добавьте заголовок:

   Authorization: Bearer <access_token>

## Эндпоинты

### Публичные эндпоинты

#### GET /

Проверка работоспособности сервера.

Ответ: ```json "pong" ```

---

#### POST /users

Регистрация нового пользователя.

Тело запроса:

```json

   {
       "nickname": "john_doe",
       "password": "secure_password",
       "about": "Описание пользователя (опционально)"
   }

```

Успешный ответ (200):

```json

   {
       "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
       "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
       "user_id": 1,
       "nickname": "john_doe"
   }

```

---

#### POST /login

Вход пользователя.

```json

Тело запроса:

   {
       "nickname": "john_doe",
       "password": "secure_password"
   }

```

Успешный ответ (200):

```json

   {
       "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
       "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
       "user_id": 1,
       "nickname": "john_doe"
   }

```

---

#### GET /users/id/{id}

Получение пользователя по ID.

Параметр пути: id — числовой идентификатор пользователя

Успешный ответ (200):

```json

   {
       "id": 1,
       "nickname": "john_doe",
       "about": "Описание пользователя",
       "created_at": "2024-01-01T00:00:00",
       "last_updated": "2024-01-01T00:00:00"
   }

```

---

#### GET /users/nickname/{nickname}

Получение пользователя по nickname.

Параметр пути: nickname — строковый никнейм пользователя

Успешный ответ (200):

```json

   {
       "id": 1,
       "nickname": "john_doe",
       "about": "Описание пользователя",
       "created_at": "2024-01-01T00:00:00",
       "last_updated": "2024-01-01T00:00:00"
   }

```

---

#### GET /users

Список пользователей с пагинацией и сортировкой.

Параметры запроса:

   offset — смещение (по умолчанию 0)
   sort_by — сортировка: nickname_asc, nickname_desc, creation_time_asc, creation_time_desc

Пример: GET /users?offset=10&sort_by=nickname_asc

Успешный ответ (200):

```json

   [
       {
           "id": 1,
           "nickname": "alice",
           "about": null,
           "created_at": "2024-01-01T00:00:00",
           "last_updated": "2024-01-01T00:00:00"
       },
       {
           "id": 2,
           "nickname": "bob",
           "about": "Hello",
           "created_at": "2024-01-02T00:00:00",
           "last_updated": "2024-01-02T00:00:00"
       }
   ]

```

---

#### GET /posts/id/{id}

Получение поста по ID.

Параметр пути: id — числовой идентификатор поста

Успешный ответ (200):

```json

   {
       "id": 1,
       "title": "Заголовок поста",
       "description": "Краткое описание",
       "content": "Полное содержание поста...",
       "author": "john_doe",
       "created_at": "2024-01-01T00:00:00",
       "last_updated": "2024-01-01T00:00:00"
   }

```

---

#### GET /posts/author/{author}

Получение всех постов автора.

Параметры пути:

   author — nickname автора

Параметры запроса:

   offset — смещение (по умолчанию 0)

Пример: GET /posts/author/john_doe?offset=10

Успешный ответ (200):

```json

   [
       {
           "id": 1,
           "title": "Первый пост",
           "description": "Описание",
           "content": "Содержание...",
           "author": "john_doe",
           "created_at": "2024-01-01T00:00:00",
           "last_updated": "2024-01-01T00:00:00"
       }
   ]

```

---

#### GET /posts

Список постов с пагинацией и сортировкой.

Параметры запроса:

- offset — смещение (по умолчанию 0)
   
- sort_by — сортировка: creation_time_asc, creation_time_desc

Пример: GET /posts?offset=10&sort_by=creation_time_desc

Успешный ответ (200):

```json

   [
       {
           "id": 1,
           "title": "Новый пост",
           "description": null,
           "content": "Содержание...",
           "author": "john_doe",
           "created_at": "2024-01-01T00:00:00",
           "last_updated": "2024-01-01T00:00:00"
       }
   ]

```

---

### Защищённые эндпоинты (требуют авторизации)

#### PATCH /users

Обновление своего профиля.

Заголовок: Authorization: Bearer <access_token>

Тело запроса:

```json

   {
       "nickname": "new_nickname",
       "about": "Новое описание"
   }

```

Оба поля опциональны.

Успешный ответ (200):

```json

   {
       "id": 1,
       "nickname": "new_nickname",
       "about": "Новое описание",
       "created_at": "2024-01-01T00:00:00",
       "last_updated": "2024-01-02T00:00:00"
   }

```

---

#### POST /posts

Создание нового поста.

Заголовок: Authorization: Bearer <access_token>

Тело запроса:

```json

   {
       "title": "Заголовок (3-50 символов)",
       "description": "Описание (3-200 символов, опционально)",
       "content": "Содержание (минимум 20 символов)"
   }

```

Успешный ответ (200):

```json

   {
       "id": 1,
       "title": "Заголовок",
       "description": "Описание",
       "content": "Содержание...",
       "author": "john_doe",
       "created_at": "2024-01-01T00:00:00",
       "last_updated": "2024-01-01T00:00:00"
   }

```

---

#### PATCH /posts

Обновление своего поста.

Заголовок: Authorization: Bearer <access_token>

Тело запроса:

```json

   {
       "id": 1,
       "title": "Новый заголовок (опционально)",
       "description": "Новое описание (опционально)",
       "content": "Новое содержание (опционально)"
   }

```

Успешный ответ (200):

```json

   {
       "id": 1,
       "title": "Новый заголовок",
       "description": "Новое описание",
       "content": "Новое содержание...",
       "author": "john_doe",
       "created_at": "2024-01-01T00:00:00",
       "last_updated": "2024-01-02T00:00:00"
   }

```

---

#### DELETE /logout

Выход из системы (отзыв refresh токена).

Заголовок: Authorization: Bearer <refresh_token>

Успешный ответ: 204 No Content

---

#### GET /refresh

Обновление пары токенов (access + refresh).

Заголовок: Authorization: Bearer <refresh_token>

Успешный ответ (200):

```json

   {
       "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
       "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
       "user_id": 1,
       "nickname": "john_doe"
   }

```

## Коды ответов

200 OK — Успешный запрос

204 No Content — Успешный запрос (без содержимого)

400 Bad Request — Некорректные данные запроса

401 Unauthorized — Неавторизованный доступ

403 Forbidden — Доступ запрещён

404 Not Found — Ресурс не найден

409 Conflict — Конфликт (например, nickname уже занят)

500 Internal Server Error — Внутренняя ошибка сервера

## Структура проекта

src/

├── handlers/        # HTTP хендлеры

├── services/        # Бизнес-логика

├── repositories/    # Работа с БД

├── models/          # DTO и сущности

├── auth/            # JWT и хэширование

├── errors/          # Обработка ошибок

├── state/           # Состояние приложения

├── constants.rs     # Константы

├── lib.rs           # Основной модуль

├── main.rs          # Точка входа

└── schema.rs        # Схема БД (Diesel)

## Разработка

Локальная разработка:

   cargo run
