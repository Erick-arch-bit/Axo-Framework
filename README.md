# Axo Framework

**Crea más rápido. Hazlo completo. Extiéndelo todo.**

Axo es un framework UI multiplataforma con motor **Rust** (wgpu) y capa de scripting **TypeScript/JavaScript** (QuickJS). Ideal para apps internas, dashboards embebidos, terminales POS, y herramientas de nicho.

## Stack

| Capa | Tecnología |
|------|-----------|
| Renderizado | wgpu (Vulkan/Metal/DX12/WebGL2) |
| Layout | Taffy (Flexbox/CSS Grid) |
| Scripting | TypeScript/JavaScript via QuickJS (rquickjs) |
| Hot Reload | File watcher (notify) |
| Texto | ab_glyph (outline rendering) |
| Objetivos | Linux, macOS, Windows, Android, iOS, Web |

## Quickstart

```bash
# Instalar
cargo install axo-cli

# Crear proyecto
axo-cli init my-app
cd my-app

# Desarrollo con hot reload
axo-cli dev

# Build producción
axo-cli build --mode release
```

## Conceptos

### UI desde TypeScript

```ts
function App() {
    return UI.View({
        style: {
            width: "100%",
            height: "100%",
            backgroundColor: "#1a1a2e",
            flexDirection: "column",
            justifyContent: "center",
            alignItems: "center",
        },
        children: [
            UI.Text("Hola Axo!", {
                fontSize: 24,
                color: "#ffffff",
            }),
            UI.Button({
                text: "Click",
                onClick: () => console.log("Boton presionado!"),
                style: {
                    backgroundColor: "#e94560",
                    width: 200,
                    height: 50,
                    margin: 10,
                },
            }),
        ],
    });
}

App;
```

### onClick — Funciones JS invocables

`onClick` recibe una **función JavaScript**. Axo la guarda y la ejecuta desde Rust cuando el usuario hace clic.

```ts
UI.Button({ text: "Click", onClick: () => console.log("click!") })

// También se acepta un id de callback registrado como string
UI.Button({ text: "Click", onClick: "cb_1" })
```

### Device API

Acceso a hardware del dispositivo desde TypeScript:

```ts
const info = Device.info();
console.log(info.os_name, info.screen_width);

Device.requestPermission("camera");
Device.getLocation();
Device.showNotification("Titulo", "Mensaje");

const data = Device.readFile("data.txt");
Device.writeFile("data.txt", "contenido");
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
axo-cli init <nombre>    # Scaffold proyecto
axo-cli dev              # Desarrollo con hot reload
axo-cli build            # Build debug
axo-cli build --mode release  # Build producción
axo-cli release          # Build + bundle
```

## Estructura del proyecto

```
my-app/
├── app/
│   ├── app.ts               # Entry point (debe evaluar a App())
│   │   └── axo/             # Stdlib JS (UI, useState, Device)
└── README.md
```

## Arquitectura

```
┌─────────────────┐     ┌──────────────────┐
│   TS/JS App     │     │   Rust Core      │
│   app.ts        │◄───►│   wgpu + Taffy   │
│   (QuickJS)     │     │   + ab_glyph     │
└────────┬────────┘     └────────┬─────────┘
          │                       │
          ▼                       ▼
    Device API              Hot Reload
    (permisos,              (file watcher)
     GPS, storage)
```

## Licencia

MIT
