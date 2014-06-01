mod gene {

	pub struct BaseGene {	
	    value : f64,
	}

	trait Gamete {
		
	}

	pub trait Gene {
		fn get_finess(&self);
		fn mutate(&self);
		fn copy(&self);
	}

	impl Add<BaseGene, BaseGene> for BaseGene {
		pub fn add(&self, rhs: &BaseGene) -> BaseGene {
			return BaseGene {value : self.value + rhs.value};	
		}
	}

	impl Gene for BaseGene {
		pub fn get_finess(&self) {
			
		}
		pub fn mutate(&self) {

		}
		pub fn copy(&self) {
			
		}
	}
}
