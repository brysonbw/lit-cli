import { LitElement, css, html } from "lit";

export class AboutPage extends LitElement {
  constructor() {
    super();
  }
  render() {
    return html`
      <div>
        <h1>What is lit-cli?</h1>
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
          <a href="https://www.npmjs.com/package/@lit-labs/cli" target="_blank"
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
          ideas. <b>Remember, anything is possible</b> - keep moving forward and
          keep making progress.
        </p>
        <p>Much peace and love!</p>
      </div>
    `;
  }

  static styles = css`
    a {
      text-decoration: none;
      color: var(--info, #4b7ce6);
    }

    a:hover {
      text-decoration: underline;
    }
  `;
}

customElements.define("about-page", AboutPage);
