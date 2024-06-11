


pub mod ocr{
    use image::Rgba;
    use oasysdb::err::Error;
    use ocrs::ImageSource;


    pub struct OCRService{
        engine: ocrs::OcrEngine
    }
    
    
    impl OCRService{
        pub fn new(detection_model_path: &str, recognition_model_path: &str) -> OCRService{

            let detection_model = rten::Model::load_file(detection_model_path).unwrap();
            let recognition_model = rten::Model::load_file(recognition_model_path).unwrap();
    
            let engine = ocrs::OcrEngine::new( ocrs::OcrEngineParams {
                detection_model: Some(detection_model),
                recognition_model: Some(recognition_model),
                ..Default::default()
            }).unwrap();
    
            Self {
                engine
            }
        }

        pub fn get_text_from_image(&self, image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<String, Box<dyn std::error::Error>>{

            let img_source = ImageSource::from_bytes(image.as_raw(), image.dimensions())?;
            let ocr_input = self.engine.prepare_input(img_source)?;

            let text = self.engine.get_text(&ocr_input)?;
            
            return Ok(text)
        }


        pub fn get_lines_from_image(&self, image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<String>{

            let img_source = ImageSource::from_bytes(image.as_raw(), image.dimensions()).unwrap();
            let ocr_input = self.engine.prepare_input(img_source).unwrap();


            let word_rects = self.engine.detect_words(&ocr_input).unwrap();

            let line_rects = self.engine.find_text_lines(&ocr_input, &word_rects);

            let line_texts = self.engine.recognize_text(&ocr_input, &line_rects).unwrap();

            let line_strings: Vec<String> = line_texts.iter()
            .flatten()
            .filter(|l| l.to_string().len() > 1)
            .map(| line | line.to_string())
            .collect();

            return line_strings;
        }

    }


}