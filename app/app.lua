local UI = require("axo")

function handleIncrement()
    log("Increment clicked!")
end

function handleDecrement()
    log("Decrement clicked!")
end

function handleReset()
    log("Reset clicked!")
end

function App()
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
            UI.Text("Axo Framework", {
                fontSize = 28,
                color = "#ffffff",
            }),
            UI.Text("Haz clic en un boton", {
                fontSize = 16,
                color = "#888888",
            }),
            UI.Button({
                text = "Incrementar",
                onClick = "handleIncrement",
                style = {
                    backgroundColor = "#e94560",
                    width = "200",
                    height = "50",
                    margin = "10",
                },
            }),
            UI.Button({
                text = "Decrementar",
                onClick = "handleDecrement",
                style = {
                    backgroundColor = "#0f3460",
                    width = "200",
                    height = "50",
                    margin = "10",
                },
            }),
            UI.Button({
                text = "Reset",
                onClick = "handleReset",
                style = {
                    backgroundColor = "#16213e",
                    width = "200",
                    height = "50",
                    margin = "10",
                },
            }),
        },
    })
end

return App
