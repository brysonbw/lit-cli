import { LitElement, css, html } from "lit";
import { Router } from "@lit-labs/router";

import "./components/header.js";
import "./components/footer.js";

export const ROUTES = [
  {
    path: "/",
    render: () =>
      html`<home-page>
        <h1>Vite + Lit</h1>
        <div>
          <small><em>Generated with lit-cli</em></small>
        </div>
      </home-page>`,
    enter: async () => {
      await import("./pages/home.js");
      document.title = `${import.meta.env.VITE_APP_NAME} | Home`;
      return true;
    },
  },
  {
    path: "/about",
    render: () => html`<about-page></about-page>`,
    enter: async () => {
      await import("./pages/about.js");
      document.title = `${import.meta.env.VITE_APP_NAME} | About`;
      return true;
    },
  },
  {
    path: "/*",
    render: () => html`<not-found-page></not-found-page>`,
    enter: async () => {
      await import("./pages/not-found.js");
      document.title = `${import.meta.env.VITE_APP_NAME} | Page not found`;
      return true;
    },
  },
];

export class App extends LitElement {
  #router = new Router(this, ROUTES);

  constructor() {
    super();
  }

  render() {
    return html`<main>
      <app-header></app-header>
      <div id="outlet">${this.#router.outlet()}</div>
      <app-footer></app-footer>
    </main>`;
  }

  static styles = css`
    :host {
      display: flex;
      flex-direction: column;
      min-height: 100vh;
    }

    main {
      width: min(95ch, 100% - 4rem);
      margin-inline: auto;
      display: flex;
      flex-direction: column;
      flex: 1;
    }

    #outlet {
      flex: 1;
      display: flex;
    }
  `;
}

customElements.define("app-root", App);
