function App() {
  const info = Device.info();
  const label = "OS: " + (info.os_name || info.os || "unknown");

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
      UI.Text(label, { fontSize: 20, color: "#ffffff" }),
      UI.Button({
        text: "Probar click",
        onClick: () => {
          console.log("click real desde JS");
          Device.showNotification("Axo", "Click OK");
        },
        style: {
          backgroundColor: "#e94560",
          width: 220,
          height: 48,
        },
      }),
    ],
  });
}

App;
