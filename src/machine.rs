use super::tokenizer::Token;
use super::tokenizer::Tokenizer;

/**
 * The Machine maintains a stack to store intermediate values of computations.
 *
 * Author: <author>
 * ONYEN: <onyen>
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */
pub struct Machine {
    stack: Vec<f64>,
}

impl Machine {
    /**
     * Default constructor starts with an empty stack.
     */
    pub fn new() -> Machine {
        Machine { stack: Vec::new() }
    }

    /**
     * Internal constructor simplify test case scenarios. Not public.
     */
    fn from(stack: Vec<f64>) -> Machine {
        Machine { stack }
    }

    /**
     * Given an input string, eval carries out its instructions. This is
     * the only public method of a Machine.
     */
    pub fn eval(&mut self, input: &str) -> Vec<String> {
        let mut output = Vec::new();

        output.push(format!("Implement me! The input was: {}", input));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut m = Machine::new();
        assert_eq!(0, m.stack.len());
    }

    #[test]
    fn from() {
        let mut m = Machine::from(vec![1.0]);
        assert_eq!(1, m.stack.len());
        assert_eq!(1.0, m.stack[0]);

        let mut m = Machine::from(vec![1.0, 2.0]);
        assert_eq!(2, m.stack.len());
        assert_eq!(1.0, m.stack[0]);
        assert_eq!(2.0, m.stack[1]);
    }
}
