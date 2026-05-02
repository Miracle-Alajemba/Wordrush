export function createWordPotContractService(options) {
  const contractAddress = String(options?.contractAddress || "").trim();
  const rpcUrl = String(options?.rpcUrl || "https://rpc.portaldot.xyz").trim();
  const localDevMode =
    options?.localDevMode === true ||
    /127\.0\.0\.1|localhost/.test(rpcUrl) ||
    process.env.PORTALDOT_LOCAL_DEV === "true";

  if (!localDevMode) {
    return {
      enabled: false,
      reason: "contract_not_wired",
      account: null,
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

  let nextRoomId = 1n;
  function mockHash(tag) {
    const raw = `${tag}:${Date.now()}:${Math.random().toString(36).slice(2)}`;
    let hex = "";
    for (let i = 0; i < raw.length && hex.length < 64; i += 1) {
      hex += raw.charCodeAt(i).toString(16).padStart(2, "0");
    }
    return `0x${hex.padEnd(64, "0").slice(0, 64)}`;
  }

  return {
    enabled: true,
    reason: "local_dev_mock",
    account: { address: contractAddress || "//Alice" },
    async createRoom(_entryFeeWei) {
      const roomId = nextRoomId;
      nextRoomId += 1n;
      return { roomId, hash: mockHash("create-room") };
    },
    async joinRoom(_roomId) {
      return { hash: mockHash("join-room") };
    },
    async settleRoom(_roomId, _players, _scores) {
      return { hash: mockHash("settle-room") };
    },
    async cancelRoom(_roomId) {
      return { hash: mockHash("cancel-room") };
    },
    async claimReward(_roomId) {
      return { hash: mockHash("claim-reward") };
    },
  };
}
