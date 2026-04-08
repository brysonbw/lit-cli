import { LitElement, css, html, type TemplateResult } from "lit";
import { customElement, property } from "lit/decorators.js";

@customElement("loading-indicator")
export class LoadingIndicator extends LitElement {
  @property({ type: String, reflect: true })
  size?: string = "default";

  render(): TemplateResult {
    return html`<div class="spinner-container">
      <div class="spinner"></div>
      <slot name="text">Loading...</slot>
    </div>`;
  }

  static styles = css`
    :host {
      display: inline-block;
      --spinner-color: var(--white);
    }

    .spinner-container {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 0.5rem;
      height: 100%;
    }

    .spinner {
      width: 2.5rem;
      height: 2.5rem;
      border: 4px solid rgba(255, 255, 255, 0.2);
      border-top-color: var(--spinner-color);
      border-radius: 50%;
      animation: spin 1s linear infinite;
    }

    .spinner-text {
      font-size: 0.875rem;
      color: var(--spinner-color, #007bff);
      text-align: center;
    }

    @keyframes spin {
      to {
        transform: rotate(360deg);
      }
    }

    :host([size="small"]) .spinner {
      width: 1.5rem;
      height: 1.5rem;
      border-width: 0.1875rem;
    }
  `;
}

declare global {
  interface HTMLElementTagNameMap {
    "loading-indicator": LoadingIndicator;
  }
}
