use super::evm::opcodes::*;

impl WrappedOpcode {

    // Returns a WrappedOpcode's solidity representation.
    pub fn solidify(&self) -> String {
        let mut solidified_wrapped_opcode = String::new();

        match self.opcode.name.as_str() {
            "ADD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} + {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "MUL" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} * {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SUB" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} - {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "DIV" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} / {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SDIV" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} / {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "MOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "{} % ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SMOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) % ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "ADDMOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({} + {}) % {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify(),
                            self.inputs[2]._solidify()
                    ).as_str()
                );
            },
            "MULMOD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({} * {}) % {}",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify(),
                            self.inputs[2]._solidify()
                    ).as_str()
                );
            },
            "EXP" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) ** ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "LT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) < ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "GT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) > ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SLT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) < ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SGT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) > ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "EQ" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) == ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "ISZERO" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) == 0",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "AND" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) & ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "OR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) | ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "XOR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) ^ ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "NOT" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "~({})",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "SHL" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) << ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SHR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) >> ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SAR" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "({}) >> ({})",
                            self.inputs[0]._solidify(),
                            self.inputs[1]._solidify()
                    ).as_str()
                );
            },
            "SHA3" => {
                // TODO: use memory from function
                solidified_wrapped_opcode.push_str("keccak256()");
            },
            "ADDRESS" => {
                solidified_wrapped_opcode.push_str("address(this)");
            },
            "BALANCE" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "address({}).balance",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "ORIGIN" => {
                solidified_wrapped_opcode.push_str("tx.origin");
            },
            "CALLER" => {
                solidified_wrapped_opcode.push_str("msg.sender");
            },
            "CALLVALUE" => {
                solidified_wrapped_opcode.push_str("msg.value");
            },
            "CALLDATALOAD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "msg.data[{}]",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "CALLDATASIZE" => {
                solidified_wrapped_opcode.push_str("msg.data.length",);
            },
            "CODESIZE" => {
                solidified_wrapped_opcode.push_str("this.code.length",);
            },
            "EXTCODESIZE" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "address({}).code.length",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "EXTCODEHASH" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "address({}).codehash",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "BLOCKHASH" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "blockhash({})",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "COINBASE" => {
                solidified_wrapped_opcode.push_str("block.coinbase");
            },
            "TIMESTAMP" => {
                solidified_wrapped_opcode.push_str("block.timestamp");
            },
            "NUMBER" => {
                solidified_wrapped_opcode.push_str("block.number");
            },
            "DIFFICULTY" => {
                solidified_wrapped_opcode.push_str("block.difficulty");
            },
            "GASLIMIT" => {
                solidified_wrapped_opcode.push_str("block.gaslimit");
            },
            "CHAINID" => {
                solidified_wrapped_opcode.push_str("block.chainid");
            },
            "SELFBALANCE" => {
                solidified_wrapped_opcode.push_str("address(this).balance");
            },
            "BASEFEE" => {
                solidified_wrapped_opcode.push_str("block.basefee");
            },
            "GAS" => {
                solidified_wrapped_opcode.push_str("gasleft()");
            },
            "GASPRICE" => {
                solidified_wrapped_opcode.push_str("tx.gasprice");
            },
            "SLOAD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "storage[{}]",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "MLOAD" => {
                solidified_wrapped_opcode.push_str(
                    format!(
                            "memory[{}]",
                            self.inputs[0]._solidify()
                    ).as_str()
                );
            },
            "MSIZE" => {
                solidified_wrapped_opcode.push_str(
                    "memory.length"
                );
            },
            "CALL" => {
                solidified_wrapped_opcode.push_str("ret0");
            }
            "CALLCODE" => {
                solidified_wrapped_opcode.push_str("ret0");
            }
            "DELEGATECALL" => {
                solidified_wrapped_opcode.push_str("ret0");
            }
            "STATICCALL" => {
                solidified_wrapped_opcode.push_str("ret0");
            }
            opcode => {

                if opcode.starts_with("PUSH") {
                    solidified_wrapped_opcode.push_str(
                        format!(
                                "{}",
                                self.inputs[0]._solidify()
                        ).as_str()
                    );
                }
                else {
                    solidified_wrapped_opcode.push_str(
                        format!(
                                "{}",
                                opcode
                        ).as_str()
                    );
                }
            }
        }

        solidified_wrapped_opcode
    }

    // creates a new WrappedOpcode from a set of raw inputs
    pub fn new(opcode_int: usize, inputs: Vec<WrappedInput>) -> WrappedOpcode {
        let mut opcode_str = format!("{:x}", opcode_int);
        if opcode_str.len() == 1 {
            opcode_str.insert(0, '0');
        };

        WrappedOpcode {
            opcode: opcode(&opcode_str),
            inputs: inputs,
        }
    }


    pub fn default() -> WrappedOpcode {
        WrappedOpcode {
            opcode: Opcode {
                name: String::from("unknown"),
                mingas: 0,
                inputs: 0,
                outputs: 0,
            },
            inputs: Vec::new(),
        }
    }

}

impl WrappedInput {

    // Returns a WrappedInput's solidity representation.
    fn _solidify(&self) -> String {
        let mut solidified_wrapped_input = String::new();

        match self {
            WrappedInput::Raw(u256) => {
                solidified_wrapped_input.push_str(
                    u256.to_string().as_str()
                );
            }
            WrappedInput::Opcode(opcode) => {
                solidified_wrapped_input.push_str(
                    format!(
                        "{}",
                        opcode.solidify()
                    ).as_str()
                );
            },
        }

        solidified_wrapped_input
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ethers::types::U256;

    #[test]
    fn test_solidify_add() {

        // wraps an ADD operation with 2 raw inputs
        let add_operation_wrapped = WrappedOpcode::new(0x01, vec![WrappedInput::Raw(U256::from(1u8)), WrappedInput::Raw(U256::from(2u8))]);
        assert_eq!(add_operation_wrapped.solidify(), "1 + 2");
        
    }

    #[test]
    fn test_solidify_add_complex() {

        // wraps an ADD operation with 2 raw inputs
        let add_operation_wrapped = WrappedOpcode::new(0x01, vec![WrappedInput::Raw(U256::from(1u8)), WrappedInput::Raw(U256::from(2u8))]);
        let complex_add_operation = WrappedOpcode::new(0x01, vec![WrappedInput::Opcode(add_operation_wrapped), WrappedInput::Raw(U256::from(3u8))]);
        assert_eq!(complex_add_operation.solidify(), "1 + 2 + 3");
        
    }
}