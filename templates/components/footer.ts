import { LitElement, css, html, type TemplateResult } from "lit";
import { customElement } from "lit/decorators.js";

function currentYear(): string {
  const date = new Date();
  return date.getFullYear().toString();
}

@customElement("app-footer")
export class FooterComponent extends LitElement {
  render(): TemplateResult {
    return html`<footer>
      <div class="footer-container">
        <div class="footer-text">
          <p>© ${currentYear()}</p>
          •
          <p><a href="/">${import.meta.env.VITE_APP_NAME}</a></p>
        </div>
      </div>
    </footer> `;
  }

  static styles = css`
    .footer-container {
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 10vh;
      flex-wrap: wrap;
    }

    .footer-text {
      display: flex;
      gap: 10px;
      justify-content: center;
      align-items: center;
      color: #acacac;
      font-size: 1rem;
    }

    a {
      text-decoration: none;
      color: var(--info, #4b7ce6);
    }

    a:hover {
      text-decoration: underline;
    }

    @media only screen and (max-width: 640px) {
      .footer-text {
        font-size: 0.7rem;
      }
    }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    "app-footer": FooterComponent;
  }
}
