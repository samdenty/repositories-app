import { createEndpoint } from "comlink-extension";
import { browser } from "webextension-polyfill-ts";
import type { Background } from "../background/background";
import * as Comlink from "comlink";
import type { FolderLike, LinkData, File } from "../Link";

export const background = Comlink.wrap<Background>(
  createEndpoint(browser.runtime.connect())
);

const { port1, port2 } = new MessageChannel();
background.connectNativePort(port1);

export function request(message: Message) {
  port2.postMessage(message);
}

port2.onmessage = (e) => {
  console.log("got message", e);
};

// Messages
export type OpenInEditor = LinkData & {
  request: "OpenInEditor";
};

export type RevealInFinder = LinkData & {
  request: "RevealInFinder";
};

export type OpenInTerminal = FolderLike & {
  request: "OpenInTerminal";
};

export type OpenFile = File & {
  request: "OpenFile";
};

export type Message = OpenInEditor | RevealInFinder | OpenInTerminal | OpenFile;
