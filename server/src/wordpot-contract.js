import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function normalizePrivateKey(value) {
  const raw = String(value || "").trim();
  if (!raw) return "";
  return raw.startsWith("0x") ? raw : `0x${raw}`;
}

function isAddress(value) {
  return /^0x[a-fA-F0-9]{40}$/.test(String(value || "").trim());
}

// MVP: Mock contract service (no real contract needed yet)
export function createWordPotContractService(options) {
  const contractAddress = String(options?.contractAddress || "").trim();
  const rpcUrl = String(options?.rpcUrl || "https://rpc.portaldot.xyz").trim();
  const operatorKey = normalizePrivateKey(options?.operatorPrivateKey);

  // For MVP, contracts are optional
  return {
    enabled: false,
    reason: "mvp_mode",
    account: { address: "0x0000000000000000000000000000000000000000" },
    async createRoom() {
      return { roomId: null, hash: null };
    },
    async joinRoom() {
      return { hash: null };
    },
    async settleRoom() {
      return { hash: null };
    },
    async cancelRoom() {
      return { hash: null };
    },
    async claimReward() {
      return { hash: null };
    },
  };
}
