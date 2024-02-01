#![no_std]

dharitri_wasm::imports!();

/// Explores panic messaging.
/// Sending panic messages to the VM is possible, as shown in this contract,
/// but it greatly inflates the bytecode size.
#[dharitri_wasm::contract]
pub trait PanicMessageFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint(panicWithMessage)]
    fn panic_with_message(&self) {
        panic!("example panic message");
    }
}
