function App() {
  return {
    type: "View",
    style: {
      width: "100%",
      height: "100%",
      backgroundColor: "#1a1a2e",
      flexDirection: "column",
      justifyContent: "center",
      alignItems: "center"
    },
    children: [
      {
        type: "Text",
        content: "Hola desde QuickJS",
        style: {
          fontSize: 24,
          color: "#ffffff"
        }
      }
    ]
  };
}

// Fase 1: devolvemos la función para que load_js_app la invoque
App;
