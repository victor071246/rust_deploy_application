use dialoguer::{Select, Input};

pub struct Conexao {
    pub usuario: String,
    pub ip: String,
    pub chave: Option<String>,
    pub porta: u16
}

pub fn pedir_conexao() -> Conexao {
    println!("\n[-] Módulo de conexão\n");

    let opcoes_conexao = vec!["Passar string de conexão", "Informar usuário/ip/chave manualmente"];
    let tipo = Select::new()
        .with_prompt("Tipo de conexão SSH")
        .items(&opcoes_conexao)
        .default(0)
        .interact()
        .unwrap();

    match tipo{
        0 => salvar_string_conexao(),
        1 => todo!("modo manual em breve"),
        _ => unreachable!()
    }
}

pub fn salvar_string_conexao() -> Conexao {
    let linha: String = Input::new()
        .with_prompt("Cole a linha de conexão:\n
        Exemplo: ssh -i /home/v/chaves_vps/vps_barbearia/chave_ed25519 v@104.198.38.100")
        .interact_text()
        .unwrap();

    parsear_string_ssh(&linha)
}

fn parsear_string_ssh(linha: &str) -> Conexao {
    let partes: Vec<&str> = linha.split_whitespace().collect();

    let mut chave = None;
    let mut usuario_ip = String::new();
    let mut porta = 22u16;

    let mut i = 0;
    while i < partes.len() {
        match partes[i] {
            "-i" => { chave = Some(partes[i+1].to_string()); i += 2;}
            "-p" => { porta = partes[i+1].parse().unwrap_or(22); i += 2;}
            p if p.contains('@') => { usuario_ip = p.to_string(); i += 1; }
            _ => {i += 1;}
        }
    }

    let partes_usuario_ip: Vec<&str> = usuario_ip.split('@').collect();

    Conexao{
        usuario: partes_usuario_ip[0].to_string(),
        ip: partes_usuario_ip[1].to_string(),
        chave,
        porta
    }

}