#include <evmone/evmone.h>
#include <evmone_precompiles/keccak.hpp>
#include <CLI/CLI.hpp>
#include <evmc/evmc.hpp>
#include <evmc/hex.hpp>
#include <evmc/mocked_host.hpp>

#include <array>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>

using namespace evmc::literals;

constexpr int64_t GAS = INT64_MAX;
constexpr auto REVISION = EVMC_LATEST_STABLE_REVISION;
const auto ZERO_ADDRESS = 0x0000000000000000000000000000000000000000_address;
const auto CALLER_ADDRESS = 0x1000000000000000000000000000000000000001_address;

evmc::bytes load_hex_bytes(const std::string& path) {
  std::ifstream file(path);
  if (!file) {
    std::cerr << "failed to open " << path << std::endl;
    std::exit(1);
  }

  std::string hex;
  file >> hex;

  evmc::bytes out;
  out.reserve(hex.size() / 2);
  evmc::from_hex(hex.begin(), hex.end(), std::back_inserter(out));
  return out;
}

void check_status(const evmc::Result& result) {
  if (result.status_code != EVMC_SUCCESS) {
    std::cerr << evmc_status_code_to_string(result.status_code) << std::endl;
    std::exit(1);
  }
}

evmc::bytes32 keccak_bytes(const uint8_t* data, size_t size) {
  const auto hash = ethash::keccak256(data, size);
  evmc::bytes32 out{};
  std::copy(std::begin(hash.bytes), std::end(hash.bytes), std::begin(out.bytes));
  return out;
}

evmc::bytes32 keccak_bytes(const evmc::bytes& data) {
  return keccak_bytes(data.data(), data.size());
}

evmc::bytes encode_nonce_rlp(uint64_t nonce) {
  if (nonce == 0) {
    return evmc::bytes{0x80};
  }

  std::array<uint8_t, sizeof(uint64_t)> buffer{};
  size_t offset = buffer.size();
  uint64_t value = nonce;
  while (value > 0) {
    buffer[--offset] = static_cast<uint8_t>(value & 0xff);
    value >>= 8;
  }

  const size_t size = buffer.size() - offset;
  evmc::bytes out;
  if (size == 1 && buffer[offset] < 0x80) {
    out.push_back(buffer[offset]);
    return out;
  }

  out.push_back(static_cast<uint8_t>(0x80 + size));
  out.insert(out.end(), buffer.begin() + static_cast<std::ptrdiff_t>(offset),
             buffer.end());
  return out;
}

evmc::address compute_create_address(const evmc::address& sender,
                                     uint64_t sender_nonce) {
  const auto nonce_rlp = encode_nonce_rlp(sender_nonce);
  const size_t payload_size = 1 + sizeof(sender.bytes) + nonce_rlp.size();

  evmc::bytes encoded;
  encoded.reserve(1 + payload_size);
  encoded.push_back(static_cast<uint8_t>(0xc0 + payload_size));
  encoded.push_back(static_cast<uint8_t>(0x80 + sizeof(sender.bytes)));
  encoded.insert(encoded.end(), std::begin(sender.bytes), std::end(sender.bytes));
  encoded.insert(encoded.end(), nonce_rlp.begin(), nonce_rlp.end());

  const auto hash = keccak_bytes(encoded);
  evmc::address out{};
  std::copy(std::end(hash.bytes) - sizeof(out.bytes), std::end(hash.bytes),
            std::begin(out.bytes));
  return out;
}

evmc::address compute_create2_address(const evmc::address& sender,
                                      const evmc::bytes32& salt,
                                      const uint8_t* init_code,
                                      size_t init_code_size) {
  const auto init_hash = keccak_bytes(init_code, init_code_size);
  std::array<uint8_t, 1 + sizeof(sender.bytes) + sizeof(salt.bytes) +
                          sizeof(init_hash.bytes)>
      buffer{};
  auto it = buffer.begin();
  *it++ = 0xff;
  it = std::copy(std::begin(sender.bytes), std::end(sender.bytes), it);
  it = std::copy(std::begin(salt.bytes), std::end(salt.bytes), it);
  std::copy(std::begin(init_hash.bytes), std::end(init_hash.bytes), it);

  const auto hash = keccak_bytes(buffer.data(), buffer.size());
  evmc::address out{};
  std::copy(std::end(hash.bytes) - sizeof(out.bytes), std::end(hash.bytes),
            std::begin(out.bytes));
  return out;
}

class StatefulHost : public evmc::MockedHost {
 public:
  explicit StatefulHost(evmc_vm* vm) : vm_(vm) {
    tx_context.block_gas_limit = GAS;
    tx_context.block_number = 1;
    tx_context.block_timestamp = 1;
    tx_context.tx_origin = CALLER_ADDRESS;
    tx_context.chain_id.bytes[31] = 1;

    auto& caller = accounts[CALLER_ADDRESS];
    caller.nonce = 0;
    caller.set_balance(UINT64_MAX);
  }

