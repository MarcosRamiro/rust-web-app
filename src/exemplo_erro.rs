use std::fmt;

#[derive(Debug)]
pub enum MyOwnError {
    DivideByZeroNotPermited(String),
}

impl fmt::Display for MyOwnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MyOwnError::DivideByZeroNotPermited(ref msg) => {
                write!(f, "Não é permitido fazer divisão por ZERO. {}.", msg)
            }
        }
    }
}

impl std::error::Error for MyOwnError {}

pub fn divide_option(x: i32, y: i32) -> Option<i32> {
    let resultado = divide(x, y);
    let retorno = match resultado {
        Ok(value) => Some(value),
        Err(_) => None,
    };
    retorno
}

pub fn divide(x: i32, y: i32) -> Result<i32, MyOwnError> {
    if y == 0 {
        let mensagem_erro: String = format!("Valor recebido para y: {}", y);
        return Err(MyOwnError::DivideByZeroNotPermited(mensagem_erro));
    }

    Ok(x / y)
}
