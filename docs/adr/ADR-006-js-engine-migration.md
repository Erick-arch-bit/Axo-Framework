# ADR-006: Migración de Lua a motor JavaScript (QuickJS)

## Estado
En progreso — Fase 3 completada (TypeScript + hot-reload mínimo)

## Fecha
2026-09-04

## Contexto
Axo Framework utiliza actualmente Lua 5.4 embebido a través del crate `mlua` como capa de scripting.
Se desea migrar a un motor JavaScript embebido para:
- Mejorar la experiencia de desarrollo (soporte de TypeScript)
- Mantener alto rendimiento y bajo peso del binario
- Conservar el modelo de un solo proceso

## Decisión
Adoptar **QuickJS** mediante el crate `rquickjs` como nuevo motor de scripting.

## Consecuencias
- El modelo de datos `UiNode` y `StyleMap` se mantiene sin cambios.
- El core de renderizado (`core/`) no se modifica.
- La migración se realiza por fases controladas.
- Fase 0 (esta) solo prepara la estructura y dependencias.
- Lua (`mlua`) permanece activo hasta que las fases posteriores estén completas y validadas.

## Alternativas consideradas
- Motor 100 % Rust (Boa) → más pesado y menos maduro para embeber.
- V8 / deno_core → demasiado pesado para el objetivo de Axo.
- Bun como proceso externo → rompe el modelo de un solo proceso y complica hot-reload.
- Escribir un motor propio desde cero → fuera de alcance.

## Fases planificadas
- Fase 0: Preparación (esta ADR)
- Fase 1: Bridge mínimo con QuickJS
- Fase 2: Stdlib TypeScript
- Fase 3: Hot-reload + soporte TypeScript
- Fase 4: Device API + eventos
- Fase 5: Limpieza de Lua + documentación final

## Progreso
- [x] Fase 0 — Preparación
- [x] Fase 1 — Bridge mínimo QuickJS
- [x] Fase 2 — Stdlib JavaScript
- [x] Fase 3 — TypeScript + hot-reload mínimo
- [ ] Fase 4 — Device API + eventos reales
- [ ] Fase 5 — Limpieza de Lua

## Nota Fase 3 — Estrategia de transpile (Opción B)
Se eligió la **Opción B**: `transpile_ts_to_js` invoca `npx --yes esbuild --loader=ts --format=esm --target=es2020`
por `std::process::Command`, pasando el fuente por stdin y capturando stdout como JS.
Motivo: solución mínima sin nuevas dependencias pesadas de compilación (sin SWC/OXC),
sin Node en runtime (solo herramienta de build/transpile) y sin Bun.
Si `npx`/`esbuild` no están disponibles, el error es claro y accionable.
El hot-reload (`watch_and_reload` en `bridge/src/watch.rs`) usa `notify 7`
(misma versión que `core`, añadido solo en `bridge`) con debounce de 200 ms;
el CLI (`axo dev`) no se tocó porque su pipeline Lua no admite un cambio trivial y seguro.
