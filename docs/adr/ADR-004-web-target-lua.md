# ADR-004: Intérprete Lua para Web (WASM)

- **Status:** Proposed
- **Date:** 2026-05-26
- **Decision-makers:** Axo Core Team

## Context
LuaJIT genera código JIT nativo (x86/ARM). WebAssembly no permite JIT, por lo que LuaJIT no funciona en web.

## Decision
Usaremos dos backends: LuaJIT para plataformas nativas y lua 5.4 (intérprete puro en C, sin JIT) compilado a WASM para el target web.

## Consequences
- **Positive:** Funciona en web sin cambios en el código Lua del usuario
- **Negative:** Rendimiento significativamente menor en web que en nativo
- **Risks:** El tamaño del binario WASM crece al incluir el intérprete Lua

## Alternatives Considered
- Precompilar Lua a Rust (Paladin): experimental, poca madurez
- Solo web via streaming de bytecode precompilado: reduce tamaño pero no mejora velocidad
