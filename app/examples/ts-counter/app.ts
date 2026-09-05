function App(): any {
  const [count, setCount]: [any, (next: any) => void] = useState("count", 0);

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
      UI.Text("Contador: " + (count as number), {
        fontSize: 24,
        color: "#ffffff"
      }),
      UI.Button({
        text: "Incrementar",
        onClick: function (): void {
          setCount((count as number) + 1);
          console.log("count =", (count as number) + 1);
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
