import * as Kv from "@spinframework/spin-kv";
import { getClientAddressFromRequest, cleanupIpAddress } from "./helpers";

export interface BlockResult {
  allowed: boolean;
  country: string | null;
  message: string;
}

// Resolve the client's country from its IP address using ip-api.com.
// Returns null when the client address is unknown or the lookup fails.
export async function lookupCountry(request: Request): Promise<string | null> {
  const clientAddress = getClientAddressFromRequest(request);
  if (!clientAddress) {
    return null;
  }
  const ip = cleanupIpAddress(clientAddress);
  try {
    const response = await fetch(`http://ip-api.com/json/${ip}`);
    const details = (await response.json()) as { country?: string };
    return details.country ?? null;
  } catch {
    return null;
  }
}

// Evaluate whether the request should be allowed, based on the KV blocklist.
export async function evaluateBlock(request: Request): Promise<BlockResult> {
  const country = await lookupCountry(request);
  if (!country) {
    return {
      allowed: false,
      country: null,
      message: "Could not determine your country from the client IP address.",
    };
  }

  const blocklist = loadBlocklist();
  if (blocklist.indexOf(country) > -1) {
    return {
      allowed: false,
      country,
      message: `Sorry, your country (${country}) is blocked.`,
    };
  }

  return {
    allowed: true,
    country,
    message: "If you can read this, you've successfully passed the blocking mechanism.",
  };
}

export function loadBlocklist(): string[] {
  const store = Kv.openDefault();
  if (!store.exists("blocklist")) {
    return [];
  }
  return store.getJson("blocklist") as string[];
}

export function storeBlocklist(blocklist: string[]): void {
  const store = Kv.openDefault();
  store.setJson("blocklist", blocklist ?? []);
}
