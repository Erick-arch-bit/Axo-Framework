local UI = require("axo")

local count = UI.useState("counter", 0)
local darkMode = UI.useState("darkMode", true)

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
            UI.Text({
                text = "Axo Framework",
                style = { fontSize = 32, color = accent, margin = "4" },
            }),
            UI.Text({
                text = "CSS-like styling with hover/active",
                style = { fontSize = 14, color = textColor, margin = "4" },
            }),
            UI.Spacer({ size = "20" }),
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
            UI.Button({
                text = "Incrementar",
                onClick = "handleIncrement",
                style = { backgroundColor = accent, width = "220", height = "48", margin = "4" },
                hoverStyle = { backgroundColor = "#ff6b6b" },
                activeStyle = { backgroundColor = "#c0392b" },
            }),
            UI.Button({
                text = "Decrementar",
                onClick = "handleDecrement",
                style = { backgroundColor = mutedBg, width = "220", height = "48", margin = "4", color = textColor },
                hoverStyle = { backgroundColor = "#1a4a7a" },
                activeStyle = { backgroundColor = "#0a2a4a" },
            }),
            UI.Button({
                text = "Reset",
                onClick = "handleReset",
                style = { backgroundColor = "#333355", width = "220", height = "48", margin = "4" },
                hoverStyle = { backgroundColor = "#4444aa" },
                activeStyle = { backgroundColor = "#222266" },
                disabled = count.get() == 0,
            }),
            UI.Spacer({ size = "16" }),
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
            UI.Text({
                text = "Rust + Lua + wgpu + Flexbox",
                style = { fontSize = 11, color = textColor, margin = "4" },
            }),
        },
    })
end

return App
