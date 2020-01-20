# Cryptography tools and utilities from TTM4135 - Applied Cryptography and Network Security

![](https://github.com/fredrikaugust/rusty-crypto/workflows/Rust/badge.svg)

_This is some of the first code I've ever written in Rust, and the code quality reflects that. I do not plan on going back to clean up code later, as I'm mainly doing this to learn Rust and some cryptography techniques._

- Euler's totient function
  - _Number of relatively prime numbers up to n_
- Euclidean (and Extended) Algorithm
  - _Find greatest common divisor for two numbers_
- Bèzout Coefficients
  - _Use Extended Euclidean Algorithm to find ax+by=d=gcd(a,b)_
- Inverse modulo
  - _Find inverse modulo of a number_
- Matrix multiplication
  - _Multiply two matrices_
- Kasiski examination
  - _Attempt to figure out key length in polyalphabetic substitution cipher_
- Miller-Rabin test
  - _Check if number is prime_
- Laplace method
  - _Find determinant by cofactor expension_
- Remove row+col from matrix (submatrix)
  - _Get submatrix by removing a row and column (used in Laplace)_
- Scalar matrix multiplication
  - _Multiply matrix by a scalar_
- Find inverse of matrix
  - _Use Laplace to find inverse of a matrix (used in Hill cipher)_
- Hill cipher encrypt
  - _Encrypt a plaintext using a key matrix_
- Matrix modulo
  - _Perform modulo on all cells of matrix_
- Modular multiplicative inverse of matrix
  - _Find the modular multiplicative inverse of a matrix_
- Hill cipher decrypt
  - _Decrypt a hill cipher given a key_
