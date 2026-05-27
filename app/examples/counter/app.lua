local UI = require("lumina")

local state = { count = 0 }

function onIncrement()
    state.count = state.count + 1
    print("Count:", state.count)
end

function App()
    return UI.View({
        style = {
            backgroundColor = "#1a1a1a",
            width = "100%",
            height = "100%",
            justifyContent = "center",
            alignItems = "center",
        },
        children = {
            UI.Text("Contador: " .. state.count, {
                fontSize = 32,
                color = "#ffffff",
            }),
            UI.Button({
                text = "Incrementar",
                onClick = "onIncrement",
                style = {
                    backgroundColor = "#e94560",
                    width = "200",
                    height = "50",
                    margin = "10",
                },
            }),
        },
    })
end

return App
