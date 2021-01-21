import "./styles.scss";
import { Link, parseLink } from "../Link";
import { watchSelector } from "./watchSelector";
import { browser } from "webextension-polyfill-ts";

const initialized = new WeakSet();

document.addEventListener("DOMContentLoaded", () => {
  const link = document.createElement("link");
  link.href = browser.extension.getURL("dist/styles.css");
  link.type = "text/css";
  link.rel = "stylesheet";

  document.head.appendChild(link);

  watchSelector("a[href]", (element) => {
    if (initialized.has(element)) return;
    const link = parseLink(element);
    if (!link) return;

    const childNodes = Array.from(element.childNodes);
    element.innerHTML = "";

    new Link({
      target: element,
      props: {
        childNodes,
        link,
        href: element.href,
      },
    });

    initialized.add(element);
  });
});
