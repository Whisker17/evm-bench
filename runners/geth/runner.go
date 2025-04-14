package main

import (
	"fmt"
	"math/big"
	"os"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/state"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/core/vm"
	"github.com/ethereum/go-ethereum/params"
	"github.com/ethereum/go-ethereum/triedb"
	"github.com/holiman/uint256"
	"github.com/spf13/cobra"
)

var (
	contractCodePath string
	calldata         string
	numRuns          int
)

func check(e error) {
	if e != nil {
		fmt.Fprintln(os.Stderr, e)
		os.Exit(1)
	}
}

var cmd = &cobra.Command{
	Use:   "runner-geth",
	Short: "go-ethereum runner for evm-bench",
	Run: func(_ *cobra.Command, _ []string) {
		contractCodeHex, err := os.ReadFile(contractCodePath)
		check(err)

		contractCodeBytes := common.Hex2Bytes(string(contractCodeHex))
		calldataBytes := common.Hex2Bytes(calldata)

		zeroAddress := common.BytesToAddress(common.FromHex("0x0000000000000000000000000000000000000000"))
		callerAddress := common.BytesToAddress(common.FromHex("0x1000000000000000000000000000000000000001"))

		config := params.MainnetChainConfig
		blockNumber := config.GrayGlacierBlock
		blockTime := config.CancunTime
		rules := config.Rules(blockNumber, true, *blockTime)

		var config1 *triedb.Config
		db := rawdb.NewMemoryDatabase()
		statedb, err := state.New(common.Hash{}, state.NewDatabase(triedb.NewDatabase(db, config1), nil))
		check(err)

		zeroValue := big.NewInt(0)
		gasLimit := ^uint64(0)
		zeroHash := common.Hash{}

		blockContextHeader := types.Header{
			ParentHash:       [32]byte{},
			UncleHash:        [32]byte{},
			Coinbase:         [20]byte{},
			Root:             [32]byte{},
			TxHash:           [32]byte{},
			ReceiptHash:      [32]byte{},
			Bloom:            [256]byte{},
			Difficulty:       &big.Int{},
			Number:           blockNumber,
			GasLimit:         gasLimit,
			GasUsed:          0,
			Time:             *blockTime,
			Extra:            []byte{},
			MixDigest:        [32]byte{1},
			Nonce:            [8]byte{},
			BaseFee:          &big.Int{},
			WithdrawalsHash:  &zeroHash,
			BlobGasUsed:      new(uint64),
			ExcessBlobGas:    nil,
			ParentBeaconRoot: &zeroHash,
			RequestsHash: &zeroHash,
		}
		
		blockContext := core.NewEVMBlockContext(&blockContextHeader, nil, &zeroAddress)

		createMsg := core.Message{
			To:                &zeroAddress,
			From:              callerAddress,
			Nonce:             0,
			Value:             zeroValue,
			GasLimit:          gasLimit,
			GasPrice:          zeroValue,
			GasFeeCap:         zeroValue,
			GasTipCap:         zeroValue,
			Data:              contractCodeBytes,
			AccessList:        []types.AccessTuple{},
			BlobGasFeeCap:     zeroValue,
			BlobHashes:        []common.Hash{},
			SetCodeAuthorizations: []types.SetCodeAuthorization{},
			SkipNonceChecks: true,
			SkipFromEOACheck: true,
		}

		statedb.Prepare(rules, callerAddress, blockContext.Coinbase, &zeroAddress, vm.ActivePrecompiles(rules), createMsg.AccessList)
		evm := vm.NewEVM(blockContext, statedb, config, vm.Config{})
		_, contractAddress, _, err := evm.Create(callerAddress, contractCodeBytes, gasLimit, uint256.NewInt(0))
		check(err)

		msg := core.Message{
			To:                &contractAddress,
			From:              callerAddress,
			Nonce:             1,
			Value:             zeroValue,
			GasLimit:          gasLimit,
			GasPrice:          zeroValue,
			GasFeeCap:         zeroValue,
			GasTipCap:         zeroValue,
			Data:              calldataBytes,
			AccessList:        []types.AccessTuple{},
			BlobGasFeeCap:     zeroValue,
			BlobHashes:        []common.Hash{},
			SetCodeAuthorizations: []types.SetCodeAuthorization{},
			SkipNonceChecks: true,
			SkipFromEOACheck: true,
		}
		for i := 0; i < numRuns; i++ {
			snapshot := statedb.Snapshot()
			statedb.Prepare(rules, msg.From, blockContext.Coinbase, msg.To, vm.ActivePrecompiles(rules), msg.AccessList)

			start := time.Now()
			_, _, err := evm.Call(callerAddress, *msg.To, msg.Data, msg.GasLimit, uint256.MustFromBig(msg.Value))
			timeTaken := time.Since(start)

			fmt.Println(float64(timeTaken.Microseconds()) / 1e3)

			check(err)

			statedb.RevertToSnapshot(snapshot)
		}
	},
}

func init() {
	cmd.Flags().StringVar(&contractCodePath, "contract-code-path", "", "Path to the hex contract code to deploy and run")
	cmd.MarkFlagRequired("contract-code-path")
	cmd.Flags().StringVar(&calldata, "calldata", "", "Hex of calldata to use when calling the contract")
	cmd.MarkFlagRequired("calldata")
	cmd.Flags().IntVar(&numRuns, "num-runs", 0, "Number of times to run the benchmark")
	cmd.MarkFlagRequired("num-runs")
}

func main() {
	if err := cmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
