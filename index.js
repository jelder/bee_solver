import init, { get_plays } from "./pkg/bee_solver.js";

class BeeSolver extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.innerHTML = `
      <form id="inputForm">
        <div>
        <label for="core">Center:</label>
        <input type="text" id="core" required pattern="[A-Za-z]" size="1" minlength="1" maxlength="1" placeholder="o">
        </div>
        <div>
        <label for="ring">Ring:</label>
        <input type="text" id="ring" required pattern="[A-Za-z]" size="6" minlength="6" maxlength="6" placeholder="zntcia">
        </div>
      </form>
      <div id="results"></div>
    `;

    this.form = this.shadowRoot.getElementById("inputForm");
    this.coreInput = this.shadowRoot.getElementById("core");
    this.ringInput = this.shadowRoot.getElementById("ring");
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
    const core = this.coreInput.value.trim();
    const ring = this.ringInput.value.trim();

    if (!core || !ring || ring.length != 6) return;

    const plays = get_plays(core, ring);

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
