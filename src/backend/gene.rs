use tch::Tensor;
use tch::nn::Module;

#[derive(Debug, Clone, Copy)]
pub struct Genome {
	module: Module
}

#[derive(Debug)]
pub struct GenomePool {

}

impl GenomePool {
	pub fn new() -> Self {
		Self {

		}
	}
}