import { useEffect, useState } from "react";

function App() {
  const [message, setMessage] = useState("");
  useEffect(() => {
    // Create object
    const socket = new WebSocket("ws://127.0.0.1:5000");
    // Connect server
    socket.addEventListener("open", (_) => {
      socket.send("Hello from client!");
    });
    // Listen for message
    socket.addEventListener("message", (event) => {
      console.log(event.data);
      setMessage(event.data);
    });
  }, []);
  return (
    <>
      <h1>axum-websocket-mytutorial</h1>
      <p>Recieved Message: {message}</p>
    </>
  );
}

export default App;
