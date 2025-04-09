use ethers::abi::{Token, encode_packed};

/// Encode a Solidity `uint16` as exactly 2 bytes (big-endian) as `abi.encodePacked(uint16)`.
fn encode_uint16_be(value: u16) -> Token {
    // 2-byte big-endian representation
    let bytes = value.to_be_bytes();
    Token::FixedBytes(bytes.to_vec())
}

/// Encode a Solidity `uint128` as exactly 16 bytes (big-endian) as `abi.encodePacked(uint128)`.
fn encode_uint128_be(value: u128) -> Token {
    // 16-byte big-endian representation
    let bytes = value.to_be_bytes(); // 16 bytes
    Token::FixedBytes(bytes.to_vec())
}

/// Encode a Solidity `uint8` as exactly 1 byte.
fn encode_uint8(value: u8) -> Token {
    Token::FixedBytes(vec![value])
}

// -------------------------
//  "newOptions()" -> 0x0003
// -------------------------

/// Equivalent of Solidity's `newOptions() => abi.encodePacked(TYPE_3)`
fn new_options() -> Vec<u8> {
    // TYPE_3 = 3 => 2 bytes big-endian: 0x0003
    let tokens = vec![encode_uint16_be(3)];
    encode_packed(&tokens).unwrap()
}

// ---------------------------------------------
//  encodeLzReceiveOption(_gas, _value) -> 16/32 bytes
// ---------------------------------------------

/// If `_value == 0`, encode `_gas` only (16 bytes).
/// Otherwise encode `_gas` + `_value` (32 bytes).
fn encode_lz_receive_option(gas: u128, value: u128) -> Vec<u8> {
    let gas_token = encode_uint128_be(gas);

    if value == 0 {
        // Single 16-byte chunk
        encode_packed(&[gas_token]).unwrap()
    } else {
        // 16 bytes for gas + 16 bytes for value => 32 total
        let value_token = encode_uint128_be(value);
        encode_packed(&[gas_token, value_token]).unwrap()
    }
}

// -----------------------------------------------------
//  addExecutorOption(_options, _optionType, _optionData)
// -----------------------------------------------------

/// Equivalent to:
/// abi.encodePacked(
///   _options,
///   WORKER_ID (1 byte),
///   uint16(_option.length + 1), // 2 bytes
///   _optionType (1 byte),
///   _option
/// )
fn add_executor_option(existing: &[u8], option_type: u8, new_option: &[u8]) -> Vec<u8> {
    let worker_id: u8 = 1; // WORKER_ID
    let length = (new_option.len() + 1) as u16;

    // Build tokens in the same order as abi.encodePacked
    let tokens = vec![
        // existing `bytes`
        Token::Bytes(existing.to_vec()), // The previous bytes
        // worker_id => 1 byte
        encode_uint8(worker_id),
        // length => 2 bytes big-endian
        encode_uint16_be(length),
        // optionType => 1 byte
        encode_uint8(option_type),
        // new_option => dynamic bytes
        Token::Bytes(new_option.to_vec()),
    ];

    encode_packed(&tokens).unwrap()
}

// -----------------------------------------------------
//  addExecutorLzReceiveOption(_options, gas, value)
// -----------------------------------------------------

/// `_options` -> add the "lzReceive" sub-option (with `_gas`, `_value`)
fn add_executor_lz_receive_option(options: &[u8], gas: u128, value: u128) -> Vec<u8> {
    // Build the sub-option data
    let opt_data = encode_lz_receive_option(gas, value);

    // OPTION_TYPE_LZRECEIVE = 1
    let option_type_lzreceive = 1u8;
    add_executor_option(options, option_type_lzreceive, &opt_data)
}

/// This replicates:
///   `Options.newOptions().addExecutorLzReceiveOption(gas, value).toBytes()`
pub fn build_options_with_lz_receive(gas: u128, value: u128) -> Vec<u8> {
    let base = new_options(); // => 0x0003
    add_executor_lz_receive_option(&base, gas, value)
}
