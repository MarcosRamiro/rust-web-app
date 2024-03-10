mod database;
mod exemplo_erro;
mod pessoa;

use database::consultar;
use exemplo_erro::*;
use pessoa::{MaiorDeIdade, Pessoa};
use std::{
    fs::File,
    io::{self, Read},
    ops::Deref,
};

fn read_name_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let file_result = File::open("/home/marcos/texto.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    file.read_to_string(&mut s)?;

    Ok(s.trim_end().to_string())
}

fn print_divisao(resultado: Option<i32>) {
    match resultado {
        Some(value) => println!("O resultado da divisao é: {}", value),
        None => println!("Deu ruim: Nao tem retorno."),
    };
}

fn is_maior_de_idade(obj: &dyn MaiorDeIdade) -> bool {
    obj.is_maior_de_idade()
}

mod jogo;

use jogo::jogar;

// cargo watch -x run
fn main() {
    jogar()

    /*
    let pessoa1 = Pessoa::default();
    let pessoa2 = Pessoa::default();
    let pessoa3 = Pessoa::default();
    let mut pessoas: Vec<&dyn MaiorDeIdade> = Vec::new();
    pessoas.push(&pessoa1);
    pessoas.push(&pessoa2);
    pessoas.push(&pessoa3);

    // println!("{:?}", pessoas);
    println!("Total de pessoas na lista = {}", pessoas.len());
    println!("Primeira pessoa é maior de 18 anos = {}", is_maior_de_idade(pessoas[0]));

    for pessoa in pessoas {
        println!("Pessoa = {}", pessoa.to_string());
    }

    let nome = read_name_from_file().unwrap();
    println!("Nome é: {}", nome);

    /*
    let teste = calc(1000, 50)?;
    println!("1 a divisao é: {}", teste);
    let teste = calc(1000, 0)?;
    println!("2 a divisao é: {}", teste);
    */

    let resultado = divide(10, 0);
    match resultado {
        Ok(value) => println!("O resultado é:: {}", value),
        Err(err) => println!("ERRO: {}", err),
    }

    // esse aqui não vai dar erro pois
    println!("a divisao é: {}", divide(100, 5)?);
    let resultado_divisao = divide_option(120, 20);
    print_divisao(resultado_divisao);
    print_divisao(divide_option(12, 0));
    println!("resultado {:?}", resultado_divisao);
    let contador = consultar();
    println!("Executado com sucesso: {}", contador );

    println!("Hello, world!");

    let nova_pessoa = {
        let idade: i32 = 23;
        let nome: String = String::from("Maria");
        let sobrenome = Some("Silva".to_string());
        println!("idade: {}", idade);
        println!("nome: {}", nome);
        println!("sobrenome: {:?}", sobrenome);

        let nova_pessoa = Pessoa::new(nome, idade, sobrenome);

        nova_pessoa
    };

    println!("Nova pessoa: {:?}", nova_pessoa);
    println!("Nova pessoa nome: {}", nova_pessoa.get_nome());
    println!("Nova pessoa nome completo: {}", nova_pessoa.get_nome_completo());
    println!("Nova pessoa idade: {}", nova_pessoa.get_idade());
    println!("Nova pessoa maior de idade: {}", nova_pessoa.is_maior_de_idade());
    println!("Nova pessoa: {:?}", nova_pessoa);

    let pessoa = Pessoa::default();
    println!("Pessoa: {:?}", pessoa);
    println!("Maior de idade: {:?}", is_maior_de_idade(&pessoa));
    println!("Endereço: {:?}", pessoa.get_endereco());
    */
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn test_read_name_from_file() {
        let expected_result: Result<String, _> =
            Ok::<String, Box<dyn Error>>("Marcos Ramiro dos Santos".to_string());
        let result = read_name_from_file();
        assert_eq!(result.unwrap(), expected_result.unwrap());
    }
}
