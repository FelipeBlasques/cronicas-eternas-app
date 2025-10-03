// src/lib/deeplink.ts
import { listen } from "@tauri-apps/api/event";

let pending: string[] = [];
let ready = false;

/** Chame isto quando sua UI terminar de montar */
export function markUiReady() {
  ready = true;
  for (const uri of pending) handleDeepLink(uri);
  pending = [];
}

function handleDeepLink(uri: string) {
  try {
    const url = new URL(uri);
    // Para "myflc://join?token=ABC&world=42", em muitos SOs "join" vira hostname
    const action = url.hostname || url.pathname.replace("/", "");
    const token = url.searchParams.get("token");
    const world = url.searchParams.get("world");

    console.log("[deep-link] action:", action, "token:", token, "world:", world);

    // PASSO 3: aqui vamos chamar seu backend e abrir o invite_url.
  } catch (e) {
    console.error("[deep-link] erro parseando:", uri, e);
  }
}

// Registra o listener somente no client
if (typeof window !== "undefined") {
  listen<string>("deep-link", (e) => {
    if (ready) handleDeepLink(e.payload);
    else pending.push(e.payload);
  });

  // Para testar sem protocolo real, você pode simular:
  // setTimeout(() => handleDeepLink("myflc://join?token=TESTE123&world=1"), 1000);
}
