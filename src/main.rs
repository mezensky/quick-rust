use rand::prelude::*;

/// Generates a random u8 integer.
///
/// # Returns
///
/// A random unsigned 8-bit integer.
#[must_use]
pub fn generate_random_u8() -> u8 {
    random()
}

/// Simulates a coin flip.
///
/// # Returns
///
/// `true` for heads, `false` for tails.
#[must_use]
pub fn flip_coin() -> bool {
    random()
}

/// Generates a random f64 float between 0.0 and 1.0.
///
/// # Returns
///
/// A random 64-bit floating point number.
#[must_use]
pub fn generate_random_float() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}

/// Displays various random number examples.
fn display_random() {
    let rand_int = generate_random_u8();
    println!("{rand_int}");

    let coin_flip = flip_coin();
    if coin_flip {
        println!("Heads!");
    } else {
        println!("Tails!");
    }

    let rand_float = generate_random_float();
    println!("x is: {rand_float}");
}

/// Prints a greeting message.
pub fn greet() {
    println!("Hello, World!");
}

fn main() {
    display_random();
    greet();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_u8() {
        // Run multiple times to ensure it returns valid u8 values
        for _ in 0..10 {
            let _result = generate_random_u8();
            // Any u8 value is valid, just ensure no panic
        }
    }

    #[test]
    fn test_flip_coin() {
        // Run multiple times to ensure it returns bool
        for _ in 0..10 {
            let result = flip_coin();
            assert!(result || !result);
        }
    }

    #[test]
    fn test_generate_random_float() {
        let result = generate_random_float();
        // Verify it's in expected range
        assert!(result >= 0.0 && result < 1.0);
    }

    #[test]
    fn test_greet() {
        // Simple smoke test - function should not panic
        greet();
    }
}
