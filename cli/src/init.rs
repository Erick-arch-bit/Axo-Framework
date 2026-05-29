use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct InitArgs {
    #[arg(default_value = "my-axo-app")]
    pub name: String,
}

const INIT_LUA: &str = r##"local Axo = {}

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

local function extractPseudoProps(props)
    return {
        hoverStyle = props.hoverStyle,
        activeStyle = props.activeStyle,
        disabled = props.disabled,
    }
end

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

function Axo.View(props)
    props = props or {}
    return baseNode(props, "View")
end

function Axo.Row(props)
    props = props or {}
    local style = props.style or {}
    style.flexDirection = "row"
    return baseNode(props, "View", { style = style })
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
    return baseNode(props, "View", { style = style })
end

function Axo.ZStack(props)
    props = props or {}
    local style = props.style or {}
    style.position = "relative"
    return baseNode(props, "View", { style = style })
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

function Axo.Text(props)
    props = props or {}
    return baseNode(props, "Text", { content = props.text or props.content or "" })
end

function Axo.Button(props)
    props = props or {}
    return baseNode(props, "Button", { content = props.text or "" })
end

function Axo.Pressable(props)
    props = props or {}
    local node = baseNode(props, "View")
    if props.onPress then node.onClick = props.onPress end
    return node
end

function Axo.Image(props)
    props = props or {}
    return baseNode(props, "Image", { content = props.source or "" })
end

function Axo.TextInput(props)
    props = props or {}
    local node = baseNode(props, "TextInput", {
        content = props.text or props.value or "",
    })
    if props.onChangeText then node.onChangeText = props.onChangeText end
    return node
end

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
        style = {
            width = props.vertical and "1" or "100%",
            height = props.vertical and "100%" or "1",
            backgroundColor = props.color or "#333333",
            margin = props.margin or "4",
        },
    })
end

function Axo.Card(props)
    props = props or {}
    local style = props.style or {}
    if not style.backgroundColor then style.backgroundColor = "#1e1e3a" end
    if not style.borderWidth then style.borderWidth = "1" end
    if not style.borderColor then style.borderColor = "#333355" end
    return baseNode(props, "View", {
        style = style,
        hoverStyle = props.hoverStyle or { borderColor = "#5555ff" },
    })
end

function Axo.Badge(props)
    props = props or {}
    return baseNode(props, "Text", {
        content = props.text or "",
        style = {
            backgroundColor = props.color or "#e94560",
            color = props.textColor or "#ffffff",
            fontSize = props.fontSize or "12",
            padding = "4 8",
            borderRadius = "12",
            alignSelf = "flex-start",
        },
    })
end

function Axo.Chip(props)
    props = props or {}
    return baseNode(props, "View", {
        style = {
            backgroundColor = props.color or "#333355",
            padding = "4 12",
            borderRadius = "16",
            alignSelf = "flex-start",
            margin = props.margin or "2",
        },
        children = { Axo.Text({
            text = props.text or "",
            style = { color = props.textColor or "#ffffff", fontSize = props.fontSize or "13" },
        })},
    })
end

