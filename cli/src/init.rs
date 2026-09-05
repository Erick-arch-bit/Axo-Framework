use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct InitArgs {
    #[arg(default_value = "my-axo-app")]
    pub name: String,
}

const APP_TS: &str = r##"function App() {
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
        fontSize: 32,
        color: "#ffffff",
      }),
      UI.Text("Edita app/app.ts para empezar.", {
        fontSize: 14,
        color: "#e94560",
      }),
    ],
  });
}

App;
"##;

const STDLIB_STATE_JS: &str = r##"// Axo Framework — stdlib JS (estado mínimo: useState).
(function () {
  if (!globalThis.__AXO_STATE) {
    globalThis.__AXO_STATE = {};
  }

  function useState(key, initialValue) {
    if (!(key in globalThis.__AXO_STATE)) {
      globalThis.__AXO_STATE[key] = initialValue;
    }
    function setValue(next) {
      globalThis.__AXO_STATE[key] = next;
    }
    return [globalThis.__AXO_STATE[key], setValue];
  }

  globalThis.useState = useState;
})();
"##;

const STDLIB_COMPONENTS_JS: &str = r##"// Axo Framework — stdlib JS (componentes UI mínimos: View, Text, Button, Input).
(function () {
  if (!globalThis.__AXO_CALLBACKS) {
    globalThis.__AXO_CALLBACKS = {};
  }
  var __axoCbCounter = 0;

  function __axoNormalizeOnClick(onClick) {
    if (typeof onClick === "function") {
      __axoCbCounter += 1;
      var id = "cb_" + __axoCbCounter;
      globalThis.__AXO_CALLBACKS[id] = onClick;
      return id;
    }
    if (typeof onClick === "string") {
      return onClick;
    }
    return null;
  }

  function View(props) {
    props = props || {};
    return {
      type: "View",
      content: "",
      style: props.style || {},
      children: props.children || []
    };
  }

  function Text(contentOrProps, maybeProps) {
    var content = "";
    var style = {};
    if (typeof contentOrProps === "string" || typeof contentOrProps === "number") {
      content = String(contentOrProps);
      var p = maybeProps || {};
      style = p.style || {};
      if (p.fontSize !== undefined && style.fontSize === undefined) {
        style.fontSize = p.fontSize;
      }
      if (p.color !== undefined && style.color === undefined) {
        style.color = p.color;
      }
    } else {
      var q = contentOrProps || {};
      content = q.content || q.text || "";
      if (typeof content !== "string") {
        content = String(content);
      }
      style = q.style || {};
    }
    return { type: "Text", content: content, style: style, children: [] };
  }

  function Button(props) {
    props = props || {};
    var content = props.text || props.content || "";
    if (typeof content !== "string") {
      content = String(content);
    }
    var node = { type: "Button", content: content, style: props.style || {}, children: [] };
    var oc = __axoNormalizeOnClick(props.onClick);
    if (oc !== null) {
      node.onClick = oc;
    }
    return node;
  }

  function Input(props) {
    props = props || {};
    var content = props.value || props.text || props.content || "";
    if (typeof content !== "string") {
      content = String(content);
    }
    var node = { type: "Input", content: content, style: props.style || {}, children: [] };
    var oc = __axoNormalizeOnClick(props.onClick);
    if (oc !== null) {
      node.onClick = oc;
    }
    return node;
  }

  globalThis.__axoComponents = { View: View, Text: Text, Button: Button, Input: Input };
})();
"##;

const STDLIB_INDEX_JS: &str = r##"// Axo Framework — stdlib JS (entrypoint: expone UI y useState como globales).
(function () {
  var c = globalThis.__axoComponents || {};
  var UI = { View: c.View, Text: c.Text, Button: c.Button, Input: c.Input };
  globalThis.UI = UI;
})();
"##;

const AXO_JSON: &str = r#"{
    "name": "my-app",
    "version": "0.1.0",
    "entry": "app/app.ts"
}
"#;

const README: &str = r##"# Axo App

Project generated with [Axo Framework](https://github.com/Erick-arch-bit/Axo-Framework).

## Quickstart

```bash
# Development with hot reload
axo-cli dev

# Production build
axo-cli build --mode release
```

## Project Structure

```
app/
  app.ts            -- Entry point (TypeScript, debe evaluar a App)
  axo/              -- Stdlib JS (UI, useState, Device bridge)
README.md
axo.json           -- Project config (entry: app/app.ts)
```

## UI en TypeScript

```ts
function App() {
  return UI.View({
    style: { width: "100%", height: "100%", backgroundColor: "#1a1a2e" },
    children: [
      UI.Text("Hola Axo!", { fontSize: 32, color: "#ffffff" }),
      UI.Button({
        text: "Click",
        onClick: () => console.log("click!"),
        style: { backgroundColor: "#e94560", width: 200, height: 50 },
      }),
    ],
  });
}

App;
```

El entry puede ser `app/app.ts` o `app/app.js` (`axo-cli dev` prefiere `.ts`).
"##;

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

    // App entry (TypeScript) + stdlib JS
    create_file(project_dir, "app/app.ts", APP_TS);
    create_file(project_dir, "app/axo/state.js", STDLIB_STATE_JS);
    create_file(project_dir, "app/axo/components.js", STDLIB_COMPONENTS_JS);
    create_file(project_dir, "app/axo/index.js", STDLIB_INDEX_JS);

    // Project config and docs
    create_file(project_dir, "axo.json", AXO_JSON);
    create_file(project_dir, "README.md", README);

    // Assets placeholder
    fs::create_dir_all(project_dir.join("assets")).unwrap();

    println!();
    println!("Project '{}' created!", args.name);
    println!();
    println!("  cd {}", args.name);
    println!("  axo-cli dev");
    println!();
    println!("Entry: app/app.ts (TypeScript via QuickJS)");
}
