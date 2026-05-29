use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct InitArgs {
    #[arg(default_value = "my-axo-app")]
    pub name: String,
}

const INIT_LUA: &str = r##"
local Axo = {}

-- Estado reactivo
local _state = {}
local _stateListeners = {}

function Axo.useState(key, initialValue)
    if _state[key] == nil then
        _state[key] = initialValue
    end
    return {
        get = function() return _state[key] end,
        set = function(v)
            _state[key] = v
            if _stateListeners[key] then
                for _, cb in ipairs(_stateListeners[key]) do
                    cb(v)
                end
            end
        end,
        subscribe = function(cb)
            if not _stateListeners[key] then
                _stateListeners[key] = {}
            end
            table.insert(_stateListeners[key], cb)
        end,
    }
end

-- Utility: shallow merge b into a (b wins)
local function mergeInto(a, b)
    if not b then return a end
    for k, v in pairs(b) do a[k] = v end
    return a
end

-- Utility: merge two tables into a new one (b wins on conflict)
local function mergeTables(a, b)
    local r = {}
    if a then for k, v in pairs(a) do r[k] = v end end
    if b then for k, v in pairs(b) do r[k] = v end end
    return r
end

-- Component registry for modding
local _components = {}

-- CSS-like pseudo props
local function extractPseudoProps(props)
    return {
        hoverStyle = props.hoverStyle,
        activeStyle = props.activeStyle,
        disabled = props.disabled,
    }
end

-- Base: every component gets pseudo props + onClick passthrough
local function baseNode(props, nodeType, overrides)
    overrides = overrides or {}
    local pseudos = extractPseudoProps(props)
    local node = {
        type = nodeType,
        content = overrides.content or "",
        style = overrides.style or props.style or {},
        children = overrides.children or props.children or {},
        hoverStyle = overrides.hoverStyle or pseudos.hoverStyle,
        activeStyle = overrides.activeStyle or pseudos.activeStyle,
        disabled = overrides.disabled or pseudos.disabled or false,
    }
    if props.onClick then node.onClick = props.onClick end
    if overrides.extra then
        for k, v in pairs(overrides.extra) do node[k] = v end
    end
    return node
end

-- ── Modding API ──

--- Define a custom component
--- @param name string        Component name (e.g. "MyButton")
--- @param config table|function
---   If function: called with (props), must return a node
---   If table: { extends="View", style={}, hoverStyle={}, activeStyle={},
---               beforeStyle=fn(style,props), afterStyle=fn(style,props),
---               render=fn(props) -> node }
function Axo.defineComponent(name, config)
    if type(config) == "function" then
        _components[name] = config
        Axo[name] = config
        return
    end
    _components[name] = config
    local parent = config.extends or "View"
    local baseStyle = config.style or {}
    local baseHover = config.hoverStyle
    local baseActive = config.activeStyle
    Axo[name] = function(props)
        props = props or {}
        local mergedStyle = mergeTables(baseStyle, props.style)
        if config.beforeStyle then config.beforeStyle(mergedStyle, props) end
        if config.afterStyle then config.afterStyle(mergedStyle, props) end
        local overrides = { style = mergedStyle }
        if baseHover or props.hoverStyle then
            overrides.hoverStyle = mergeTables(baseHover, props.hoverStyle)
        end
        if baseActive or props.activeStyle then
            overrides.activeStyle = mergeTables(baseActive, props.activeStyle)
        end
        if config.render then
            local node = config.render(props)
            if node then return node end
        end
        return baseNode(props, parent, overrides)
    end
end

--- Alias for defineComponent
Axo.createComponent = Axo.defineComponent

--- Retrieve a registered component (useful for meta-programming)
function Axo.getComponent(name)
    return _components[name] or Axo[name]
end

--- List all registered component names
function Axo.listComponents()
    local names = {}
    for name, _ in pairs(_components) do table.insert(names, name) end
    for name, _ in pairs(Axo) do
        if type(Axo[name]) == "function" and name ~= "defineComponent"
            and name ~= "createComponent" and name ~= "getComponent"
            and name ~= "listComponents" and name ~= "useState"
            and name ~= "Device" then
            table.insert(names, name)
        end
    end
    return names
end

-- ── Layout ──

function Axo.View(props)
    props = props or {}
    return baseNode(props, "View")
end

