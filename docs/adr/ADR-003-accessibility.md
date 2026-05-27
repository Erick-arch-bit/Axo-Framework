# ADR-003: Estrategia de Accesibilidad

- **Status:** Proposed
- **Date:** 2026-05-26
- **Decision-makers:** Lumina Core Team

## Context
El renderizado pixel-by-pixel sin vistas nativas rompe la accesibilidad nativa (VoiceOver, TalkBack, lectores de pantalla).

## Decision
Implementaremos un semantic tree paralelo que se expone al SO mediante platform channels. En iOS se usará UIAccessibility, en Android AccessibilityNodeInfo.

## Consequences
- **Positive:** Apps accesibles sin sacrificar el motor de renderizado unificado
- **Negative:** Mantener dos árboles (visual + semántico) agregar complejidad
- **Risks:** Podría haber desincronización entre árbol visual y semántico

## Alternatives Considered
- Ignorar accesibilidad: inviable para apps enterprise/gobierno
- Usar overlays nativos: rompe la premisa de renderizado unificado
