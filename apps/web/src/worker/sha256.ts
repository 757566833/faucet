import { sha256 } from "@/utils";

globalThis.addEventListener("message", async (event: MessageEvent<string>) => {
  postMessage(JSON.stringify(await sha256(event.data)));
});
