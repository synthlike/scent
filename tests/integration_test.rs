use scent::analysis::FunctionSelector;
use scent::analysis::analyze_function_selectors;
use scent::parser::parse_bytecode;

#[test]
fn counter_contract_function_selectors() {
    let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b506101b88061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c80633fb5c1cb146100435780638381f58a1461005f578063d09de08a1461007d575b5f5ffd5b61005d600480360381019061005891906100e4565b610087565b005b610067610090565b604051610074919061011e565b60405180910390f35b610085610095565b005b805f8190555050565b5f5481565b5f5f8154809291906100a690610164565b9190505550565b5f5ffd5b5f819050919050565b6100c3816100b1565b81146100cd575f5ffd5b50565b5f813590506100de816100ba565b92915050565b5f602082840312156100f9576100f86100ad565b5b5f610106848285016100d0565b91505092915050565b610118816100b1565b82525050565b5f6020820190506101315f83018461010f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61016e826100b1565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036101a05761019f610137565b5b60018201905091905056fea164736f6c634300081e000a").expect("invalid hex"); // counter.sol
    let instructions = parse_bytecode(&input);
    let selectors = analyze_function_selectors(&instructions);

    assert_eq!(selectors.len(), 3);

    assert_eq!(
        selectors[0],
        FunctionSelector {
            offset: 59,
            selector: [0x3f, 0xb5, 0xc1, 0xcb],
            name: Some("func_3fb5c1cb".to_string())
        }
    );
    assert_eq!(
        selectors[1],
        FunctionSelector {
            offset: 70,
            selector: [0x83, 0x81, 0xf5, 0x8a],
            name: Some("func_8381f58a".to_string())
        }
    );
    assert_eq!(
        selectors[2],
        FunctionSelector {
            offset: 81,
            selector: [0xd0, 0x9d, 0xe0, 0x8a],
            name: Some("func_d09de08a".to_string())
        }
    );
}
// must be run via cargo test -- --nocapture
// #[test]
// fn print_empty_contract_bytecode() {
//     let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b50601580601a5f395ff3fe60806040525f5ffdfea164736f6c634300081e000a").expect("invalid hex"); // empty.sol
//     let instructions = parse_bytecode(&input);
//     let view = View::from_instructions(&instructions, true);
//     view.print_entries();
// }

// #[test]
// fn print_return_constant_bytecode() {
//     let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b50608680601a5f395ff3fe6080604052348015600e575f5ffd5b50600436106026575f3560e01c80632096525514602a575b5f5ffd5b60306044565b604051603b91906062565b60405180910390f35b5f602a905090565b5f819050919050565b605c81604c565b82525050565b5f60208201905060735f8301846055565b9291505056fea164736f6c634300081e000a").expect("invalid hex"); // return_const.sol
//     let instructions = parse_bytecode(&input);
//     let view = View::from_instructions(&instructions, true);
//     view.print_entries();
// }

// #[test]
// fn print_counter_bytecode() {
//     let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b506101b88061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c80633fb5c1cb146100435780638381f58a1461005f578063d09de08a1461007d575b5f5ffd5b61005d600480360381019061005891906100e4565b610087565b005b610067610090565b604051610074919061011e565b60405180910390f35b610085610095565b005b805f8190555050565b5f5481565b5f5f8154809291906100a690610164565b9190505550565b5f5ffd5b5f819050919050565b6100c3816100b1565b81146100cd575f5ffd5b50565b5f813590506100de816100ba565b92915050565b5f602082840312156100f9576100f86100ad565b5b5f610106848285016100d0565b91505092915050565b610118816100b1565b82525050565b5f6020820190506101315f83018461010f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61016e826100b1565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036101a05761019f610137565b5b60018201905091905056fea164736f6c634300081e000a").expect("invalid hex"); // counter.sol
//     let instructions = parse_bytecode(&input);
//     let view = View::from_instructions(&instructions, true);
//     view.print_entries();
// }
