extern crate gene;
extern crate gene = "gene#0.1";

#[deriving(Show)]

fn main() {
	let g1 = gene::BaseGene { value : 1.0 };
	let g2 = gene::BaseGene { value : 2.0 };

	println!("genes {}", g1 + g2)


}