function Axo.Row(props)
    props = props or {}
    local style = props.style or {}
    style.flexDirection = "row"
    return baseNode(props, "View", {
        style = style,
    })
end

function Axo.Column(props)
    props = props or {}
    return Axo.View(props)
end

function Axo.Center(props)
    props = props or {}
    local style = props.style or {}
    style.justifyContent = "center"
    style.alignItems = "center"
    return baseNode(props, "View", {
        style = style,
    })
end

function Axo.ZStack(props)
    props = props or {}
    local style = props.style or {}
    style.position = "relative"
    -- children will need position=absolute individually
    return baseNode(props, "View", {
        style = style,
    })
end

function Axo.ScrollView(props)
    props = props or {}
    return baseNode(props, "ScrollView")
end

function Axo.SafeAreaView(props)
    props = props or {}
    return baseNode(props, "View")
end

function Axo.Spacer(props)
    props = props or {}
    local size = props.size or "auto"
    return baseNode(props, "View", {
        style = {
            width = props.horizontal and size or "1",
            height = props.horizontal and "1" or size,
            flexGrow = props.flex and 1 or 0,
        },
    })
end

-- ── Display ──

function Axo.Text(props)
    props = props or {}
    return baseNode(props, "Text", {
        content = props.text or props.content or "",
    })
end

function Axo.Button(props)
    props = props or {}
    return baseNode(props, "Button", {
        content = props.text or "",
    })
end

function Axo.Pressable(props)
    props = props or {}
    local node = baseNode(props, "View")
    if props.onPress then node.onClick = props.onPress end
    return node
end

function Axo.Image(props)
    props = props or {}
    return baseNode(props, "Image", {
        content = props.source or "",
    })
end

function Axo.TextInput(props)
    props = props or {}
    local node = baseNode(props, "TextInput", {
        content = props.text or props.value or "",
    })
    if props.onChangeText then node.onChangeText = props.onChangeText end
    return node
end

-- ── Data display ──

function Axo.FlatList(props)
    props = props or {}
    local items = {}
    if props.data then
        for i, item in ipairs(props.data) do
            if props.renderItem then
                local node = props.renderItem({ item = item, index = i - 1 })
                if node then table.insert(items, node) end
            end
        end
    end
    return baseNode(props, "View", { children = items })
end

function Axo.Divider(props)
    props = props or {}
    return baseNode(props, "View", {
        style = mergeTables({
            width = props.vertical and "1" or "100%",
            height = props.vertical and "100%" or "1",
            backgroundColor = props.color or "#333333",
            margin = props.margin or "4",
        }, props.style),
    })
end

-- ── New Components ──

function Axo.Card(props)
    props = props or {}
    local style = props.style or {}
    if not style.backgroundColor then style.backgroundColor = "#1e1e3a" end
    if not style.borderWidth then style.borderWidth = "1" end
    if not style.borderColor then style.borderColor = "#333355" end
    local hoverStyle = props.hoverStyle or { borderColor = "#5555ff" }
    return baseNode(props, "View", {
        style = style,
        hoverStyle = hoverStyle,
    })
end

function Axo.Badge(props)
    props = props or {}
    local s = mergeTables({
        backgroundColor = props.color or "#e94560",
        color = props.textColor or "#ffffff",
        fontSize = props.fontSize or "12",
        padding = "4 8",
        borderRadius = "12",
        alignSelf = "flex-start",
    }, props.style)
    return baseNode(props, "Text", { content = props.text or "", style = s })
end

function Axo.Chip(props)
    props = props or {}
    local s = mergeTables({
        backgroundColor = props.color or "#333355",
        padding = "4 12",
        borderRadius = "16",
        alignSelf = "flex-start",
        margin = props.margin or "2",
    }, props.style)
    local ts = mergeTables({ color = props.textColor or "#ffffff", fontSize = props.fontSize or "13" }, props.textStyle)
    return baseNode(props, "View", {
        style = s,
        children = { Axo.Text({ text = props.text or "", style = ts }) },
    })
end

function Axo.ProgressBar(props)
    props = props or {}
    local progress = math.min(math.max(props.progress or 0, 0), 1)
    local s = mergeTables({
        width = props.width or "100%",
        height = props.height or "8",
        backgroundColor = props.trackColor or "#333355",
        borderRadius = "4",
        overflow = "hidden",
    }, props.style)
    return baseNode(props, "View", {
        style = s,
        children = {
            Axo.View({ style = {
                width = tostring(progress * 100) .. "%",
                height = "100%",
                backgroundColor = props.color or "#e94560",
                borderRadius = "4",
            }}),
        },
    })
