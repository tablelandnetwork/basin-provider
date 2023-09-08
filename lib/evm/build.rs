fn main() {
    ethers::prelude::Abigen::new("BasinStorage", "./contracts/BasinStorage.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("./src/contract.rs")
        .unwrap();
}
