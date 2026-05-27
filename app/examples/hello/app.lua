local UI = require("lumina")

function onHelloClick()
    print("Hola desde Lumina Framework!")
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
            UI.Text("Hola desde Lumina Framework", {
                fontSize = 24,
                color = "#ffffff",
            }),
            UI.Button({
                text = "Presioname",
                onClick = "onHelloClick",
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