end

function Axo.Switch(props)
    props = props or {}
    local isOn = props.value or false
    local s = mergeTables({
        width = "44",
        height = "24",
        backgroundColor = isOn and (props.activeColor or "#e94560") or (props.inactiveColor or "#444466"),
        borderRadius = "12",
        justifyContent = "center",
        margin = props.margin or "4",
    }, props.style)
    return baseNode(props, "View", {
        style = s,
        children = {
            Axo.View({ style = {
                width = "20", height = "20",
                backgroundColor = "#ffffff",
                borderRadius = "10",
                marginLeft = isOn and "22" or "2",
            }}),
        },
    })
end

function Axo.Checkbox(props)
    props = props or {}
    local checked = props.value or false
    local s = mergeTables({
        width = "20", height = "20",
        backgroundColor = checked and (props.activeColor or "#e94560") or (props.inactiveColor or "#333355"),
        borderRadius = "4",
        borderWidth = "2",
        borderColor = checked and (props.activeColor or "#e94560") or (props.borderColor or "#555577"),
        justifyContent = "center",
        alignItems = "center",
    }, props.style)
    return baseNode(props, "View", {
        style = s,
        children = checked and {
            Axo.Text({ text = "✓", style = { color = "#ffffff", fontSize = "14", fontWeight = "bold" }}),
        } or {},
    })
end

function Axo.RadioButton(props)
    props = props or {}
    local selected = props.value or false
    local s = mergeTables({
        width = "20", height = "20",
        backgroundColor = "transparent",
        borderRadius = "10",
        borderWidth = "2",
        borderColor = selected and (props.activeColor or "#e94560") or (props.borderColor or "#555577"),
        justifyContent = "center",
        alignItems = "center",
    }, props.style)
    return baseNode(props, "View", {
        style = s,
        children = selected and {
            Axo.View({ style = {
                width = "10", height = "10",
                backgroundColor = props.activeColor or "#e94560",
                borderRadius = "5",
            }}),
        } or {},
    })
end

function Axo.Slider(props)
    props = props or {}
    local val = props.value or 0.5
    local s = mergeTables({ width = props.width or "200", height = "24", justifyContent = "center" }, props.style)
    return baseNode(props, "View", {
        style = s,
        children = {
            Axo.View({ style = mergeTables(mergeTables({
                width = "100%", height = "4",
                backgroundColor = props.trackColor or "#333355",
                borderRadius = "2",
            }, props.trackStyle), {
                children = {
                    Axo.View({ style = {
                        width = tostring(val * 100) .. "%", height = "100%",
                        backgroundColor = props.color or "#e94560",
                        borderRadius = "2",
                    }}),
                },
            })}),
            Axo.View({ style = mergeTables({
                position = "absolute", width = "16", height = "16",
                backgroundColor = "#ffffff", borderRadius = "8",
                marginLeft = tostring(val * 100 - 8) .. "%",
            }, props.thumbStyle)}),
        },
    })
end

function Axo.List(props)
    props = props or {}
    local items = {}
    if props.data then
        for i, item in ipairs(props.data) do
            if i > 1 then
                table.insert(items, Axo.Divider({ color = props.separatorColor or "#333355", margin = "0" }))
            end
            local row = Axo.View({
                style = { flexDirection = "row", alignItems = "center", padding = props.itemPadding or "12 16" },
                children = { props.renderItem and props.renderItem({ item = item, index = i - 1 }) },
            })
            if props.onItemPress then
                row.onClick = function() props.onItemPress({ item = item, index = i - 1 }) end
            end
            table.insert(items, row)
        end
    end
    return baseNode(props, "View", {
        style = mergeTables({ width = "100%", backgroundColor = props.backgroundColor or "transparent" }, props.style),
        children = items,
    })
end

-- ── Device API wrappers ──

function Axo.Device()
    return _G.Device
end

function Axo.getDeviceInfo()
    if _G.Device then return _G.Device.info() end
    return {}
end

function Axo.getSystemInfo()
    if _G.Device then return _G.Device.getSystemInfo() end
    return {}