  evmc::Result call(const evmc_message& msg) noexcept override {
    if (msg.kind == EVMC_CREATE || msg.kind == EVMC_CREATE2) {
      return execute_create(msg);
    }
    return execute_call(msg);
  }

 private:
  evmc_vm* vm_;

  evmc::Result execute_message(const evmc_message& msg, const uint8_t* code,
                               size_t code_size) noexcept {
    const auto raw =
        evmc_execute(vm_, &get_interface(), reinterpret_cast<evmc_host_context*>(this),
                     REVISION, &msg, code, code_size);
    return evmc::Result{raw};
  }

  evmc::Result execute_call(const evmc_message& msg) noexcept {
    const auto snapshot = accounts;

    const auto code_address =
        evmc::is_zero(msg.code_address) ? msg.recipient : msg.code_address;
    const auto it = accounts.find(code_address);
    if (it == accounts.end() || it->second.code.empty()) {
      return evmc::Result{EVMC_SUCCESS, msg.gas, 0};
    }

    auto result = execute_message(msg, it->second.code.data(), it->second.code.size());
    if (result.status_code != EVMC_SUCCESS) {
      accounts = snapshot;
    }
    return result;
  }

  evmc::Result execute_create(const evmc_message& msg) noexcept {
    const auto snapshot = accounts;

    auto& sender = accounts[msg.sender];
    const auto sender_nonce = static_cast<uint64_t>(sender.nonce);
    sender.nonce += 1;

    const auto created_address =
        msg.kind == EVMC_CREATE
            ? compute_create_address(msg.sender, sender_nonce)
            : compute_create2_address(msg.sender, msg.create2_salt, msg.input_data,
                                      msg.input_size);

    const auto existing = accounts.find(created_address);
    if (existing != accounts.end() &&
        (!existing->second.code.empty() || existing->second.nonce != 0)) {
      accounts = snapshot;
      return evmc::Result{EVMC_FAILURE, 0, 0, created_address};
    }

    auto& created = accounts[created_address];
    created.nonce = 1;

    evmc_message create_msg = msg;
    create_msg.recipient = created_address;
    create_msg.code_address = ZERO_ADDRESS;
    create_msg.input_data = nullptr;
    create_msg.input_size = 0;

    auto result =
        execute_message(create_msg, msg.input_data, msg.input_size);
    if (result.status_code != EVMC_SUCCESS) {
      accounts = snapshot;
      result.create_address = created_address;
      return result;
    }

    created.code.assign(result.output_data, result.output_data + result.output_size);
    created.codehash = keccak_bytes(created.code);
    result.create_address = created_address;
    return result;
  }
};

int main(int argc, char** argv) {
  std::string contract_code_path;
  std::string calldata;
  unsigned int num_runs;

  CLI::App app{"evmone runner"};
  app.add_option("--contract-code-path", contract_code_path,
                 "Path to the hex contract code to deploy and run")
      ->required();
  app.add_option("--calldata", calldata,
                 "Hex of calldata to use when calling the contract")
      ->required();
  app.add_option("--num-runs", num_runs, "Number of times to run the benchmark")
      ->required();

  CLI11_PARSE(app, argc, argv);

  const auto contract_code = load_hex_bytes(contract_code_path);

  evmc::bytes calldata_bytes;
  calldata_bytes.reserve(calldata.size() / 2);
  evmc::from_hex(calldata.begin(), calldata.end(),
                 std::back_inserter(calldata_bytes));

  const auto vm = evmc_create_evmone();

  StatefulHost deployment_host{vm};

  evmc_message create_msg{};
  create_msg.kind = EVMC_CREATE;
  create_msg.gas = GAS;
  create_msg.sender = CALLER_ADDRESS;
  create_msg.input_data = contract_code.data();
  create_msg.input_size = contract_code.size();

  const auto create_result = deployment_host.call(create_msg);
  check_status(create_result);

  const auto contract_address = create_result.create_address;
  const auto base_host = deployment_host;

  evmc_message call_msg{};
  call_msg.kind = EVMC_CALL;
  call_msg.gas = GAS;
  call_msg.input_data = calldata_bytes.data();
  call_msg.input_size = calldata_bytes.size();
  call_msg.recipient = contract_address;
  call_msg.code_address = contract_address;
  call_msg.sender = CALLER_ADDRESS;

  for (unsigned int i = 0; i < num_runs; ++i) {
    auto host = base_host;
    const auto start = std::chrono::steady_clock::now();
    const auto call_result = host.call(call_msg);
    const auto end = std::chrono::steady_clock::now();
    check_status(call_result);

    using namespace std::literals;
    std::cout << (end - start) / 1.ms << std::endl;
  }

  vm->destroy(vm);
}
