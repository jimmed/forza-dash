const fields = {
  Timestamp: (b) => b.getUint32(4) / 1000,
};

const tauri = window.__TAURI__;

const state$ = document.querySelector("#server-state");
const stateListening$ = state$.querySelector("#listening");
const stateRunning$ = state$.querySelector("#running");
const main$ = document.querySelector("main");
const dl$ = main$.querySelector("dl");
for (const key of Object.keys(fields)) {
  const dd$ = document.createElement("dd");
  dd$.innerText = key;
  dl$.appendChild(dd$);
  dl$.appendChild(document.createElement("dt"));
}

tauri.event.listen("state", ({ payload: { listening, running } }) => {
  stateListening$.checked = listening;
  stateRunning$.checked = running;
});
tauri.event.listen("packet", ({ payload }) => {
  console.log(payload);
  try {
    const packet = new Uint8Array(payload);
    const buffer = new DataView(packet.buffer);
    for (const [index, method] of Object.entries(Object.values(fields))) {
      const value = method(buffer);
      const dt$ = dl$.childNodes[index * 2 + 1];
      dt$.innerText = value;
    }
  } catch (error) {
    console.error(error);
  }
});
tauri.invoke("listen");
