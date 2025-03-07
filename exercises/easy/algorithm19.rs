/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
struct Matrix([[i64; 2]; 2]);

impl Matrix {
    // Multiply two matrices
    fn multiply(&self, other: &Matrix) -> Matrix {
        let a = self.0;
        let b = other.0;
        Matrix([
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1],
            ],
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1],
            ],
        ])
    }

    // Raise the matrix to the power of n using exponentiation by squaring
    fn pow(&self, n: i32) -> Matrix {
        if n == 0 {
            return Matrix([[1, 0], [0, 1]]); // Identity matrix
        }
        if n == 1 {
            return self.clone();
        }
        let mut result = Matrix([[1, 0], [0, 1]]);
        let mut base = self.clone();
        let mut exp = n as u32;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.multiply(&base);
            }
            base = base.multiply(&base);
            exp /= 2;
        }

        result
    }
}

pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    // Define the transformation matrix
    let matrix = Matrix([[1, 1], [1, 0]]);
    // Compute the (n-1)th power of the matrix
    let powered_matrix = matrix.pow(n - 1);
    // The nth Fibonacci number is in the top left corner of the resulting matrix
    powered_matrix.0[0][0] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
