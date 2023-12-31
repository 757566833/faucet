async function digestMessage(message: string) {
  const msgUint8 = new TextEncoder().encode(message); // encode as (utf-8) Uint8Array
  const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8); // hash the message
  const hashArray = Array.from(new Uint8Array(hashBuffer)); // convert buffer to byte array
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join(""); // convert bytes to hex string
  return hashHex;
}
export const sha256 = async (root: string) => {
  let hash = "";
  let startTime = new Date().getTime();
  let nonce = 0;
  do {
    hash = await digestMessage(`${root}${nonce}`);
    nonce++;
  } while (!hash.startsWith("00000"));
  let endTime = new Date().getTime();
  console.log("time", endTime - startTime);
  return { hash, nonce };
};
