# Triangle Classification Program

This is a simple Rust program that allows you to classify a triangle based on its side lengths. It takes user input for the lengths of the triangle's sides and determines whether it's equilateral, isosceles, or scalene.

## Prerequisites

Before running this program, you need to have Rust installed on your system. You can download and install Rust from the official website: [Rust - Install](https://www.rust-lang.org/tools/install)

## Usage

1. Clone this repository to your local machine or download the source code.
2. Navigate to the project directory: `cd rust-triangle-classification`
3. Compile and run: `cargo run`
4. Follow the prompts to enter the lengths of the triangle's sides (x, y, and z) when prompted.

5. The program will classify the triangle based on the side lengths and display the result.

## Code Structure

- `main.rs`: The main program logic, including user input and triangle classification.
- `lib.rs`: The module defining the `Triangle` struct and its methods.
- `README.md`: This README file.

## How It Works

The program uses a `Triangle` struct to represent a triangle with three side lengths (x, y, z). It classifies the triangle into one of the following types:

- Equilateral: All three sides are of equal length.
- Isosceles: At least two sides are of equal length.
- Scalene: No sides are of equal length.

The user is prompted to enter the lengths of the sides, and the program determines the triangle type based on these inputs.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This program was created as a learning exercise in Rust programming.

Feel free to modify and use this README template for your project, adding more details and sections as needed.