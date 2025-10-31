# JobTrackr — Local-First Job Application Manager
**Version:** v0.1.0

JobTrackr is a desktop-first, single-user application that helps manage job applications, companies, contacts, notes, interactions, and reminders — entirely offline.
All data is stored locally in an embedded SQLite database, and no cloud or network access is required.

---

## Overview

JobTrackr provides a structured local workflow for tracking every step of your job search.
It integrates a Rust + Tauri backend for database and file handling, with a modern SvelteKit + Tailwind + shadcn-svelte frontend.

---

## Tech Stack

- **Backend:** Rust, Tauri v2, `sqlx` (SQLite), `tracing` for logging
- **Frontend:** SvelteKit 5 (runes), TailwindCSS, shadcn-svelte, Lucide icons
- **Dates:** `@internationalized/date` for timezone-safe `CalendarDate` handling
- **Data Storage:** `~/.JobTrackr/jobtrackr.db`
- **Logs:** `~/.JobTrackr/logs/YYYY-MM/jobtrackr.YYYY-MM-DD.log`

---

## Architecture

- **Tauri commands** act as the bridge between Svelte and Rust, exposing CRUD for all entities.
- **Services** in Rust perform logic and formatting before returning JSON responses.
- **Svelte stores** mirror backend entities and expose load/create/update/delete functions.
- **Dialogs** are modular Svelte components for creating and editing records.
- **Homepage** preloads all stores to ensure fast navigation and data availability.

---

## Data Model

### Entities
- Company
- Person
- JobListing
- Application
- Interaction
- Note
- Reminder

### Enumerations
`Stage`, `WorkType`, `SeniorityLevel`, `Currency`, `Role`, `NoteType`, `InteractionType`

Each entity includes timestamps (`createdAt`, `updatedAt`) and a generated `displayLabel`.

---

## Frontend Behavior

### Reactive State (Svelte 5 Runes)
Use `$state` and `$bindable`, and **never** mutate objects in place.

```ts
// Correct
$formData = { ...existing, field: value };

// Incorrect (non-reactive)
Object.assign($formData, existing);
```

### Edit Mode
Dialogs rehydrate by fully reassigning `$formData` to maintain reactivity.

### Global Preload
All entity stores are loaded when the homepage mounts:

```ts
onMount(() => Promise.all([
  loadApplications(),
  loadCompanies(),
  loadNotes(),
  loadJobListings(),
  loadPeople(),
  loadReminders(),
  loadInteractions()
]));
```

### Date Handling
Avoid `.toISOString()` to prevent UTC shifts.
Construct date strings manually:

```ts
const d = $formData.reminderDate;
const yyyyMmDd = `${d.year}-${String(d.month).padStart(2, "0")}-${String(d.day).padStart(2, "0")}`;
```

---

## Features (v0.1.0)

- Full CRUD dialogs for all entities
- Unified alert system (success, error, info)
- Homepage with:
  - Recent **Applications**
  - Pending **Reminders**
  - Recent **Notes**
- Fallback messages when lists are empty
- Live-updated stores and dialogs
- Timezone-safe date handling
- Automatic store preload for subpages

---

## Directory Structure

```
src-tauri/
  src/
    commands/      # Tauri command functions
    services/      # Business logic layer
    db/            # SQL schema, models, enums
    utils/         # Logging, helpers

src/
  lib/
    stores/        # Svelte stores for CRUD
    components/
      formDialogs/ # Entity dialogs
      ui/          # shadcn-svelte components
      utils/       # Custom selectors, alerts, date pickers
  routes/
    +page.svelte   # Homepage using live store data
```

---

## Installation

### Prerequisites

**All platforms**
- Rust (stable)
- Cargo
- Node.js 18+
- `pnpm` or `npm`

**Linux**
```bash
sudo apt install libgtk-3-dev webkit2gtk-4.1 pkg-config appmenu-gtk3-module
```

**macOS**
```bash
xcode-select --install
```

**Windows**
- Visual Studio Build Tools
- WebView2 Runtime

---

### Steps

```bash
# Clone the repository
git clone https://github.com/<your-user>/JobTrackr.git
cd JobTrackr

# Install frontend dependencies
pnpm install     # or: npm install

# Fetch Rust dependencies
cargo fetch
```

---

## Development

```bash
pnpm tauri dev    # or: npm run tauri dev
```

- SQLite database is created at `~/.JobTrackr/jobtrackr.db`
- Logs are stored under `~/.JobTrackr/logs/YYYY-MM/`

---

## Build (Release)

```bash
pnpm tauri build   # or: npm run tauri build
```

Binaries are placed in:
```
src-tauri/target/release/bundle/
```

---

## Usage

### Dialog Lifecycle
1. Open dialog and fill form
2. Submit → backend updates SQLite
3. Store updates → UI refreshes → dialog closes

### Store Interface
Each store exposes:
```ts
loadX();
createX(payload);
updateX(id, updates);
deleteX(id);
```

### Routing
Links use:
```svelte
<a href={resolve("/path")}>Link</a>
```
This ensures safety under SvelteKit 2+.

---

## Logging

Logs rotate daily and are grouped by month:
`~/.JobTrackr/logs/YYYY-MM/jobtrackr.YYYY-MM-DD.log`

Includes startup, database initialization, and runtime events.

---

## Troubleshooting

**Dialogs don’t update properly**
Reassign `$formData` entirely rather than mutating.

**Date saved as previous day**
Avoid `.toISOString()`; build `YYYY-MM-DD` manually.

**Selectors empty**
Ensure preload occurs before opening dialogs.

**Linux build fails**
Install missing GTK or WebKit dependencies.

---

## Security and Privacy

- 100% local data — no internet access required
- No telemetry, analytics, or cloud sync
- All user content stored in `~/.JobTrackr/`

---

## Known Limitations

- No schema migration UI
- No multi-user support
- Minimal form validation
- No filtering/sorting UI yet

---

## Roadmap

- Sorting and filtering for all list pages
- export (JSON, CSV, Excel, DB)
- import (JSON, CSV, Excel, DB)

---

## License

This project is licensed under the [MIT License](./LICENSE).

---

## Credits

- **UI Components:** shadcn-svelte (adapted for Svelte 5)
- **Icons:** Lucide
- **Frameworks:** Tauri, SvelteKit
- **Author:** Chris Nielsen (Rodhor)
