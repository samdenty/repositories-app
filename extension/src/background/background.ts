import { browser } from "webextension-polyfill-ts";
import {
  createBackgroundEndpoint,
  forward,
  isMessagePort,
} from "comlink-extension";
import * as Comlink from "comlink";

const nativePort = browser.runtime.connectNative("repositories");

export class Background {
  public async connectNativePort(port: MessagePort) {
    await forward(port, nativePort);
  }
}

browser.runtime.onConnect.addListener((port) => {
  if (isMessagePort(port)) return;

  Comlink.expose(new Background(), createBackgroundEndpoint(port));
});
