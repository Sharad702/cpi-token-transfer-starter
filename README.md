# CPI: Token Transfer via Program Challenge

Build a Solana program that performs Cross-Program Invocations (CPI) to transfer SPL tokens.

## 🎯 Objective

Create a Solana program that:
1. Performs token transfers via CPI
2. Handles PDA signers for CPI
3. Implements a payment splitter (fees to protocol)
4. Validates all CPI accounts properly

## 📋 Requirements

### Core Features
- [ ] Transfer tokens via CPI to Token Program
- [ ] PDA-signed CPI for vault operations
- [ ] Payment splitter (user payment → protocol fee + recipient)
- [ ] Proper CPI account validation

### Security Requirements
- [ ] Validate Token Program ID in CPI
- [ ] Verify token account mints
- [ ] Check sufficient balances before CPI
- [ ] Prevent CPI to malicious programs

## 🏗️ Project Structure

```
cpi-token-transfer-starter/
├── programs/
│   └── cpi-token-transfer/
│       └── src/
│           └── lib.rs          # Your program logic
├── tests/
│   └── cpi-token-transfer.ts   # Test file
├── Anchor.toml
├── Cargo.toml
└── package.json
```

## 🚀 Getting Started

### Prerequisites
- Rust & Cargo
- Solana CLI
- Anchor Framework

### Installation

```bash
npm install
anchor build
anchor test
```

## 📝 Evaluation Criteria

| Criteria | Weight |
|----------|--------|
| **Correctness** | Pass/Fail |
| **CPI Implementation** | 35% |
| **Security** | 40% |
| **Code Quality** | 25% |

### What We Check
- **CPI Implementation:** Correct CPI calls, PDA signers
- **Security:** Program ID validation, account checks
- **Code Quality:** Clean Rust code, proper error handling

## 📚 Resources

- [Solana CPI Documentation](https://docs.solana.com/developing/programming-model/calling-between-programs)
- [Anchor CPI Guide](https://www.anchor-lang.com/docs/cpi)
- [SPL Token Transfer](https://spl.solana.com/token#transferring-tokens)

## 🔗 Submission

1. Fork this repository
2. Implement the CPI program
3. Ensure all tests pass
4. Submit your repo URL on the platform

Good luck! 🎉
# cpi-token-transfer-starter
