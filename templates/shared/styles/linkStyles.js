import { css } from "lit";

const linkStyles = css`
  a {
    text-decoration: none;
    color: var(--info, #4b7ce6);
  }

  a:hover {
    text-decoration: underline;
  }
`;

export { linkStyles };
