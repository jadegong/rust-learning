pub mod learning;
pub mod algorithm;

#[cfg(test)]
pub mod tests;

fn main() {
    // println!("Hello, rust!");
    learning::learn_ferris_says();

    // algorithm::array_algorithm::test_remove_element();
}