use std::str::FromStr;
use std::collections::HashMap;

type Stack<T> = Vec<T>;

#[derive(Default)]
pub struct Interpreter<'a, 'b, T: 'b> {
    data_stack: Stack<T>,
    vocabulary: HashMap<&'a str, &'b Word<T>>,
}

impl<'a, 'b, T: 'b> Interpreter<'a, 'b, T> where T: FromStr {

    fn push(&mut self, value: T) {
        self.data_stack.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.data_stack.pop()
    }

    fn eval_token(&mut self, token: &str) {
        if let Ok(literal) = token.parse::<T>() {
            self.push(literal);
        } else {
            if let Some(&word) = self.vocabulary.get(token) {
                word.eval_within(self);
            }
        }
    }

    fn define_word(&mut self, name: &'a str, word: &'b Word<T>) {
        self.vocabulary.insert(name, word);
    }

    fn lookup(&self, token: &str) -> Option<&&Word<T>> {
        self.vocabulary.get(token)
    }
}

struct Word<T> {
    entry: Box<Fn(&mut Interpreter<T>) -> ()>,
}

impl<T> Word<T> {
    fn eval_within(&self, mut i: &mut Interpreter<T>) {
      (self.entry)(&mut i)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Word;

    fn fixture_plus() -> Word<i32> {
        Word {
            entry: Box::new(|ref mut i| {
                if let (Some(b), Some(a)) = (i.pop(), i.pop()) {
                    i.push(a + b)
                } else { panic!("Stack underflow") }
            })
        }
    }

    #[test]
    fn register_word() {
        let plus = fixture_plus();
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.define_word("+", &plus);
        interpreter.push(51);
        interpreter.push(42);
        interpreter.eval_token("+");
        assert_eq!(Some(93), interpreter.pop())
    }

    #[test]
    fn plus_word() {
        let mut interpreter = Interpreter::<i32>::default();
        let plus = fixture_plus();
        interpreter.push(42);
        interpreter.push(51);
        plus.eval_within(&mut interpreter);
        assert_eq!(Some(93), interpreter.pop())
    }

    #[test]
    fn push_literal() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.eval_token("42");
        assert_eq!(Some(42), interpreter.pop());
    }

    #[test]
    fn pops_nothing() {
        let mut interpreter = Interpreter::<i32>::default();
        assert_eq!(None, interpreter.pop())
    }

    #[test]
    fn pops_as_pushed() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.push(42);
        assert_eq!(Some(42), interpreter.pop());
        assert_eq!(None, interpreter.pop())
    }

    #[test]
    fn pops_in_reverse_order() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.push(42);
        interpreter.push(51);
        assert_eq!(Some(51), interpreter.pop());
        assert_eq!(Some(42), interpreter.pop());
        assert_eq!(None, interpreter.pop())
    }
}