function Axo.ProgressBar(props)
    props = props or {}
    local progress = math.min(math.max(props.progress or 0, 0), 1)
    return baseNode(props, "View", {
        style = {
            width = props.width or "100%",
            height = props.height or "8",
            backgroundColor = props.trackColor or "#333355",
            borderRadius = "4",
            overflow = "hidden",
        },
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
    return baseNode(props, "View", {
        style = {
            width = "44", height = "24",
            backgroundColor = isOn and (props.activeColor or "#e94560") or (props.inactiveColor or "#444466"),
            borderRadius = "12",
            justifyContent = "center",
            margin = props.margin or "4",
        },
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
    return baseNode(props, "View", {
        style = {
            width = "20", height = "20",
            backgroundColor = checked and (props.activeColor or "#e94560") or (props.inactiveColor or "#333355"),
            borderRadius = "4",
            borderWidth = "2",
            borderColor = checked and (props.activeColor or "#e94560") or (props.borderColor or "#555577"),
            justifyContent = "center",
            alignItems = "center",
        },
        children = checked and {
            Axo.Text({ text = "\226\156\145", style = { color = "#ffffff", fontSize = "14" }}),
        } or {},
    })
end

function Axo.RadioButton(props)
    props = props or {}
    local selected = props.value or false
    return baseNode(props, "View", {
        style = {
            width = "20", height = "20",
            backgroundColor = "transparent",
            borderRadius = "10",
            borderWidth = "2",
            borderColor = selected and (props.activeColor or "#e94560") or (props.borderColor or "#555577"),
            justifyContent = "center",
            alignItems = "center",
        },
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
    return baseNode(props, "View", {
        style = { width = props.width or "200", height = "24", justifyContent = "center" },
        children = {
            Axo.View({ style = {
                width = "100%", height = "4",
                backgroundColor = props.trackColor or "#333355",
                borderRadius = "2",
                children = {
                    Axo.View({ style = {
                        width = tostring(val * 100) .. "%", height = "100%",
                        backgroundColor = props.color or "#e94560",
                        borderRadius = "2",
                    }}),
                },
            }}),
            Axo.View({ style = {
                position = "absolute", width = "16", height = "16",
                backgroundColor = "#ffffff", borderRadius = "8",
                marginLeft = tostring(val * 100 - 8) .. "%",
            }}),
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
            local row = baseNode(props, "View", {
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
        style = { width = "100%", backgroundColor = props.backgroundColor or "transparent" },
        children = items,
    })
end

-- Device API wrappers
function Axo.Device() return _G.Device end
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

const APP_LUA: &str = r##"local UI = require("axo")

local count = UI.useState("counter", 0)
local darkMode = UI.useState("darkMode", true)
local deviceInfo = UI.useState("deviceInfo", nil)

function handleIncrement()
    count.set(count.get() + 1)
end

function handleDecrement()
    count.set(count.get() - 1)
end

function handleReset()
    count.set(0)
end

function toggleDarkMode()
    darkMode.set(not darkMode.get())
end

function showDeviceInfo()
    local info = UI.getDeviceInfo()
    UI.showAlert("Device Info",
        "OS: " .. (info.os_name or "unknown") .. "\n" ..
        "Version: " .. (info.os_version or "") .. "\n" ..
        "Screen: " .. (info.screen_width or 0) .. "x" .. (info.screen_height or 0) .. "\n" ..
        "Mobile: " .. tostring(info.is_mobile or false) .. "\n" ..
        "Language: " .. (info.language or "")
    )
end

function App()
    local isDark = darkMode.get()
    local bg = isDark and "#1a1a2e" or "#f0f0f0"
    local textColor = isDark and "#ffffff" or "#1a1a2e"
    local accent = "#e94560"
    local cardBg = isDark and "#16213e" or "#ffffff"
    local mutedBg = isDark and "#0f3460" or "#e0e0e0"

    return UI.View({
        style = {
            width = "100%",
            height = "100%",
            backgroundColor = bg,
            flexDirection = "column",
            alignItems = "center",
            justifyContent = "center",
            padding = "20",
        },
        children = {
            -- Title
            UI.Text({
                text = "Axo Framework",
                style = { fontSize = 32, color = accent, margin = "4" },
            }),

            -- Subtitle
            UI.Text({
                text = "CSS-like styling with hover/active",
                style = { fontSize = 14, color = textColor, margin = "4", opacity = "0.7" },
            }),

            UI.Spacer({ size = "20" }),

            -- Counter card
            UI.Card({
                backgroundColor = cardBg,
                padding = "24",
                children = {
                    UI.Text({
                        text = "Contador: " .. count.get(),
                        style = { fontSize = 28, color = accent, textAlign = "center" },
                    }),
                },
            }),

            UI.Spacer({ size = "16" }),

            -- Buttons with hover/active styles
            UI.Button({
                text = "Incrementar",
                onClick = "handleIncrement",
                style = { backgroundColor = accent, width = "220", height = "48", margin = "4", borderRadius = "8" },
                hoverStyle = { backgroundColor = "#ff6b6b" },
                activeStyle = { backgroundColor = "#c0392b" },
            }),
            UI.Button({
                text = "Decrementar",
                onClick = "handleDecrement",
                style = { backgroundColor = mutedBg, width = "220", height = "48", margin = "4", borderRadius = "8", color = textColor },
                hoverStyle = { backgroundColor = "#1a4a7a" },
                activeStyle = { backgroundColor = "#0a2a4a" },
            }),
            UI.Button({
                text = "Reset",
                onClick = "handleReset",
                style = { backgroundColor = "#333355", width = "220", height = "48", margin = "4", borderRadius = "8" },
                hoverStyle = { backgroundColor = "#4444aa" },
                activeStyle = { backgroundColor = "#222266" },
                disabled = count.get() == 0,
            }),

            UI.Spacer({ size = "16" }),

            -- Toggle and info buttons
            UI.Button({
                text = isDark and "☀ Modo Claro" or "☾ Modo Oscuro",
                onClick = "toggleDarkMode",
                style = { backgroundColor = "transparent", width = "220", height = "40", margin = "2", borderWidth = "1", borderColor = textColor, color = textColor },
                hoverStyle = { backgroundColor = textColor, color = bg },
            }),

            UI.Button({
                text = "📱 Info del Dispositivo",
                onClick = "showDeviceInfo",
                style = { backgroundColor = "transparent", width = "220", height = "40", margin = "2", color = textColor },
                hoverStyle = { color = accent },
            }),

            UI.Spacer({ size = "12" }),

            -- Platform info
            UI.Text({
                text = "Rust + Lua + wgpu + Flexbox",
                style = { fontSize = 11, color = textColor, margin = "4", opacity = "0.5" },
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
axo-cli dev
```

## Build

```bash
axo-cli build
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
    println!("  axo-cli dev");
}
