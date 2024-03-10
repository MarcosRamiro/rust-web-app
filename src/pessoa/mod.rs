use std::fmt::format;

#[derive(Debug)]
pub struct Pessoa {
    nome: String,
    sobrenome: Option<String>,
    idade: i32,
    endereco: Option<Endereco>,
}

#[derive(Debug)]
struct Endereco {
    rua: String,
    numero: String,
    complemento: Option<String>,
}

impl Pessoa {
    pub fn new(nome: String, idade: i32, sobrenome: Option<String>) -> Self {
        Self {
            nome: nome,
            idade: idade,
            sobrenome: sobrenome,
            endereco: None,
        }
    }

    pub fn get_nome(&self) -> &str {
        self.nome.as_ref()
    }

    pub fn get_idade(&self) -> i32 {
        self.idade
    }

    pub fn get_endereco(&self) -> String {
        let retorno: String = match &self.endereco {
            Some(end) => match &end.complemento {
                Some(comp) => format!("{} {} {}", end.rua, end.numero, comp),
                None => format!("{} {}", end.rua, end.numero),
            },
            None => "".into(),
        };
        retorno
    }

    pub fn get_nome_completo(&self) -> String {
        format!("{} {}", self.get_nome(), self.get_sobrenome())
    }

    fn get_sobrenome(&self) -> &str {
        match self.sobrenome.as_ref() {
            Some(value) => value,
            _ => "",
        }
    }
}

pub trait MaiorDeIdade {
    fn is_maior_de_idade(&self) -> bool;
    fn to_string(&self) -> String;
}

impl MaiorDeIdade for Pessoa {
    fn is_maior_de_idade(&self) -> bool {
        self.idade >= 18
    }
    fn to_string(&self) -> String {
        format!(
            "Pessoa (nome: {}, idade: {}, is_maior_de_idade: {}, endereco: {:?})",
            self.get_nome(),
            self.get_idade(),
            self.is_maior_de_idade(),
            self.get_endereco()
        )
    }
}

impl Default for Endereco {
    fn default() -> Self {
        Self {
            rua: "Rua Salvador".into(),
            numero: "45 B".into(),
            complemento: Option::None,
        }
    }
}

impl Default for Pessoa {
    fn default() -> Self {
        Self {
            nome: "Marcos".into(),
            idade: 34,
            sobrenome: Option::Some("Ramiro".into()),
            endereco: Option::Some(Endereco::default()),
            //endereco: None,
        }
    }
}
