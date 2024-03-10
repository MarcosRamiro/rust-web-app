use std::io::stdin;

const MSG_JOGADA_DUPLICADA: &str = "Jogada duplicada.";

#[derive(Debug)]
struct Jogador {
    nome: String,
}

impl PartialEq for Jogador {
    fn eq(&self, other: &Self) -> bool {
        self.nome.eq(&other.nome)
    }
}

impl Jogador {
    fn maria() -> Self {
        Self {
            nome: "Maria".to_string(),
        }
    }
    fn joao() -> Self {
        Self {
            nome: "Joao".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Jogada<'a> {
    posicao: i32,
    jogador: &'a Jogador,
}

impl<'a> Jogada<'a> {
    fn new(posicao: i32, jogador: &'a Jogador) -> Self {
        Self {
            posicao: posicao,
            jogador: jogador,
        }
    }

    fn get_posicao(&self) -> i32 {
        self.posicao
    }
}

impl<'a> PartialEq for Jogada<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.posicao == other.posicao && self.jogador == other.jogador
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum EstadoJogo<'a> {
    Aberto,
    Empate,
    Finalizado(&'a Jogador),
}

struct Jogo<'a> {
    estado_jogo: EstadoJogo<'a>,
    jogadas: Vec<Jogada<'a>>,
}

trait FazerJogada<'a> {
    fn jogar(&mut self, posicao: i32, jogador: &'a Jogador) -> Result<EstadoJogo<'a>, String>;
}

fn is_jogada_duplicada(jogadas: &Vec<Jogada<'_>>, jogada: &Jogada<'_>) -> bool {
    jogadas
        .iter()
        .any(|jogada_| jogada_.get_posicao() == jogada.get_posicao())
}

impl<'a> FazerJogada<'a> for Jogo<'a> {
    fn jogar(&mut self, posicao: i32, jogador: &'a Jogador) -> Result<EstadoJogo<'a>, String> {
        if self.estado_jogo != EstadoJogo::Aberto {
            return Err("Estado inválido.".to_string());
        }

        let jogada = Jogada::new(posicao, jogador);

        if is_jogada_duplicada(&self.jogadas, &jogada) {
            return Err(MSG_JOGADA_DUPLICADA.to_string());
        }

        self.jogadas.push(jogada);

        for (pos1, pos2, pos3) in get_jogadas_vencedoras(jogador) {
            if self.jogadas.contains(&pos1)
                && self.jogadas.contains(&pos2)
                && self.jogadas.contains(&pos3)
            {
                self.estado_jogo = EstadoJogo::Finalizado(jogador);
                return Ok(self.estado_jogo);
            }
        }

        if self.jogadas.len() == 9 {
            self.estado_jogo = EstadoJogo::Empate;
            return Ok(self.estado_jogo);
        }

        Ok(EstadoJogo::Aberto)
    }
}

impl<'a> Jogo<'a> {
    fn new() -> Jogo<'a> {
        Jogo {
            estado_jogo: EstadoJogo::Aberto,
            jogadas: Vec::new(),
        }
    }
}

fn criar_jogada(posicao: i32, jogador: &Jogador) -> Jogada {
    Jogada::new(posicao, jogador)
}

fn construir_tupla(
    n1: i32,
    n2: i32,
    n3: i32,
    jogador: &Jogador,
) -> (Jogada<'_>, Jogada<'_>, Jogada<'_>) {
    (
        criar_jogada(n1, jogador),
        criar_jogada(n2, jogador),
        criar_jogada(n3, jogador),
    )
}

fn get_jogadas_vencedoras(jogador: &Jogador) -> Vec<(Jogada, Jogada, Jogada)> {
    let mut retorno = Vec::new();
    retorno.push(construir_tupla(1, 2, 3, jogador));
    retorno.push(construir_tupla(4, 5, 6, jogador));
    retorno.push(construir_tupla(7, 8, 9, jogador));
    retorno.push(construir_tupla(1, 4, 7, jogador));
    retorno.push(construir_tupla(2, 5, 8, jogador));
    retorno.push(construir_tupla(3, 6, 9, jogador));
    retorno.push(construir_tupla(1, 5, 9, jogador));
    retorno.push(construir_tupla(7, 5, 3, jogador));

    retorno
}


fn is_posicao_valida(posicao: i32) -> bool {
    posicao < 1 || posicao > 9
}

pub fn jogar() {
    let maria: Jogador = Jogador::maria();
    let joao: Jogador = Jogador::joao();
    let mut jogador_atual: &Jogador = &maria;
    let mut jogo: Jogo<'_> = Jogo::new();

    let mut buffer = String::new();

    loop {
        buffer.clear();

        println!("Olá {}, informe a sua jogada: ", jogador_atual.nome);

        stdin()
            .read_line(&mut buffer)
            .expect("Não foi possível ler a entrada do usuário.");

        let entrada = buffer.trim();
        let entrada_resultado = entrada.parse();
        let posicao: i32 = match entrada_resultado {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Não foi possível converter '{}' para número inteiro.",
                    entrada
                );
                continue;
            }
        };

        if is_posicao_valida(posicao) {
            println!("Posição inválida. Digite um número entre 1 e 9.");
            continue;
        }

        let resultado = jogo.jogar(posicao, jogador_atual);

        let estado_jogo = match resultado {
            Ok(estado) => estado,
            Err(msg) => {
                if MSG_JOGADA_DUPLICADA == msg {
                    println!("{}", MSG_JOGADA_DUPLICADA);
                    continue;
                }
                panic!("Erro: {}", msg);
            }
        };

        match estado_jogo {
            EstadoJogo::Aberto => {
                jogador_atual = if jogador_atual == &maria {
                    &joao
                } else {
                    &maria
                };
            }
            EstadoJogo::Empate => {
                println!("Empate!");
                break;
            }
            EstadoJogo::Finalizado(vencedor) => {
                println!("Parabéns {}, você venceu!!", vencedor.nome);
                break;
            }
        }
    }
}
