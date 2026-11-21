// Day mini site: loaded through a relative <script src>, proving bundled JS runs.
(function () {
  var clicks = 0;
  var btn = document.getElementById("count");
  if (btn) {
    btn.addEventListener("click", function () {
      clicks += 1;
      btn.textContent = "Clicked " + clicks + (clicks === 1 ? " time" : " times");
    });
  }
  // The app answers each message; the promise resolves with its reply (docs/webview.md).
  var sent = 0;
  var send = document.getElementById("send");
  var reply = document.getElementById("reply");
  if (send && reply) {
    send.addEventListener("click", function () {
      if (typeof window.dayMessage !== "function") {
        reply.textContent = "This engine has no channel to the app.";
        return;
      }
      sent += 1;
      window.dayMessage("showcase", "Hello from the page, " + sent).then(function (text) {
        reply.textContent = text;
      });
    });
  }
  var year = document.getElementById("year");
  if (year) {
    year.textContent = String(new Date().getFullYear());
  }
})();
