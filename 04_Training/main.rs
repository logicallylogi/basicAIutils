use classifier::NaiveBayes;
use std::fs;
use serde_json::Value;

#[test]
fn food_document_test() {
    
    // Do initial setup
    let mut nb = NaiveBayes::new();

    // Declare what examples is
    let examples = vec![];

    // Pull examples from a file
    let json: String = fs::read_to_string("model.csv")?.parse()?;
    let v: Value = serde_json::from_str(json)?;
    for i in v.iter() {
        match i {
            examples.push(i["example"], i["rating"]);
        }
    }

    // Add every example to the classifier
    for &(document, label) in examples.iter() {
        nb.add_document(&document.to_string(), &label.to_string());
    }

    // Train the classifier
    nb.train();

    // Export the classifier
    &nb.to_json()