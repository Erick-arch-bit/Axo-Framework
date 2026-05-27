# Lumina Framework — Arquitectura

## Diagrama C4 Level 2 (Containers)

```mermaid
graph TD
    subgraph "App Layer (Lua)"
        LU["app/*.lua\n(User Code)"]
        LF["app/lumina/*.lua\n(Std Library)"]
    end

    subgraph "Bridge Layer (Rust + mlua)"
        BR["bridge/\nmlua FFI\nAPI pública\nSerialización\nDiffing"]
    end

    subgraph "Core Engine (Rust)"
        RE["core/renderer/\nwgpu pipeline\nShaders\nFrame present"]
        LA["core/layout/\nTaffy\nFlexbox/CSS Grid"]
        WI["core/window/\nWinit\nEvent loop"]
        TX["core/text/\nglyphon\nText shaping"]
        HR["core/hot_reload/\nFile watcher\nWS Server\nLua runtime"]
    end

    subgraph "Platform Layer (Rust + native)"
        DE["Desktop\nwinit + wgpu"]
        AN["Android\nJNI + NativeActivity"]
        IO["iOS\nUIKit + Metal"]
        WE["Web\nWASM + Canvas/WebGL2"]
    end

    LU --> BR
    LF --> BR
    BR --> RE
    BR --> LA
    BR --> WI
    BR --> TX
    BR --> HR
    HR --> LU
    RE --> DE
    RE --> AN
    RE --> IO
    RE --> WE
    WI --> DE
    WI --> AN
    WI --> IO
```

## Capas

### App Layer (Lua)
Código del desarrollador. UI declarativa, estado, lógica de negocio.
- `app/*.lua` — entrada de la aplicación
- `app/lumina/*.lua` — librería estándar de componentes (View, Text, Button, etc.)

### Bridge Layer (Rust)
Comunicación Lua ↔ Rust via mlua.
- `api.rs` — funciones expuestas a Lua
- `serde.rs` — serialización del árbol UI
- `diff.rs` — diffing de virtual DOM para eficiencia

### Core Engine (Rust)
Motor de renderizado, layout y eventos.
- **Renderer**: wgpu (Vulkan/Metal/DX12/WebGL2)
- **Layout**: Taffy (Flexbox/CSS Grid)
- **Window**: Winit (eventos, ventanas)
- **Text**: glyph handling con cosmic-text o glyphon
- **Hot Reload**: file watcher + WebSocket server

### Platform Layer (Rust + platform code)
Adaptadores por plataforma.
- Desktop: nativo vía winit + wgpu
- Android: JNI + NativeActivity
- iOS: UIKit + Metal via objc bindings
- Web: WASM + canvas HTML5

## Lema

> **Crea más rápido. Hazlo completo. Extiéndelo todo.**
