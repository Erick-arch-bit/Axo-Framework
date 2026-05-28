local Axo = {}

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
