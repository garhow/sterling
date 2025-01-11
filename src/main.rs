mod model;

use model::{get_model_by_lang, Model};
use vosk::Recognizer;

fn main() {
    let current_model: Model = get_model_by_lang("en").unwrap();
    current_model.install().unwrap();
    if !current_model.is_installed() {
        return;
    }

    let samples = vec![100, -2, 700, 30, 4, 5];

    let model = vosk::Model::new(current_model.get_local_path()).unwrap();
    let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample).unwrap();
        println!("{:#?}", recognizer.partial_result());
    }

    println!("{:#?}", recognizer.final_result().multiple().unwrap());
}
