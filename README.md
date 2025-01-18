# MUJI LANG

## Project Description
The language supports basic arithmetic operations, input/output operations, string operations, and conditional statements.

## Installation Instructions
To install and set up the project, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/konseptt/nepalibhasas.git
   cd nepalibhasas
   ```

2. Build the project:
   ```sh
   cargo build
   ```

3. Run the project:
   ```sh
   cargo run
   ```

## Usage Examples
Here are some examples of how to use the Nepali Bhasas programming language:

1. Variables and Basic Arithmetic:
   ```nbh
   bol mug "=== Variables and Arithmetic ==="
   oi mug a = 10
   oi mug b = 5
   mug jod a, b lai sum
   bol mug "Sum: " + sum
   mug ghata a, b lai diff
   bol mug "Difference: " + diff
   bol mug ""
   ```

2. Input/Output Operations:
   ```nbh
   bol mug "=== Input/Output Demo ==="
   bol mug "Timro naam k ho?"
   oi mug bhan userName
   bol mug "Namaste " + userName + "!"
   bol mug ""
   ```

3. String Operations:
   ```nbh
   bol mug "=== String Operations ==="
   oi mug firstName = "Ram"
   oi mug lastName = "Bahadur"
   oi mug fullName = firstName + " " + lastName
   bol mug "Full Name: " + fullName
   bol mug ""
   ```

4. Simple Conditional:
   ```nbh
   bol mug "=== Simple Conditional ==="
   bol mug "Timro umer kati ho?"
   oi mug bhan age
   yedi age babaal "18" bhane
       bol mug "Tapai adult hunu huncha"
   sakiyo
   bol mug ""
   ```

5. Multiple Conditions Chain:
   ```nbh
   bol mug "=== Multiple Conditions ==="
   bol mug "Timro favourite color k ho?"
   oi mug bhan color

   yedi color babaal "rato" bhane
       bol mug "Rato rang maya ko rang ho"
   sakiyo
   aile feri color babaal "nilo" bhane
       bol mug "Nilo rang akash jastai ho"
   sakiyo
   aile feri color babaal "hariyo" bhane
       bol mug "Hariyo rang prakrti ko rang ho"
   sakiyo
   ```

## Contribution Guide
We welcome contributions to the Nepali Bhasas project! To contribute, please follow these guidelines:

1. Fork the repository and create a new branch for your feature or bugfix.
2. Write tests for your changes and ensure all existing tests pass.
3. Submit a pull request with a clear description of your changes.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
