(function () {
  var runningInference = false;

  function el(id) {
    return document.getElementById(id);
  }

  function setStatus(msg) {
    el("status").textContent = msg || "";
  }

  function pill(sentiment) {
    switch (sentiment) {
      case "positive":
        return '<span class="pill positive">Positive</span>';
      case "negative":
        return '<span class="pill negative">Negative</span>';
      case "neutral":
        return '<span class="pill neutral">Neutral</span>';
      default:
        return '<span class="pill error">Unsure</span>';
    }
  }

  function analyze() {
    if (runningInference) {
      setStatus("Already running inference, please wait…");
      return;
    }

    var input = el("sentence-input");
    var sentence = input.value.trim();
    if (sentence === "") {
      setStatus("Please enter some text to analyze.");
      return;
    }

    setStatus("");
    input.value = "";

    // Insert a card with a loading indicator at the top of the results.
    var card = document.createElement("div");
    card.className = "result";
    card.innerHTML =
      '<span class="sentence"></span><span class="pill loading">Analyzing…</span>';
    card.querySelector(".sentence").textContent = sentence;
    var results = el("results");
    results.insertBefore(card, results.firstChild);

    runningInference = true;
    el("analyze-btn").disabled = true;

    fetch("/api/sentiment-analysis", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ sentence: sentence }),
    })
      .then(function (response) {
        if (!response.ok) {
          throw new Error("Request failed with status " + response.status);
        }
        return response.json();
      })
      .then(function (data) {
        card.querySelector(".pill").outerHTML = pill(data.sentiment);
      })
      .catch(function (error) {
        console.error(error);
        card.querySelector(".pill").outerHTML =
          '<span class="pill error">Failed</span>';
        setStatus("Something went wrong. Check that the Ollama server is reachable.");
      })
      .finally(function () {
        runningInference = false;
        el("analyze-btn").disabled = false;
      });
  }

  function clearResults() {
    el("results").innerHTML = "";
    el("sentence-input").value = "";
    setStatus("");
  }

  document.addEventListener("DOMContentLoaded", function () {
    el("analyze-btn").addEventListener("click", analyze);
    el("clear-btn").addEventListener("click", clearResults);

    // Submit on Ctrl/Cmd + Enter from the textarea.
    el("sentence-input").addEventListener("keydown", function (event) {
      if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
        analyze();
      }
    });
  });
})();
