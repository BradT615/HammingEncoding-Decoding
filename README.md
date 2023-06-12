# Hamming Encoding and Decoding Program

This program demonstrates encoding and decoding processes using a modified Hamming code algorithm. The encoding process converts characters into binary representations and adds redundant bits based on the Hamming code. The decoding process reverses the encoding and verifies the correctness of the Hamming code.

## Usage

1. Make sure you have Rust installed on your system.

2. Clone the repository or download the source code files.

3. Provide the input files:
   - **Encoding**: Place the text you want to encode in the `Homework #3 - Encoding Text.txt` file.
   - **Decoding**: Place the encoded text in the `Homework #3 - Decoding Text.txt` file.

4. Open a terminal or command prompt and navigate to the project directory.

5. Build and run the program using the following command:
   ```
   cargo run
   ```

6. The program will execute the encoding and decoding processes and display the results in the terminal.

## Program Overview

The program consists of the following main components:

### Encoding Process

The encoding process reads lines from the `Homework #3 - Encoding Text.txt` file and performs the following steps for each character:

1. Convert the character into its binary representation using ASCII encoding.

2. Add redundant bits based on the Hamming code algorithm.

3. Print the character, its token, and the Hamming code.

### Decoding Process

The decoding process reads lines from the `Homework #3 - Decoding Text.txt` file and performs the following steps for each line:

1. Convert the ASCII-encoded characters into integers.

2. Convert the integers into binary representations.

3. Verify the correctness of the Hamming code by checking the redundant bits.

4. Convert the binary digits back into characters and print them.

### Other Functions

The program also includes several utility functions that support the encoding and decoding processes:

- `pull_lines`: Reads the contents of a file and splits it into lines.

- `pull_chars`: Converts a string into a vector of characters.

- `find_binary`: Converts a character into a binary representation using ASCII encoding.

- `find_hamming_binary`: Calculates the redundant bits (R1, R2, R4, and R8) for a binary vector based on the Hamming code algorithm.

- `find_token`: Calculates the token (decimal representation) for a binary vector based on the bits starting from index 5.

- `char_to_ascii`: Converts a string into a vector of ASCII values.

- `ascii_to_int`: Converts a vector of ASCII values into a vector of integers.

- `binary`: Converts an integer into a vector of binary digits.

- `check`: Verifies the correctness of the Hamming code by checking the values of the redundant bits.

- `convert`: Converts a binary vector into characters using the Hamming code algorithm.

## File Structure

The program consists of the following files:

- `main.rs`: The main source code file containing the program logic.

- `Homework #3 - Encoding Text.txt`: The input file for the encoding process, containing the text to be encoded.

- `Homework #3 - Decoding Text.txt`: The input file for the decoding process, containing the encoded text.

## License

This program is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Acknowledgments

The program is based on the concept of Hamming codes and utilizes the Rust programming language. It serves as an educational example and demonstrates encoding and decoding processes using a modified Hamming code algorithm.
