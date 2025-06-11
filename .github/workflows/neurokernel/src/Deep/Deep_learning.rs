use burn::{
    module::Module,
    nn,
    tensor::{backend::ndarray::NdArrayBackend, Tensor},
};

type Backend = NdArrayBackend<f32>;

#[derive(Module, Debug)]
struct NeuroMemory {
    input_layer: nn::Linear<Backend>,
    output_layer: nn::Linear<Backend>,
}

impl NeuroMemory {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        Self {
            input_layer: nn::LinearConfig::new(input_size, hidden_size).init(),
            output_layer: nn::LinearConfig::new(hidden_size, output_size).init(),
        }
    }

    fn forward(&self, input: Tensor<Backend, 2>) -> Tensor<Backend, 2> {
        let hidden = self.input_layer.forward(input).relu();
        self.output_layer.forward(hidden)
    }
}

// This will be called by chatbot
pub fn get_user_vector(brain_score: f32, chat_freq: f32, form_score: f32) -> Vec<f32> {
    let input_data = Tensor::<Backend, 2>::from_floats([[brain_score, chat_freq, form_score]]);

    let model = NeuroMemory::new(3, 4, 2); // 3 input fields → 2 output features
    let output = model.forward(input_data);

    output.to_data().value[0].clone()
}
