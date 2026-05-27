# Lumina Framework

**Crea más rápido. Hazlo completo. Extiéndelo todo.**

Lumina es un framework UI multiplataforma con motor **Rust** (wgpu) y capa de scripting **Lua**. Ideal para apps internas, dashboards embebidos, terminales POS, y herramientas de nicho.

## Stack

| Capa | Tecnología |
|------|-----------|
| Renderizado | wgpu (Vulkan/Metal/DX12/WebGL2) |
| Layout | Taffy (Flexbox/CSS Grid) |
| Scripting | Lua 5.4 via mlua |
| Hot Reload | File watcher (notify) |
| Texto | ab_glyph (outline rendering) |
| Objetivos | Linux, macOS, Windows, Android, iOS, Web |

## Quickstart

```bash
# Instalar
cargo install --path cli

# Crear proyecto
lumina init my-app
cd my-app

# Desarrollo con hot reload
lumina dev

# Build producción
lumina build --mode release
```

## Conceptos

### UI desde Lua

```lua
local UI = require("lumina")

function App()
    return UI.View({
        style = {
            width = "100%",
            height = "100%",
            backgroundColor = "#1a1a2e",
            flexDirection = "column",
            justifyContent = "center",
            alignItems = "center",
        },
        children = {
            UI.Text("Hola Lumina!", {
                fontSize = 24,
                color = "#ffffff",
            }),
            UI.Button({
                text = "Click",
                onClick = "handleClick",
                style = {
                    backgroundColor = "#e94560",
                    width = 200,
                    height = 50,
                    margin = 10,
                },
            }),
        },
    })
end

function handleClick()
    log("Boton presionado!")
end

return App
```

### onClick — Nombres de función global

`onClick` recibe un **string** con el nombre de una función Lua global. Cuando el usuario hace clic, Lumina busca y ejecuta esa función.

```lua
-- Function reference (almacenada automáticamente)
UI.Button({ onClick = function() print("click") end })

-- Global function name
UI.Button({ onClick = "handleClick" })
```

### Device API

Acceso a hardware del dispositivo desde Lua:

```lua
local info = Device.info()
print(info.os_name, info.screen_width)

Device.requestPermission("camera")
Device.getLocation()
Device.showNotification("Titulo", "Mensaje")

local data = Device.readFile("data.txt")
Device.writeFile("data.txt", "contenido")
```

| Función | Descripción |
|---------|-------------|
| `Device.info()` | OS, versión, modelo, pantalla |
| `Device.checkPermission(name)` | Estado de permiso |
| `Device.requestPermission(name)` | Solicitar permiso |
| `Device.getLocation()` | GPS (lat, lng, accuracy) |
| `Device.getSensors()` | Acelerómetro, giroscopio |
| `Device.readFile(path)` | Leer archivo |
| `Device.writeFile(path, content)` | Escribir archivo |
| `Device.deleteFile(path)` | Eliminar archivo |
| `Device.showNotification(title, body)` | Notificación |
| `Device.takePhoto()` | Cámara (stub) |

### Permisos disponibles

- `"camera"`, `"location"`, `"storage"`, `"notifications"`, `"microphone"`, `"contacts"`

## CLI

```bash
lumina init <nombre>    # Scaffold proyecto
lumina dev              # Desarrollo con hot reload
lumina build            # Build debug
lumina build --mode release  # Build producción
lumina release          # Build + bundle
```

## Estructura del proyecto

```
my-app/
├── app/
│   ├── app.lua              # Entry point (debe retornar App())
│   └── lumina/init.lua      # Std library (View, Text, Button, etc.)
└── README.md
```

## Arquitectura

```
┌─────────────────┐     ┌──────────────────┐
│   Lua App       │     │   Rust Core      │
│   app.lua       │◄───►│   wgpu + Taffy   │
│   init.lua      │     │   + ab_glyph     │
└────────┬────────┘     └────────┬─────────┘
         │                       │
         ▼                       ▼
   Device API              Hot Reload
   (permisos,              (file watcher)
    GPS, storage)
```

## Licencia

MIT
# Lumina-Framework
