use alloy::sol;

sol!(Contract, "src/Contract.json");

sol! {
    contract Contract2 {
        enum Enum2 {
            A2,
            B2,
            C2
        }
    }
}

// doesn't compile
fn main() {
    let a = Contract::Enum::A; // throws an error
    let a2 = Contract2::Enum2::A2; // doesn't throw an error
}
