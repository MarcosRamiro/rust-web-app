use rusqlite::Connection;
use uuid::Uuid;

const PATH_ARQUIVO_DB: &str = "/home/marcos/test.db";
const MSG_ERRO_ABRIR_ARQUIVO_DB: &str = "Erro ao tentar abrir o arquivo DB /home/marcos/test.db";

#[derive(Debug)]
struct Jogo {
    id: Uuid,
    jogador1: String,
    jogador2: String,
    estado_jogo: String,
}

impl Jogo {
    fn new(id: Uuid, jogador1: String, jogador2: String, estado_jogo: String) -> Self {
        Jogo {
            id: id,
            jogador1: jogador1,
            jogador2: jogador2,
            estado_jogo: estado_jogo,
        }
    }
}

pub fn consultar() -> i32 {
    println!("Entrou consultar");

    let conn = Connection::open(PATH_ARQUIVO_DB).expect(MSG_ERRO_ABRIR_ARQUIVO_DB);

    let mut stmt = conn
        .prepare("SELECT id, jogador1, jogador2, estado_jogo FROM jogo")
        .expect("Não foi possível executar a consulta na tabela.");
    let jogo_iter = stmt
        .query_map([], |row| {
            Ok(Jogo::new(
                Uuid::new_v4(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
            ))
        })
        .unwrap();

    let mut contador: i32 = 0;

    for _ in jogo_iter {
        //println!("{:?}", jogo?);
        contador += 1;
    }

    println!("Saiu consultar");

    contador
}
