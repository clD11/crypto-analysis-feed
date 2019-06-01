console.log("Running Index");

const URL = "wss://localhost:8081";

(function() {
    let websocket = new WebSocket(URL);
    websocket.onmessage = function (event) {
        console.log(event.data);
        let node = document.createElement("div");
        node.append(event.data);
        document.getElementById("raw_tweet")
            .appendChild(node);
    };
})();