# Changelog — Axo Framework

## v0.2.0

- **Removed**: Lua / mlua scripting (`app.lua`, `app/axo/init.lua`, bridge Lua)
- **Added**: TypeScript/JavaScript via QuickJS (`app/app.ts`, stdlib `app/axo/*.{js,ts}`)
- **Added**: Device API + onClick invocable desde Rust en JS/TS
- **Breaking**: entrypoint `app/app.ts` (ya no `app/app.lua`); `axo-cli init/dev` trabajan con TS/JS
- Migración desde Lua → ver `docs/adr/ADR-006-js-engine-migration.md`
