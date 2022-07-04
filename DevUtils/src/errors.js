module.exports = {
    AUTH_BOTH: "only the sender or the recipient of the stream can perform this action",
    AUTH_RECIPIENT: "only the stream recipient is allowed to perform this action",
    AUTH_SENDER: "only the stream sender is allowed to perform this action",
    BLOCK_DELTA: "the block difference needs to be higher than the payment interval",
    BLOCK_DELTA_MULTIPLICITY: "the block difference needs to be a multiple of the payment interval",
    BLOCK_START: "the start block needs to be higher than the current block number",
    BLOCK_STOP: "the stop block needs to be higher than the start block",
    CONTRACT_ALLOWANCE: "contract not allowed to transfer enough tokens",
    CONTRACT_EXISTENCE: "token contract address needs to be provided",
    INSOLVENCY: "not enough funds",
    STREAM_EXISTENCE: "stream doesn't exist",
    TERMS_NOT_CHANGED: "stream has these terms already",
  };

//   Number	Error	Reason
// 1	stream does not exist
// The provided stream id does not point to a valid stream
// 2	caller is not the sender or the recipient of the stream
// The contract call originates from an unauthorized third-party
// 3	SafeERC20: low-level call failed
// Possibly insufficient allowance, but not necessarily so
// 4	stream to the zero address
// Attempt to stream money to the zero address
// 5	stream to the contract itself
// Attempt to stream money to the Sablier contract
// 6	stream to the caller
// Happens when the caller attempts to stream money to herself
// 7	deposit is zero
// Attempt to stream 0 tokens
// 8	start time before block.timestamp
// Money cannot be streamed retroactively
// 9	stop time before the start time
// Negative streaming is not allowed
// 10	deposit smaller than time delta
// The deposit, measured in units of the token, is smaller than the time delta
// 11	deposit not multiple of time delta
// The deposit has a non-zero remainder when divided by the time delta
// 12	amount is zero
// Attempt to withdraw 0 tokens
// 13	amount exceeds the available balance
// Attempt to withdraw more money than the available balance
// 14	recipient balance calculation error
// Happens only when streaming an absurdly high number of tokens (close to 2^256)