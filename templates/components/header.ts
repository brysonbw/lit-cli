import { LitElement, css, html, type TemplateResult } from "lit";
import { customElement } from "lit/decorators.js";

@customElement("app-header")
export class HeaderComponent extends LitElement {
  render(): TemplateResult {
    return html`<header>
      <nav>
        <div class="nav-logo">
          <img
            class="logo"
            src="/lit.svg"
            alt=${`${import.meta.env.VITE_APP_NAME.toLowerCase()} logo`}
          />
          <p class="app-name" aria-current="page">
            ${import.meta.env.VITE_APP_NAME}
          </p>
        </div>

        <div class="nav-links">
          <a href="/about">About</a>
        </div>
      </nav>
    </header>`;
  }

  static styles = css`
    nav {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .nav-logo {
      display: flex;
      justify-content: center;
      align-items: center;
    }

    .logo {
      height: 1.8rem;
      width: auto;
      align-items: center;
      margin-right: 0.5rem;
    }

    .app-title {
      font-size: 1.125rem;
      font-weight: 500;
    }

    .nav-links {
      display: flex;
      gap: 1.5rem;
    }

    a {
      text-decoration: none;
      font-weight: 500;
    }

    a:hover {
      text-decoration: underline;
    }

    @media only screen and (max-width: 640px) {
      .logo {
        height: 1.65rem;
      }

      .app-title {
        font-size: 0.95rem;
      }

      a {
        font-size: 0.95rem;
      }
    }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    "app-header": HeaderComponent;
  }
}
