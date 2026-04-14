use std::process::Command;
use crate::conexao::Conexao;
use crate::scanner::Projeto;
use crate::progress_animation::{self, *};

pub fn executar(conexao: &Conexao, projeto: &Projeto) {
    enviar_arquivos(conexao, projeto);
    subir_containers(conexao, projeto);
}

fn enviar_arquivos(conexao: &Conexao, projeto: &Projeto) {
    //rsync -avz -e "ssh -i /caminho/chave -p 22" /pasta/local user@ip:/pasta/remoto

    let spinner = progress_animation::iniciar_animacao_carregamento("\n Enviando arquivos".to_string());

    let ssh_cmd = match &conexao.chave {
        Some(chave) => format!("ssh -i {} -p {}", chave, conexao.porta),
        None => format!("ssh -p {}", conexao.porta)
    };

    let destino = format!("{}@{}:/app/", conexao.usuario, conexao.ip);

    let status = Command::new("rsync")
        .arg("-avz")
        .arg("-e")
        .arg(&ssh_cmd)
        .arg(&projeto.pasta)
        .arg(&destino)
        .status()
        .unwrap();

    if status.success() {
        progress_animation::finalizar_animacao_carregamento(spinner, "\n ● Arquivos enviados com sucesso".to_string());
    }
    else {
        println!("\n ● Erro ao enviar arquivos");
        std::process::exit(1);
    }
}

fn subir_containers(conexao: &Conexao, projeto: &Projeto) {

    let spinner = progress_animation::iniciar_animacao_carregamento("[-] Subindo containers docker".to_string());


    let chave_arg = match &conexao.chave {
        Some(chave) => format!("-i {}", chave),
        None => String::new(),
    };

    let cmd = format!(
        "cd /app/{} && docker-compose up -d --build",
        std::path::Path::new(&projeto.pasta)
            .file_name()
            .unwrap()
        .to_string_lossy()
    );

    let status = Command::new("ssh")
        .arg(format!("-p {}", conexao.porta))
        .args(chave_arg.split_whitespace())
        .arg(format!("{}@{}", conexao.usuario, conexao.ip))
        .arg(&cmd)
        .status()
        .unwrap();

    if status.success() {
        progress_animation::finalizar_animacao_carregamento(spinner, "  ● Containers subidos".to_string());
    } else {
        println!("  ● Erro ao subir containers");
        std::process::exit(1);
    }
}