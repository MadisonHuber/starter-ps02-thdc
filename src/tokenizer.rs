use std::iter::Peekable;
use std::str::Chars;

/**
 * thdc's Tokenizer
 *
 * Given an input &str, a Tokenizer will serve as an Iterator<Token>
 * yielding the Tokens of thdc's little language.
 *
 * Author: <author>
 * ONYEN: <onyen>
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.

 */

/**
 * The tokens types of `thdc` v1 are defined below.
 *
 * We are deriving from Debug so that a Token can be printed using
 * the debug placeholder: "{:?}"
 *
 * We are deriving from PartialEq so that you can use assert_eq! in
 * tests to compare a generated token to an expected token.
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown(char),
    Print,
    FullStack,
    Operator(char),
    Number(f64),
}

/**
 * The internal state of a Tokenizer is maintained by a peekable character
 * iterator over a string. The 'str lifetime parameter indicates a tokenizer
 * cannot outlive the &str it is processing. Note the name 'str is just a name
 * to help make the connection more obvious than 'a. It could also have been
 * assigned a lifetime parameter named 'a or 'foo instead. There's no real
 * relationship between 'str and a &str outside the name hint. A peekable
 * iterator is one that you can call the peek() method on to see what comes
 * next without actually advancing the iterator forward by one.
 */
pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

/**
 * The public facing impl of Tokenizer.
 */
impl<'str> Tokenizer<'str> {
    /**
     * Constructor - The input &str a Tokenizer is constructed with
     * establishes lifetime of the Tokenizer constructed.
     */
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

/**
 * The Iterator trait is implemented for Tokenizer in this impl. This is
 * like implementing an interface in Java. Notice it will produce items of
 * type Token and has a `next` method that returns Option<Token>.
 */
impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    /**
     * The `next` method ignores leading whitespace and returns the next
     * complete Some(Token) in the Tokenizer's input string or None at all.
     */
    fn next(&mut self) -> Option<Token> {
        // Remove this note in your impl:
        // next is _only_ peeking here. The take helper methods are
        // responsible for actually consuming the iterator's chars.
        // If they do not you will infinitely loop because the chars 
        // iterator is not making forward progress.
        if let Some(c) = self.chars.peek() {
            Some(match c {
                'p' => self.take_print(),
                _ => self.take_unknown(),
            })
        } else {
            None
        }
    }
}

/**
 * Unit Tests for the `next` method.
 */
#[cfg(test)]
mod iterator_tests {
    use super::*;

    #[test]
    fn empty() {
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn unknown() {
        let mut tokens = Tokenizer::new("u");
        assert_eq!(tokens.next(), Some(Token::Unknown('u')));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn print() {
        let mut tokens = Tokenizer::new("p");
        assert_eq!(tokens.next(), Some(Token::Print));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn poop() {
        let mut tokens = Tokenizer::new("poop");
        assert_eq!(tokens.next(), Some(Token::Print));
        assert_eq!(tokens.next(), Some(Token::Unknown('o')));
        assert_eq!(tokens.next(), Some(Token::Unknown('o')));
        assert_eq!(tokens.next(), Some(Token::Print));
        assert_eq!(tokens.next(), None);
    }
}

/*
 * Helper methods of Tokenizer are in the impl block below. None
 * are defined as pub meaning they are only callable from within
 * this module.
 */
impl<'str> Tokenizer<'str> {
    /**
     * The print helper method consumes a 'p' character from the
     * tokenizer's chars iterator and panics if called erroneously.
     * Returns a Print token.
     */
    fn take_print(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        assert_eq!(c, 'p');
        Token::Print
    }

    /**
     * When an unknown character is encountered in the input this
     * function will consume it and return an Unknown token.
     */
    fn take_unknown(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        Token::Unknown(c)
    }
}

/**
 * Unit Tests for helper methods.
 */
#[cfg(test)]
mod helper_method_tests {
    // Import identifiers from containing module
    use super::*;

    // Helper Method Tests
    #[test]
    fn take_print() {
        let mut tokens = Tokenizer::new("p");
        assert_eq!(tokens.take_print(), Token::Print);
        // Ensure we consumed the 'p' character
        assert_eq!(tokens.chars.next(), None);
    }

    #[test]
    fn take_unknown() {
        let mut tokens = Tokenizer::new("u");
        assert_eq!(tokens.take_unknown(), Token::Unknown('u'));
        // Ensure we consumed the unknown character
        assert_eq!(tokens.chars.next(), None);
    }
}
