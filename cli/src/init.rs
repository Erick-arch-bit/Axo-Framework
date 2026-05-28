use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct InitArgs {
    #[arg(default_value = "my-axo-app")]
    pub name: String,
}

const INIT_LUA: &str = r##"local Axo = {}

function Axo.View(props)
    return {
        type = "View",
        style = props.style or {},
        children = props.children or {},
        content = "",
    }
end

function Axo.Text(content, style)
    return {
        type = "Text",
        content = content or "",
        style = style or {},
        children = {},
    }
end

function Axo.Button(props)
    return {
        type = "Button",
        content = props.text or "",
        style = props.style or {},
        children = {},
    }
end

function Axo.Image(props)
    return {
        type = "Image",
        content = props.source or "",
        style = props.style or {},
        children = {},
    }
end

function Axo.ScrollView(props)
    return {
        type = "ScrollView",
        style = props.style or {},
        children = props.children or {},
        content = "",
    }
end

function Axo.TextInput(props)
    return {
        type = "TextInput",
        content = props.value or "",
        style = props.style or {},
        children = {},
    }
end

-- Device API — wraps the Rust `Device` global for convenience
function Axo.Device()
    return _G.Device
end

function Axo.getDeviceInfo()
    if _G.Device then
        return _G.Device.info()
    end
    return {}
end

function Axo.checkPermission(name)
    if _G.Device then
        return _G.Device.checkPermission(name)
    end
    return "denied"
end

function Axo.requestPermission(name)
    if _G.Device then
        return _G.Device.requestPermission(name)
    end
    return "denied"
end

function Axo.getLocation()
    if _G.Device then
        return _G.Device.getLocation()
    end
    return nil
end

function Axo.getSensors()
    if _G.Device then
        return _G.Device.getSensors()
    end
    return { accelerometer = nil, gyroscope = nil, magnetometer = nil }
end

function Axo.readFile(path)
    if _G.Device then
        return _G.Device.readFile(path)
    end
    return nil
end

function Axo.writeFile(path, content)
    if _G.Device then
        return _G.Device.writeFile(path, content)
    end
    return false
end

function Axo.deleteFile(path)
    if _G.Device then
        return _G.Device.deleteFile(path)
    end
    return false
end

function Axo.fileExists(path)
    if _G.Device then
        return _G.Device.fileExists(path)
    end
    return false
end

function Axo.storagePath()
    if _G.Device then
        return _G.Device.storagePath()
    end
    return ""
end

function Axo.showNotification(title, body)
    if _G.Device then
        return _G.Device.showNotification(title, body)
    end
    return false
end

function Axo.takePhoto()
    if _G.Device then
        return _G.Device.takePhoto()
    end
    return nil
end

return Axo
"##;

const APP_LUA: &str = r##"local UI = require("axo")

function onButtonClick()
    log("Boton clickeado!")
end

function App()
    local info = UI.getDeviceInfo()

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
            UI.Text("Hola desde Axo!", {
                fontSize = 28,
                color = "#ffffff",
            }),
            UI.Button({
                text = "Mi Boton",
                onClick = "onButtonClick",
                style = {
                    backgroundColor = "#e94560",
                    width = "200",
                    height = "50",
                    margin = "10",
                },
            }),
            UI.Text("OS: " .. (info.os_name or "desconocido"), {
                fontSize = 12,
                color = "#666666",
            }),
        },
    })
end

return App
"##;

const README: &str = r#"# Axo App

Proyecto generado con Axo Framework.

## Desarrollo

```bash
axo dev
```

## Build

```bash
axo build
```
"#;

fn create_file(path: &Path, name: &str, content: &str) {
    let file_path = path.join(name);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&file_path, content).unwrap();
    println!("  Created {}", file_path.display());
}

pub fn run(args: InitArgs) {
    let project_dir = Path::new(&args.name);

    if project_dir.exists() {
        eprintln!("Error: directory '{}' already exists", args.name);
        return;
    }

    println!("Creating Axo project: {}", args.name);

    fs::create_dir_all(project_dir.join("app")).unwrap();

    create_file(project_dir, "app/app.lua", APP_LUA);
    create_file(project_dir, "app/axo/init.lua", INIT_LUA);
    create_file(project_dir, "README.md", README);

    println!();
    println!("Project '{}' created!", args.name);
    println!();
    println!("  cd {}", args.name);
    println!("  axo dev");
}
