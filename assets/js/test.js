document.addEventListener('readystatechange', function(event) {
    if (document.readyState === "complete") {
        onload();
    }
});

function onload(){
    const itemEjemplo = document.getElementById("ejemplo");
    var eventSource = new EventSource("/sse");
    eventSource.onmessage = function (e) {
        var newElement = document.createElement("li");

        newElement.innerHTML = "message: " + e.data;
        ejemplo.appendChild(newElement);
    };
}
