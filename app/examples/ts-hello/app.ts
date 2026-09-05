function App() {
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
      UI.Text("Hola desde TypeScript", {
        fontSize: 24,
        color: "#ffffff",
      }),
    ],
  });
}

App;
