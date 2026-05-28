# ADR-005: Manejo del Ciclo de Vida Mobile

- **Status:** Proposed
- **Date:** 2026-05-26
- **Decision-makers:** Axo Core Team

## Context
Android (onResume/onPause/onDestroy) e iOS (applicationState) tienen ciclos de vida que el core Rust debe manejar para no perder estado.

## Decision
El core Rust expondrá callbacks que Lua puede suscribir: on_resume, on_pause, on_destroy. El core notificará a Lua cuando el SO emita estos eventos.

## Consequences
- **Positive:** El desarrollador controla el ciclo de vida desde Lua
- **Negative:** Si Lua no suscribe callbacks, el comportamiento default debe ser seguro
- **Risks:** Estado de la GPU (wgpu) puede perderse al suspender la app

## Alternatives Considered
- Que Rust maneje todo internamente: menos flexible pero más robusto
