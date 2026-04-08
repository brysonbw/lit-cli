import { LitElement, css, html, type TemplateResult } from "lit";
import { customElement } from "lit/decorators.js";
import { Router } from "@lit-labs/router";
import type { RouteConfig } from "@lit-labs/router";

import "./components/header.ts";
import "./components/footer.ts";

export const ROUTES: RouteConfig[] = [
  {
    path: "/",
    render: () =>
      html`<home-page>
        <h1>Vite + Lit</h1>
        <div>
          <small><em>Generated with lit-cli</em></small>
        </div>
      </home-page>`,
    enter: async (): Promise<boolean> => {
      await import("./pages/home.ts");
      document.title = `${import.meta.env.VITE_APP_NAME} | Home`;
      return true;
    },
  },
  {
    path: "/about",
    render: () => html`<about-page></about-page>`,
    enter: async (): Promise<boolean> => {
      await import("./pages/about.ts");
      document.title = `${import.meta.env.VITE_APP_NAME} | About`;
      return true;
    },
  },
  {
    path: "/*",
    render: () => html`<not-found-page></not-found-page>`,
    enter: async (): Promise<boolean> => {
      await import("./pages/not-found.ts");
      document.title = `${import.meta.env.VITE_APP_NAME} | Page not found`;
      return true;
    },
  },
];

@customElement("app-root")
export class App extends LitElement {
  private _router = new Router(this, ROUTES);

  render(): TemplateResult {
    return html` <main>
      <app-header></app-header>
      <div id="outlet">${this._router.outlet()}</div>
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

declare global {
  interface HTMLElementTagNameMap {
    "app-root": App;
  }
}
