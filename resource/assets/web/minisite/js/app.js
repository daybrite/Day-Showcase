// Day mini site — loaded through a relative <script src>, proving bundled JS runs.
(function () {
  var clicks = 0;
  var btn = document.getElementById("count");
  if (btn) {
    btn.addEventListener("click", function () {
      clicks += 1;
      btn.textContent = "Clicked " + clicks + (clicks === 1 ? " time" : " times");
    });
  }
  var year = document.getElementById("year");
  if (year) {
    year.textContent = String(new Date().getFullYear());
  }
})();
