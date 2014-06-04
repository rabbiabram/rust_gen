#![crate_id = "gene#0.1"]
extern crate core;
	
pub struct BaseGene {	
    pub value : f64,
}

trait Gamete {
	
}

pub trait Gene {
	fn get_finess(&self);
	fn mutate(&self);
	fn copy(&self);
}

impl Add<BaseGene, BaseGene> for BaseGene {
	fn add(&self, rhs: &BaseGene) -> BaseGene {
		return BaseGene {value : self.value + rhs.value};	
	}
}

impl core::fmt::Show for BaseGene {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		core::fmt::secret_show(&self, f)
	}
}

impl Gene for BaseGene {
	fn get_finess(&self) {
		
	}
	fn mutate(&self) {

	}
	fn copy(&self) {
		
	}
}