end

function Axo.getDisplayInfo()
    if _G.Device then return _G.Device.getDisplayInfo() end
    return {}
end

function Axo.getBatteryInfo()
    if _G.Device then return _G.Device.getBatteryInfo() end
    return {}
end

function Axo.getNetworkInfo()
    if _G.Device then return _G.Device.getNetworkInfo() end
    return {}
end

function Axo.getPlatform()
    if _G.Device then return _G.Device.getPlatform() end
    return {}
end

function Axo.checkPermission(name)
    if _G.Device then return _G.Device.checkPermission(name) end
    return "denied"
end

function Axo.requestPermission(name)
    if _G.Device then return _G.Device.requestPermission(name) end
    return "denied"
end

function Axo.getPermissionInfo(name)
    if _G.Device then return _G.Device.getPermissionInfo(name) end
    return {}
end

function Axo.openSettings()
    if _G.Device then return _G.Device.openSettings() end
    return false
end

function Axo.getLocation()
    if _G.Device then return _G.Device.getLocation() end
    return nil
end

function Axo.getSensors()
    if _G.Device then return _G.Device.getSensors() end
    return { accelerometer = nil, gyroscope = nil, magnetometer = nil }
end

function Axo.readFile(path)
    if _G.Device then return _G.Device.readFile(path) end
    return nil
end

function Axo.writeFile(path, content)
    if _G.Device then return _G.Device.writeFile(path, content) end
    return false
end

function Axo.deleteFile(path)
    if _G.Device then return _G.Device.deleteFile(path) end
    return false
end

function Axo.fileExists(path)
    if _G.Device then return _G.Device.fileExists(path) end
    return false
end

function Axo.listFiles(path)
    if _G.Device then return _G.Device.listFiles(path) end
    return nil
end

function Axo.storagePath()
    if _G.Device then return _G.Device.storagePath() end
    return ""
end

function Axo.cacheDir()
    if _G.Device then return _G.Device.cacheDir() end
    return ""
end

function Axo.tempDir()
    if _G.Device then return _G.Device.tempDir() end
    return ""
end

function Axo.documentsDir()
    if _G.Device then return _G.Device.documentsDir() end
    return ""
end

function Axo.showNotification(title, body)
    if _G.Device then return _G.Device.showNotification(title, body) end
    return false
end

function Axo.scheduleNotification(title, body, delayMs)
    if _G.Device then return _G.Device.scheduleNotification(title, body, delayMs) end
    return false
end

function Axo.takePhoto()
    if _G.Device then return _G.Device.takePhoto() end
    return nil
end

function Axo.setClipboard(text)
    if _G.Device then return _G.Device.setClipboard(text) end
    return false
end

function Axo.getClipboard()
    if _G.Device then return _G.Device.getClipboard() end
    return nil
end

function Axo.hasClipboard()
    if _G.Device then return _G.Device.hasClipboard() end
    return false
end

function Axo.vibrate(durationMs)
    if _G.Device then return _G.Device.vibrate(durationMs) end
    return false
end

function Axo.hapticImpact(style)
    if _G.Device then return _G.Device.hapticImpact(style or "medium") end
    return false
end

function Axo.showAlert(title, message, buttons)
    if _G.Device then return _G.Device.showAlert(title, message, buttons or { "OK" }) end
    return 0
end

function Axo.showConfirm(title, message)
    if _G.Device then return _G.Device.showConfirm(title, message) end
    return false
end

function Axo.showPrompt(title, message, defaultText)
    if _G.Device then return _G.Device.showPrompt(title, message, defaultText or "") end
    return ""
end

function Axo.keepScreenOn(keep)
    if _G.Device then return _G.Device.keepScreenOn(keep) end
    return false
end

function Axo.setBrightness(level)
    if _G.Device then return _G.Device.setBrightness(level) end
    return false
end

return Axo

"##;

const APP_LUA: &str = r##"-- Define custom components using the modding API
UI.defineComponent("PrimaryButton", {
    extends = "Button",
    style = {
        backgroundColor = "#e94560",
        color = "#ffffff",
        borderRadius = "8",
        height = "48",
        width = "220",
    },
    hoverStyle = { backgroundColor = "#ff6b6b" },
    activeStyle = { backgroundColor = "#c0392b" },
})

