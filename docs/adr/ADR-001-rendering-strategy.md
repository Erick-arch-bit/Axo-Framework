# ADR-001: Estrategia de Renderizado

- **Status:** Proposed
- **Date:** 2026-05-26
- **Decision-makers:** Axo Core Team

## Context
El framework necesita renderizar interfaces en múltiples plataformas sin usar vistas nativas. Las opciones son wgpu (GPU directa) o skia-rs (skia-safe como backend).

## Decision
Usaremos wgpu como backend primario de renderizado, con capacidad de fallback a WebGL2 en web.

## Consequences
- **Positive:** Control total del pipeline gráfico, binaries más pequeños, renderizado unificado
- **Negative:** Requiere implementar text shaping, trazado vectorial y accesibilidad desde cero
- **Risks:** wgpu no tiene text shaper nativo; dependemos de glyphon o cosmic-text

## Alternatives Considered
- Skia-rs: más maduro, pero binarios más pesados y bindings C++ complejos
