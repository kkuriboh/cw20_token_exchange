const CONTRACT_ADDRESS =
	Bun.env.CONTRACT_ADDRESS ??
	"sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m";

export default {
	RPC_ENDPOINT: "http://127.0.0.1:26657",
	DEFAULT_SEED_PHRASE:
		Bun.env.SEED_PHRASE ??
		"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
	CONTRACT_ADDRESS,
	CONTRACT_ID: Bun.env.CONTRACT_ID ? Number(Bun.env.CONTRACT_ID) : 3,
	COIN_DENOM: `factory/${CONTRACT_ADDRESS}/banana`,
};
