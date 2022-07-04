#![no_std]
#![no_main]

/// Events:

/// CreateStream - notice Emits when a stream is successfully created
// uint256 indexed streamId,
// address indexed sender,
// address indexed recipient,
// uint256 deposit,
// address tokenAddress,
// uint256 startTime,
// uint256 stopTime

/// WithdrawFromStream - notice Emits when the recipient of a stream withdraws a portion or all their pro rata share of the stream.
// uint256 indexed streamId,
// address indexed recipient,
// uint256 amount

/// CancelStream - notice Emits when a stream is successfully cancelled and tokens are transferred back on a pro rata basis.
// uint256 indexed streamId,
// address indexed sender,
// address indexed recipient,
// uint256 senderBalance,
// uint256 recipientBalance

const NEW_STREAM_ID: &str = "new_stream_id"

#[no_mangle]
pub extern "C" fn balanceOf() {
    
    // Read runtime args:

    // uint256 streamId
    // address who
    
    // Returns:
    // uint256 balance

}

#[no_mangle]
pub extern "C" fn getStream() {
    
    // Read runtime args:

    // uint256 streamId
    
    /// Parameters to return:

    // address sender
    // address recipient
    // uint256 deposit
    // address tokenAddress
    // uint256 startTime
    // uint256 stopTime
    // uint256 remainingBalance,
    // uint256 ratePerSecond

}

#[no_mangle]
pub extern "C" fn createStream() {
    
    // Read runtime args:
    
    // address recipient
    
    // uint256 deposit
    
    // address tokenAddress
    
    // uint256 startTime
    
    // uint256 stopTime

    // returns (uint256 streamId)
    // ret(streamId)

}

#[no_mangle]
pub extern "C" fn withdrawFromStream() {
    
    // Read runtime args:
    
    // uint256 streamId
    
    // uint256 funds / amount
    
    // Returns bool
    // ret(streamCanceled);

    // Returns bool
    // ret(withdrawSucced);

}

#[no_mangle]
pub extern "C" fn cancelStream() {
    
    // Read runtime args:
    
    // uint256 streamId
    
    // Returns bool
    // ret(streamCanceled);

}

#[no_mangle]
fn call() {
    
    // Read PublicKey from runtime args
    let new_public_key: PublicKey = runtime::get_named_arg(PUBLIC_KEY);

    // Entry points
    let entry_points: EntryPoints = entry_points::hub_contract_entry_points();

    // Named keys
    let mut named_keys = NamedKeys::new();

    // Create WITHDRAW_INSTANCES dictionary uref
    let withdraw_instances_uref = storage::new_dictionary(WITHDRAW_INSTANCES_REGISTRY).unwrap_or_revert();

    // Get WITHDRAW_INSTANCES dictionary key
    let withdraw_instances_key = {
        runtime::remove_key(WITHDRAW_INSTANCES_REGISTRY);
        Key::from(withdraw_instances_uref)
    };
    
    // Add WITHDRAW_INSTANCES dictionary key to Named keys
    named_keys.insert(WITHDRAW_INSTANCES_REGISTRY.to_string(), withdraw_instances_key);

    // Create STAKERS_WITHDRAWALS dictionary uref
    let manager_uref = storage::new_dictionary(STAKERS_WITHDRAWALS_REGISTRY).unwrap_or_revert();

    // Get STAKERS_WITHDRAWALS dictionary key
    let manager_key = {
        runtime::remove_key(STAKERS_WITHDRAWALS_REGISTRY);
        Key::from(manager_uref)
    };

    // Add STAKERS_WITHDRAWALS dictionary key to Named keys
    named_keys.insert(STAKERS_WITHDRAWALS_REGISTRY.to_string(), withdraw_instances_key);

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(HASH_NAME.to_string()),
        Some(UREF_NAME.to_string()),
    );

    // Initialize contract
    // Set Hub's contract CSPR 'deposit' and 'withdrawal' purses
    let _: () = runtime::call_contract(contract_hash, ENTRY_POINT_INIT, RuntimeArgs::new());

    // TODO
    // Call versioned contract for ENTRY_POINT_INIT

}