UI.defineComponent("OutlineButton", {
    extends = "Button",
    style = {
        backgroundColor = "transparent",
        borderRadius = "8",
        height = "40",
        borderWidth = "1",
    },
    hoverStyle = { backgroundColor = "#ffffff" },
})

-- State
local counter = UI.useState("counter", 0)
local darkMode = UI.useState("darkMode", true)

-- Handlers as globals for onClick="stringName"
_G.handleIncrement = function() counter.set(counter.get() + 1) end
_G.handleDecrement = function() counter.set(counter.get() - 1) end
_G.handleReset = function() counter.set(0) end
_G.toggleDark = function() darkMode.set(not darkMode.get()) end

function App()
    local isDark = darkMode.get()
    local bg = isDark and "#1a1a2e" or "#f5f5f5"
    local tc = isDark and "#ffffff" or "#1a1a2e"
    local cardBg = isDark and "#16213e" or "#ffffff"

    return UI.Center({
        style = { width = "100%", height = "100%", backgroundColor = bg, padding = "20" },
        children = {
            UI.Text({ text = "Axo Framework", style = { fontSize = "32", color = "#e94560" } }),
            UI.Text({ text = "Build UI with Rust + Lua + wgpu", style = { fontSize = 14, color = tc } }),

            UI.Spacer({ size = "24" }),

            UI.Card({
                style = { backgroundColor = cardBg, padding = "24" },
                children = {
                    UI.Text({ text = "Count: " .. counter.get(), style = { fontSize = 36, color = "#e94560", textAlign = "center" } }),
                }
            }),

            UI.Spacer({ size = "16" }),

            UI.PrimaryButton({ text = "Increment", onClick = "handleIncrement" }),
            UI.PrimaryButton({ text = "Decrement", onClick = "handleDecrement" }),
            UI.PrimaryButton({ text = "Reset", disabled = counter.get() == 0, onClick = "handleReset" }),

            UI.Spacer({ size = "16" }),

            UI.OutlineButton({
                text = isDark and "☀ Light Mode" or "☾ Dark Mode",
                onClick = "toggleDark",
                style = { borderColor = tc, color = tc },
            }),

            UI.Spacer({ size = "24" }),

            UI.Text({ text = "Edit app.lua to get started!", style = { fontSize = 11, color = tc } }),
        },
    })
end

return App
"##;

const AXO_JSON: &str = r#"{
    "name": "my-app",
    "version": "0.1.0",
    "entry": "app/app.lua",
    "modules": [
        "components"
    ]
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
  app.lua         -- Entry point
  axo/init.lua    -- Framework library (components + device API)
  components/     -- Your custom components (auto-loaded)
README.md
axo.json         -- Project config
```

## Modding API

Create reusable components without touching Rust:

```lua
UI.defineComponent("MyButton", {
    extends = "Button",
    style = { backgroundColor = "#e94560", borderRadius = "8" },
    hoverStyle = { backgroundColor = "#ff6b6b" },
    activeStyle = { backgroundColor = "#c0392b" },
})
```

Then use them like built-in components:

```lua
UI.MyButton({ text = "Click me", onClick = "handleClick" })
```
"##;

const EXAMPLE_COMPONENT: &str = r##"-- Example: define a reusable component using the modding API
-- Import axo UI library
local UI = require("axo")

-- Define a custom button with defaults
UI.defineComponent("PrimaryButton", {
    extends = "Button",
    style = {
        backgroundColor = "#e94560",
        color = "#ffffff",
        borderRadius = "8",
        height = "48",
        padding = "0 24",
    },
    hoverStyle = { backgroundColor = "#ff6b6b" },
    activeStyle = { backgroundColor = "#c0392b" },
})

return UI
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

    // Core app files
    create_file(project_dir, "app/app.lua", APP_LUA);
    create_file(project_dir, "app/axo/init.lua", INIT_LUA);

    // Components directory (auto-loaded via package.path)
    create_file(project_dir, "app/components/primary-button.lua", EXAMPLE_COMPONENT);

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
    println!("New in this version:");
    println!("  - defineComponent/createComponent API for modding");
    println!("  - auto-loading components from app/components/");
    println!("  - mergeTables style merging in all components");
    println!("  - install via: curl -fsSL https://raw.githubusercontent.com/Erick-arch-bit/Axo-Framework/dev/scripts/install.sh | sh");
}
