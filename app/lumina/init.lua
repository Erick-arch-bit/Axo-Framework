local Lumina = {}

function Lumina.View(props)
    return {
        type = "View",
        style = props.style or {},
        children = props.children or {},
        content = "",
    }
end

function Lumina.Text(content, style)
    return {
        type = "Text",
        content = content or "",
        style = style or {},
        children = {},
    }
end

function Lumina.Button(props)
    return {
        type = "Button",
        content = props.text or "",
        style = props.style or {},
        children = {},
    }
end

function Lumina.Image(props)
    return {
        type = "Image",
        content = props.source or "",
        style = props.style or {},
        children = {},
    }
end

function Lumina.ScrollView(props)
    return {
        type = "ScrollView",
        style = props.style or {},
        children = props.children or {},
        content = "",
    }
end

function Lumina.TextInput(props)
    return {
        type = "TextInput",
        content = props.value or "",
        style = props.style or {},
        children = {},
    }
end

-- Device API — wraps the Rust `Device` global for convenience
function Lumina.Device()
    return _G.Device
end

function Lumina.getDeviceInfo()
    if _G.Device then
        return _G.Device.info()
    end
    return {}
end

function Lumina.checkPermission(name)
    if _G.Device then
        return _G.Device.checkPermission(name)
    end
    return "denied"
end

function Lumina.requestPermission(name)
    if _G.Device then
        return _G.Device.requestPermission(name)
    end
    return "denied"
end

function Lumina.getLocation()
    if _G.Device then
        return _G.Device.getLocation()
    end
    return nil
end

function Lumina.getSensors()
    if _G.Device then
        return _G.Device.getSensors()
    end
    return { accelerometer = nil, gyroscope = nil, magnetometer = nil }
end

function Lumina.readFile(path)
    if _G.Device then
        return _G.Device.readFile(path)
    end
    return nil
end

function Lumina.writeFile(path, content)
    if _G.Device then
        return _G.Device.writeFile(path, content)
    end
    return false
end

function Lumina.deleteFile(path)
    if _G.Device then
        return _G.Device.deleteFile(path)
    end
    return false
end

function Lumina.fileExists(path)
    if _G.Device then
        return _G.Device.fileExists(path)
    end
    return false
end

function Lumina.storagePath()
    if _G.Device then
        return _G.Device.storagePath()
    end
    return ""
end

function Lumina.showNotification(title, body)
    if _G.Device then
        return _G.Device.showNotification(title, body)
    end
    return false
end

function Lumina.takePhoto()
    if _G.Device then
        return _G.Device.takePhoto()
    end
    return nil
end

return Lumina
