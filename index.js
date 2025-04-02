import init, { get_plays } from "./pkg/bee_solver.js";

class BeeSolver extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.innerHTML = `
      <form id="inputForm">
        <div>
        <label for="centerLetter">Center Letter:</label>
        <input type="text" id="centerLetter" required minlength="1" maxlength="1" placeholder="o">
        </div>
        <div>
        <label for="outerLetters">Outer Letters:</label>
        <input type="text" id="outerLetters" required minlength="6" maxlength="6" placeholder="zntcia">
        </div>
      </form>
      <div id="results"></div>
    `;

    this.form = this.shadowRoot.getElementById("inputForm");
    this.centerLetterInput = this.shadowRoot.getElementById("centerLetter");
    this.outerLettersInput = this.shadowRoot.getElementById("outerLetters");
    this.resultsContainer = this.shadowRoot.getElementById("results");

    this.updateTable = this.updateTable.bind(this);
  }

  connectedCallback() {
    this.form.addEventListener("input", this.updateTable);
  }

  disconnectedCallback() {
    this.form.removeEventListener("input", this.updateTable);
  }

  async updateTable() {
    await init();
    const centerLetter = this.centerLetterInput.value.trim();
    const outerLetters = this.outerLettersInput.value.trim();

    if (!centerLetter || !outerLetters || outerLetters.length != 6) return;

    const plays = get_plays(centerLetter, outerLetters);

    // Group plays by score
    const groupedByScore = plays.reduce((acc, play) => {
      if (!acc[play.score]) {
        acc[play.score] = [];
      }
      acc[play.score].unshift(play);
      return acc;
    }, {});

    // Clear existing results
    this.resultsContainer.innerHTML = "";

    // Create a new table
    const table = document.createElement("table");
    table.border = "0";

    // Populate table rows
    for (const [score, plays] of Object.entries(groupedByScore).reverse()) {
      const row = table.insertRow();
      const scoreCell = row.insertCell();
      scoreCell.textContent = score;

      const wordCell = row.insertCell();
      wordCell.innerHTML = plays
        .map((play) =>
          play.is_pangram ? `<strong>${play.word}</strong>` : play.word
        )
        .join(", ");
    }

    // Append the table to the results container
    this.resultsContainer.appendChild(table);
  }
}

customElements.define("bee-solver", BeeSolver);
