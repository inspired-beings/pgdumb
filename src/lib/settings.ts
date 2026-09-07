import { Store } from "@tauri-apps/plugin-store";

export type SplitOrientation = "vertical" | "horizontal";

const SETTINGS_FILE = "settings.json";
const SPLIT_ORIENTATION_KEY = "splitOrientation";
const DEFAULT_SPLIT_ORIENTATION: SplitOrientation = "vertical";

let storePromise: ReturnType<typeof Store.load> | null = null;

function getStore() {
  if (!storePromise) {
    storePromise = Store.load(SETTINGS_FILE);
  }
  return storePromise;
}

export async function getSplitOrientation(): Promise<SplitOrientation> {
  const store = await getStore();
  const value = await store.get<SplitOrientation>(SPLIT_ORIENTATION_KEY);
  return value ?? DEFAULT_SPLIT_ORIENTATION;
}

export async function setSplitOrientation(orientation: SplitOrientation): Promise<void> {
  const store = await getStore();
  await store.set(SPLIT_ORIENTATION_KEY, orientation);
}
