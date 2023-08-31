document.addEventListener('readystatechange', function(event) {
    if (document.readyState === "complete") {
        onload();
    }
});

function onload(){
    // Create WebSocket connection.
    const socket = new WebSocket("ws://localhost:6969/websocket");

    // Connection opened
    socket.addEventListener("open", (event) => {
      socket.send("Hello Server!");
    });

    // Listen for messages
    socket.addEventListener("message", (event) => {
      console.log("Message from server ", event.data);
    });
}
