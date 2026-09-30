// Listen for the Enter key being pressed
document.addEventListener("keydown", function(event) {
  if (event.keyCode === 13) {
    newCard();
  }
});

var globalCardCount = 0;
var runningInference = false;

function newCard() {
  if (runningInference) {
    console.log("Already running inference, please wait...");
    setAlert("Already running inference, please wait...");
    return;
  }
  var inputElement = document.getElementById("sentence-input");
  var sentence = inputElement.value;
  if (sentence === "") {
    console.log("Please enter a sentence to analyze");
    setAlert("Please enter a sentence to analyze");
    return;
  }
  inputElement.value = "";

  var cardIndex = globalCardCount;
  globalCardCount++;
  var newCard = document.createElement("div");
  newCard.id = "card-" + cardIndex;
  newCard.className = "result-card";
  newCard.innerHTML = `
    <span class="sentence">${sentence}</span>
    <span class="dots"></span>
    `;
  var responses = document.getElementById("responses");
  responses.prepend(newCard);

  console.log("Running inference on sentence: " + sentence);
  runningInference = true;
  fetch("/api/sentiment-analysis", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ sentence: sentence }),
  })
    .then((response) => response.json())
    .then((data) => {
      console.log(data);
      updateCard(cardIndex, sentence, data.sentiment);
    })
    .catch((error) => {
      console.log(error);
      runningInference = false;
    });
}

function updateCard(cardIndex, sentence, sentiment) {
  var badge = "";
  if (sentiment === "positive") {
    badge = `<span class="result-badge positive">Positive</span>`;
  } else if (sentiment === "negative") {
    badge = `<span class="result-badge negative">Negative</span>`;
  } else if (sentiment === "neutral") {
    badge = `<span class="result-badge neutral">Neutral</span>`;
  } else {
    badge = `<span class="result-badge unsure">Unsure</span>`;
  }
  var cardElement = document.getElementById("card-" + cardIndex);
  cardElement.innerHTML = `
    <span class="sentence">${sentence}</span>
    ${badge}
    `;
  runningInference = false;
}

function clearContext() {
  document.getElementById("responses").innerHTML = "";
  document.getElementById("alert").innerHTML = "";
  globalCardCount = 0;
}

function setAlert(msg) {
  var alertElement = document.getElementById("alert");
  alertElement.innerHTML = `<div class="alert">${msg}</div>`;
  setTimeout(function() {
    alertElement.innerHTML = "";
  }, 3000);
}
