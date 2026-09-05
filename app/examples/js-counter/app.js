function App() {
  const [count, setCount] = useState("count", 0);

  return UI.View({
    style: {
      width: "100%",
      height: "100%",
      backgroundColor: "#1a1a2e",
      flexDirection: "column",
      justifyContent: "center",
      alignItems: "center"
    },
    children: [
      UI.Text("Contador: " + count, {
        fontSize: 24,
        color: "#ffffff"
      }),
      UI.Button({
        text: "Incrementar",
        onClick: function () {
          setCount(count + 1);
          console.log("count =", count + 1);
        },
        style: {
          backgroundColor: "#e94560",
          width: 200,
          height: 50,
          margin: 10
        }
      })
    ]
  });
}

App;
