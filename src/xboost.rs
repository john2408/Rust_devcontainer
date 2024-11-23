// extern crate xgboost;
// extern crate rand;

// use rand::seq::SliceRandom;
// use xgboost::{TrainOptions, Booster, DMatrix};

// fn main() {
//     // Load the iris dataset
//     let iris = iris::load_data().unwrap();
//     let (x, y) = (iris.data, iris.target);

//     // Shuffle the data and split into training and testing sets
//     let mut data: Vec<_> = x.iter().zip(y.iter()).collect();
//     data.shuffle(&mut rand::thread_rng());

//     let split_idx = (data.len() as f32 * 0.8) as usize;
//     let (train_x, train_y) = data[..split_idx].iter().cloned().unzip();
//     let (test_x, test_y) = data[split_idx..].iter().cloned().unzip();

//     // Convert the training and testing datasets to DMatrices
//     let dtrain = DMatrix::from_dense(&train_x).unwrap();
//     let dtest = DMatrix::from_dense(&test_x).unwrap();

//     // Convert the labels to floats
//     let train_labels: Vec<f32> = train_y.iter().map(|&x| x as f32).collect();
//     let test_labels: Vec<f32> = test_y.iter().map(|&x| x as f32).collect();

//     // Set the parameters for the XGBoost model
//     let params = vec![
//         ("objective", "multi:softmax"),
//         ("num_class", &iris.target_names.len().to_string()),
//         ("max_depth", "5"),
//         ("eta", "0.1"),
//         ("subsample", "0.5"),
//         ("colsample_bytree", "0.5"),
//     ];

//     // Train the XGBoost model
//     let mut booster = Booster::train(&dtrain, &train_labels, &TrainOptions::new().set_params(&params)).unwrap();

//     // Evaluate the accuracy of the model on the test set
//     let preds = booster.predict(&dtest).unwrap();
//     let test_acc = preds
//         .iter()
//         .zip(&test_labels)
//         .filter(|(&pred, &label)| pred.round() == label)
//         .count() as f32
//         / test_labels.len() as f32;

//     println!("Test accuracy: {:.2}", test_acc);

//     // Save the trained model
//     booster.save_model("iris.model").unwrap();
// }
