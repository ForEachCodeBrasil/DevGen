# GEMINI.md - DevGen Project Context

## Project Overview
DevGen is a cross-platform desktop utility application designed for developers to generate various types of mock and valid data. It is built using the **Tauri** framework, combining a high-performance **Rust** backend with a modern **Vue 3** frontend.

### Main Technologies
- **Frontend:** Vue 3 (Composition API), TypeScript, Vite, Tailwind CSS, Pinia (State Management), Vue Router, Vue i18n.
- **Backend:** Rust, Tauri v2, Serde, Rand.
- **Icons:** Lucide Vue Next.
- **Testing:** Vitest.

### Core Architecture
- **Generator Registry:** The backend uses a trait-based registry system (`src-tauri/src/generators/`) to manage different data generators (Documents, Person, Company, Vehicle, Utilities).
- **Tray Integration:** The app resides in the system tray (`src-tauri/src/lib.rs`) for quick access to common generation tasks.
- **Preferences Store:** User settings and history are persisted via a local store (`src-tauri/src/store.rs`).
- **I18n:** Supports multiple languages, currently featuring Portuguese (pt-BR) and English (en-US).

---

## Building and Running

### Development
- **Start Web Dev Server:** `npm run dev`
- **Start Tauri App (Backend + Frontend):** `npm run tauri dev`

### Production
- **Build Application:** `npm run tauri build`
- **Preview Frontend Build:** `npm run preview`

### Quality & Testing
- **Run Tests:** `npm run test`
- **Type Check:** `npm run typecheck`
- **Linting:** (Inferring standard Vite/Vue linting) `npm run lint` (TODO: verify if a lint script is explicitly added)

---

## Development Conventions

### Backend (Rust)
- **Generators:** All new generators must implement the `Generator` trait defined in `src-tauri/src/generators/mod.rs` and be registered in `src-tauri/src/lib.rs`.
- **Error Handling:** Use the custom `GeneratorError` enum for consistency.
- **Tauri Commands:** Command handlers are located in `src-tauri/src/lib.rs`.

### Frontend (Vue/TypeScript)
- **Component Pattern:** Use `<script setup>` with TypeScript for all SFCs.
- **State Management:** Use Pinia stores located in `src/stores/`.
- **Styling:** Utilize Tailwind CSS utility classes. Avoid complex custom CSS when possible.
- **I18n:** Ensure all UI strings are added to `src/i18n/en-US.json` and `src/i18n/pt-BR.json`.

### Folder Structure
- `src-tauri/src/generators/impls/`: Implementation of specific data generators.
- `src-tauri/src/datasets/data/`: JSON datasets (names, surnames, etc.) used by generators.
- `src/views/`: Primary application views (Home, Generator, Settings).
- `src/components/`: Reusable UI components.
