use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Algorithm {
    Sha256,
    Keccak256,
    Blake3,
}
