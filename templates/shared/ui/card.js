import { LitElement, css, html } from "lit";

/** @typedef {"light" | "dark"} Theme */

/** Base/UI Card */
export class UICard extends LitElement {
  static properties = {
    variant: { type: String, reflect: true },
  };

  constructor() {
    super();
    /** @type {Theme} */
    this.variant = "dark";
  }

  render() {
    return html`<li class="card">
      <div class="card-header">
        <slot name="header"></slot>
        <slot name="actions"></slot>
      </div>

      <div class="card-content">
        <slot name="content"></slot>
      </div>

      <div class="card-footer">
        <slot name="footer"></slot>
      </div>
    </li>`;
  }

  static styles = css`
    :host {
      display: block;
      list-style: none;
      /** Local variables */
      --card-background-color: #2c2c2c;
      --card-border-color: #3a3a3a;
      --card-border-hover: #4a4a4a;
      --card-text-color: #f1f1f1;
    }

    .card {
      border: 0.0625rem solid;
      border-color: var(--card-border-color);
      background-color: var(--card-background-color);
      border-radius: 0.375rem;
      padding: 1rem;
      display: flex;
      flex-direction: column;
      height: 100%;
      box-sizing: border-box;
      color: var(--card-text-color);
    }

    .card-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 0.5rem;
      gap: 0.5rem;
    }

    ::slotted([slot="header"]) {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      flex-grow: 1;
    }

    .card-content {
      flex-grow: 1;
      display: flex;
      flex-direction: column;
    }

    .card-footer {
      display: flex;
      flex-wrap: wrap;
      gap: 0.5rem;
      margin-top: auto;
    }

    .card:hover {
      border-color: var(--card-border-hover);
    }

    :host([variant="light"]) .card:hover {
      border-color: var(--card-border-color);
    }

    :host([variant="light"]) {
      --card-background-color: var(--white);
      --card-border-color: var(--gray);
      --card-border-hover: #666666;
      --card-text-color: var(--background-color);
    }
  `;
}

customElements.define("ui-card", UICard);
