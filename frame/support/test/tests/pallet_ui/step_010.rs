#[frame_support::pallet]
mod pallet {
	#[pallet::trait_]
	pub trait Trait: frame_system::Trait {}

	#[pallet::module]
	pub struct Foo {}
}

fn main() {
}