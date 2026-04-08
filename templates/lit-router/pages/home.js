import { LitElement, css, html } from "lit";

export class HomePage extends LitElement {
  static properties = {
    count: { type: String },
  };

  constructor() {
    super();
    this.count = 0;
  }

  render() {
    return html`<div>
        <a href="https://vite.dev" target="_blank">
          <img src=/vite.svg class="logo" alt="Vite logo" />
        </a>
        <a href="https://lit.dev" target="_blank">
          <img src="/lit.svg" class="logo lit" alt="Lit logo" />
        </a>
      </div>
      <slot></slot>
      <p>Count: ${this.count}</p>
      <div class="card">
        <button @click=${this.#decrement}>Decrement Count</button>
        <button @click=${this.#reset} .disabled=${this.count == 0}>Reset</button>
        <button @click=${this.#increment}>Increment Count</button>
      </div>`;
  }

  #increment() {
    this.count++;
  }

  #reset() {
    this.count = 0;
  }

  #decrement() {
    this.count--;
  }

  static styles = css`
    :host {
      margin: 0 auto;
      padding: 2rem;
      text-align: center;
    }

    .logo {
      height: 6em;
      padding: 1.5em;
      will-change: filter;
      transition: filter 300ms;
    }
    .logo:hover {
      filter: drop-shadow(0 0 2em #646cffaa);
    }
    .logo.lit:hover {
      filter: drop-shadow(0 0 2em #325cffaa);
    }

    p {
      padding-top: 1.5rem;
    }

    .card {
      display: flex;
      flex-direction: row;
      flex-wrap: wrap;
      justify-content: center;
      align-items: center;
      gap: 1rem;
      padding: 0 2rem;
    }

    a {
      font-weight: 500;
      color: #646cff;
      text-decoration: inherit;
    }
    a:hover {
      color: #535bf2;
    }

    ::slotted(h1) {
      font-size: 3.2em;
      line-height: 1.1;
    }

    button {
      border-radius: 8px;
      border: 1px solid transparent;
      padding: 0.6em 1.2em;
      font-size: 1em;
      font-weight: 500;
      font-family: inherit;
      background-color: #1a1a1a;
      cursor: pointer;
      transition: border-color 0.25s;
    }
    button:hover {
      border-color: #646cff;
    }
    button:focus,
    button:focus-visible {
      outline: 4px auto -webkit-focus-ring-color;
    }

    @media (prefers-color-scheme: light) {
      a:hover {
        color: #747bff;
      }
      button {
        background-color: #f9f9f9;
      }
    }
  `;
}

customElements.define("home-page", HomePage);
