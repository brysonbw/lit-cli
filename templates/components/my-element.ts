import { LitElement, css, html, type TemplateResult } from "lit";
import { customElement, property } from "lit/decorators.js";

@customElement("my-element")
export class MyElement extends LitElement {
  @property({ type: Number })
  count: number = 0;

  render(): TemplateResult {
    return html`<div>
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" class="logo" alt="Vite logo" />
        </a>
        <a href="https://lit.dev" target="_blank">
          <img src="/lit.svg" class="logo lit" alt="Lit logo" />
        </a>
      </div>
      <slot></slot>
      <p>Count: ${this.count}</p>
      <div class="card">
        <button @click=${this._decrement}>Decrement Count</button>
        <button @click=${this._reset} .disabled=${this.count == 0}>
          Reset
        </button>
        <button @click=${this._increment}>Increment Count</button>
      </div>
      <details>
        <summary>What is lit-cli?</summary>
        <div>
          <p>
            lit-cli is a
            <a href="https://www.rust-lang.org/" target="_blank">Rust</a>
            <a
              href="https://en.wikipedia.org/wiki/Command-line_interface"
              target="_blank"
              >CLI</a
            >
            tool to scaffold and manage
            <a href="https://lit.dev/" target="_blank">Lit</a> projects. It uses
            the <a href="https://vite.dev/" target="_blank">Vite</a> build tool
            (server and bundler), which builds on the (Lit)
            <a
              href="https://vite.dev/guide/#scaffolding-your-first-vite-project"
              target="_blank"
              >scaffolding experience provided by Vite</a
            >
            with additional opinionated project scaffolding and features.
          </p>
          <p>
            If you find this project useful, please consider starring or
            contributing to the
            <a href="https://github.com/brysonbw/lit-cli" target="_blank"
              >lit-cli GitHub repository</a
            >! You can also check out the
            <a
              href="https://www.npmjs.com/package/@lit-labs/cli"
              target="_blank"
              >official Lit CLI tool</a
            >.
          </p>
          <p>
            Lastly, continue to dream, work, collaborate, build, create, spread
            love, and
            <b
              >most importantly I encourage you to build for the greater good of
              humanity</b
            >. Also, you have agency over your life so think and act on your
            ideas. <b>Remember, anything is possible</b> - keep moving forward
            and keep making progress.
          </p>
          <p>Much peace and love!</p>
        </div>
      </details> `;
  }

  private _increment(): void {
    this.count++;
  }

  private _reset(): void {
    this.count = 0;
  }

  private _decrement(): void {
    this.count--;
  }

  static styles = css`
    :host {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      flex-wrap: wrap;
      justify-content: center;
      align-items: center;
      text-align: center;
      gap: 0.5rem;
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

    details {
      margin-top: 1rem;
      border-radius: 4px;
      padding: 0.5rem;
    }

    details:hover {
      cursor: pointer;
    }

    summary {
      font-weight: 500;
      margin: -0.5rem;
      padding: 0.5rem;
    }

    details[open] {
      padding: 0.5rem 1rem;
    }

    details[open] summary {
      padding: 0.5rem 1rem;
      box-shadow: inset 0 -1px 0 #aaaaaa;
    }

    details[open] div {
      margin: 1rem 2rem;
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

declare global {
  interface HTMLElementTagNameMap {
    "my-element": MyElement;
  }
}
