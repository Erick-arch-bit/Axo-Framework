# ADR-002: Modelo de Datos del Bridge

- **Status:** Proposed
- **Date:** 2026-05-26
- **Decision-makers:** Lumina Core Team

## Context
El puente Lua↔Rust debe transferir árboles de UI de forma eficiente. Enviar el árbol completo en cada frame (60fps) puede ser un bottleneck.

## Decision
Implementaremos un virtual DOM inmutable en Rust. Lua enviará solo patches (diff) del árbol UI, no el árbol completo.

## Consequences
- **Positive:** Reducción drástica de tráfico en el bridge, escalable a 60fps
- **Negative:** Complejidad adicional en el diffing y reconciliación de estado
- **Risks:** El diffing puede ser costoso si el árbol es muy profundo

## Alternatives Considered
- Árbol completo por frame: simple pero cuello de botella asegurado
- Doble buffer de árboles: más memoria pero predecible
