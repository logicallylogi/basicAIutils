from tflite_model_maker import model_spec
from tflite_model_maker import text_classifier
from tflite_model_maker.config import ExportFormat
from tflite_model_maker.text_classifier import DataLoader

def main():
    spec = model_spec.get('mobilebert_classifier')
    train_data = DataLoader.from_folder(
        "model", model_spec=spec, is_training=True,
        class_labels=None, shuffle=True, cache_dir=None
    )
    model = text_classifier.create(train_data, model_spec=spec, epochs=20)
    model.export(export_dir='models/latest')

main()