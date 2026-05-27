use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct InitArgs {
    #[arg(default_value = "my-lumina-app")]
    pub name: String,
}

const APP_LUA: &str = r##"local UI = require("lumina")

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
            UI.Text("Hola desde Lumina!", {
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

const README: &str = r#"# Lumina App

Proyecto generado con Lumina Framework.

## Desarrollo

```bash
lumina dev
```

## Build

```bash
lumina build
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

    println!("Creating Lumina project: {}", args.name);

    fs::create_dir_all(project_dir.join("app")).unwrap();

    create_file(project_dir, "app/app.lua", APP_LUA);
    create_file(project_dir, "README.md", README);

    println!();
    println!("Project '{}' created!", args.name);
    println!();
    println!("  cd {}", args.name);
    println!("  lumina dev");
}
