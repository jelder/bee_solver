import init, { get_plays } from "./pkg/bee_solver.js";

class BeeSolver extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          --bs-ls: 2ch;
          --bs-gap: 1.25;
          --bs-fz: 1.5em;
          --_bs-bgsz: calc(var(--bs-ls) + 1ch);
        }
        :host input:where([type=text]) {
          all: unset;
          text-transform: uppercase;
          caret-color: var(--bs-cc, #333);
          clip-path: inset(0% calc(var(--bs-ls) / 2) 0% 0%);
          font-family: ui-monospace, monospace;
          font-size: var(--bs-fz, 2.5em);
          inline-size: calc(var(--bs-digits) * var(--_bs-bgsz));
          letter-spacing: var(--bs-ls);
          padding-block: var(--bs-pb, 1ch);
          padding-inline-start: calc(((var(--bs-ls) - 1ch) / 2) * var(--bs-gap));
        }
        :host input:where([id=core]) {
          --bs-digits: 1;
          --bs-bg: rgb(247, 218, 33);
          background: linear-gradient(90deg, 
            var(--bs-bg) calc(var(--bs-gap) * var(--bs-ls)),
            transparent 0
          ) 0 0 / var(--_bs-bgsz) 100%;
        }
        :host input:where([id=ring]) {
          --bs-digits: 6;
          --bs-bg: #EEE;
          background: linear-gradient(90deg, 
            var(--bs-bg) calc(var(--bs-gap) * var(--bs-ls)),
            transparent 0
          ) 0 0 / var(--_bs-bgsz) 100%;
        }
      </style>
      <form id="inputForm">
        <input type="text" id="core" required pattern="[A-Za-z]" size="1" minlength="1" maxlength="1" spellcheck="false">
        <input type="text" id="ring" required pattern="[A-Za-z]" size="6" minlength="6" maxlength="6" spellcheck="false">
      </form>
      <div id="results"></div>
    `;

    this.form = this.shadowRoot.getElementById("inputForm");
    this.coreInput = this.shadowRoot.getElementById("core");
    this.ringInput = this.shadowRoot.getElementById("ring");
    this.resultsContainer = this.shadowRoot.getElementById("results");

    this.updateTable = this.updateTable.bind(this);

    this.coreInput.value = "A";
    const simulateInput = async (inputElement, value) => {
      for (let i = 0; i < value.length; i++) {
        await new Promise((resolve) => setTimeout(resolve, 300)); // Simulate typing delay
        inputElement.value = value.slice(0, i + 1);
        inputElement.dispatchEvent(new Event("input", { bubbles: true }));
      }
    };

    this.ringInput.value = "";
    simulateInput(this.ringInput, "KMOBCE");
  }

  connectedCallback() {
    this.form.addEventListener("input", this.updateTable);
  }

  disconnectedCallback() {
    this.form.removeEventListener("input", this.updateTable);
  }

  async updateTable() {
    const core = this.coreInput.value.trim();
    const ring = this.ringInput.value.trim();

    if (!core || !ring || ring.length != 6) return;

    await init();
    const plays = get_plays(core, ring);

    // Group plays by score
    const groupedByScore = plays.reduce((acc, play) => {
      if (!acc[play.score]) {
        acc[play.score] = [];
      }
      acc[play.score].unshift(play);
      return acc;
    }, {});

    this.resultsContainer.innerHTML = "";
    const table = document.createElement("table");

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

    this.resultsContainer.appendChild(table);
  }
}

customElements.define("bee-solver", BeeSolver);
