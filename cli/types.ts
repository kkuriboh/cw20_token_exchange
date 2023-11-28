export type ExecMessage = "c_w20_to_native" | {
	native_to_c_w20: {
		denom: string;
	}
};
