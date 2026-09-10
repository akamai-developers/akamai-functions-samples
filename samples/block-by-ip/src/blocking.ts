import * as Kv from "@spinframework/spin-kv";

const loadBlocklist = (): string[] => {
  const store = Kv.openDefault();
  if (!store.exists("blocklist")) {
    return [];
  }
  return (store.getJson("blocklist") as string[]) ?? [];
};

const storeBlocklist = (blocklist: string[] | null | undefined): void => {
  const store = Kv.openDefault();
  store.setJson("blocklist", blocklist ?? []);
};

export {
  loadBlocklist,
  storeBlocklist
};
