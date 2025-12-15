pub struct Engine {
    stack: Vec<f64>,
}

impl Engine {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn eval_line(&mut self, input: &str) -> Result<Option<f64>, String> {
        let tokens = tokenize(input)?;

        for token in tokens {
            match token {
                Token::Number(n) => self.stack.push(n),
                Token::Add => {
                    let (a, b) = self.pop2()?;
                    self.stack.push(a + b);
                }
                Token::Subtract => {
                    let (a, b) = self.pop2()?;
                    self.stack.push(a - b);
                }
                Token::Multiply => {
                    let (a, b) = self.pop2()?;
                    self.stack.push(a * b);
                }
                Token::Divide => {
                    let (a, b) = self.pop2()?;
                    self.stack.push(a / b);
                }
            }
        }

        Ok(self.stack.last().copied())
    }

    pub fn stack(&self) -> &[f64] {
        &self.stack
    }

    fn pop1(&mut self) -> Result<f64, String> {
        self.stack.pop().ok_or("Something happened.".to_string())
    }

    fn pop2(&mut self) -> Result<(f64, f64), String> {
        let b = self.pop1()?;
        let a = self.pop1()?;
        Ok((a, b))
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    for part in input.split_whitespace() {
        let token = match part {
            "+" => Token::Add,
            "-" => Token::Subtract,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            number => match number.parse() {
                Ok(n) => Token::Number(n),
                Err(_) => return Err("Something happened".to_string()),
            },
        };
        tokens.push(token);
    }
    Ok(tokens)
}

pub enum Token {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
