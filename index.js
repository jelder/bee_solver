import init, { get_plays } from "./pkg/bee_solver.js";

class BeeSolver extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          --bs-ls: 2ch; /* Letter spacing: 2 character units */
          --bs-gap: 1.25; /* Gap multiplier for background gradients */
          --bs-fz: 1.5em; /* Font size */
          --_bs-bgsz: calc(var(--bs-ls) + 1ch); /* Background segment size (letter spacing + 1 character) */
          /* Set total digits for the single field (1 yellow + 6 grey = 7) */
          --bs-digits: 7; 
        }
        :host input:where([type=text]) {
          all: unset; /* Remove all default user agent styles */
          text-transform: uppercase; /* Convert text to uppercase */
          caret-color: var(--bs-cc, #333); /* Color of the text input caret */
          clip-path: inset(0% calc(var(--bs-ls) / 2) 0% 0%); /* Clip the input to hide half of the letter spacing on the right */
          font-family: ui-monospace, monospace; /* Monospace font for consistent character width */
          font-size: var(--bs-fz, 2.5em); /* Font size for the input text */
          /* Calculate inline-size (width) based on the new total digits and background segment size */
          inline-size: calc(var(--bs-digits) * var(--_bs-bgsz)); 
          letter-spacing: var(--bs-ls); /* Apply letter spacing */
          padding-block: var(--bs-pb, 1ch); /* Vertical padding */
          padding-inline-start: calc(((var(--bs-ls) - 1ch) / 2) * var(--bs-gap)); /* Horizontal padding at the start */
          
          /* Define multiple background images for layering */
          background-image: 
            /* Yellow background for the first character */
            linear-gradient(
              90deg, /* Direction of the gradient: from left to right */
              rgb(247, 218, 33) /* Color of the gradient start (yellow) */ 
              calc(var(--bs-gap) * var(--bs-ls)), /* Color stop: yellow extends up to this point */
              transparent /* Second color of the gradient (transparent) */
              0 /* Color stop: transparent starts at this point (immediately after the yellow) */
            ),
            /* Grey background for the remaining characters, designed to repeat */
            linear-gradient(
              90deg, /* Direction of the gradient: from left to right */
              #EEE /* Start color: light grey */
              calc(var(--bs-gap) * var(--bs-ls)), /* Color stop: grey extends up to this point */
              transparent /* Second color of the gradient (transparent) */
              0 /* Color stop: transparent starts at this point (immediately after the grey) */
            );
          
          /* Define positions for each background image */
          background-position: 
            0 0, /* Position for yellow gradient: top-left corner */
            var(--_bs-bgsz) 0; /* Position for grey gradient: starts after the first character's segment */
          
          /* Define sizes for each background image */
          background-size: 
            var(--_bs-bgsz) 100%, /* Size for yellow gradient: width of one segment, full height */
            var(--_bs-bgsz) 100%; /* Size for grey gradient: width of one segment, full height (this segment will be repeated) */
          
          /* Define repeat behavior for each background image */
          background-repeat: 
            no-repeat, /* Yellow background: does not repeat */
            repeat-x; /* Grey background: repeats horizontally to cover the remaining characters */
        }
        /* Removed specific ID selectors for core and ring as they are no longer needed */
      </style>
      <form id="inputForm">
        <input type="text" id="input" required pattern="[A-Za-z]" size="7" minlength="7" maxlength="7" spellcheck="false">
      </form>
      <div id="results"></div>
    `;

    this.form = this.shadowRoot.getElementById("inputForm");
    this.input = this.shadowRoot.getElementById("input");
    this.resultsContainer = this.shadowRoot.getElementById("results");

    this.updateTable = this.updateTable.bind(this);

    const simulateInput = async (inputElement, value) => {
      for (let i = 0; i < value.length; i++) {
        await new Promise((resolve) => setTimeout(resolve, 300)); // Simulate typing delay
        inputElement.value = value.slice(0, i + 1);
        inputElement.dispatchEvent(new Event("input", { bubbles: true }));
      }
    };

    this.input.value = "";
    simulateInput(this.input, "AKMOBCE");

    this.input.addEventListener("focus", () => {
      this.input.value = "";
    });
  }

  connectedCallback() {
    this.form.addEventListener("input", this.updateTable);
  }

  disconnectedCallback() {
    this.form.removeEventListener("input", this.updateTable);
  }

  async updateTable() {
    this.input.value = [
      ...new Set(
        this.input.value
          .trim()
          .toUpperCase()
          .replace(/[^A-Z]/g, "")
      ),
    ].join("");

    const input = this.input.value;

    if (!input || input.length != 7) return;

    await init();
    const core = input.substr(0, 1);
    const ring = input.slice(1);
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
