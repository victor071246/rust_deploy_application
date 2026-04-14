use std::process::Command;
use crate::conexao::Conexao;
use crate::scanner::Projeto;
use crate::progress_animation::{self, *};

pub fn executar(conexao: &Conexao, projeto: &Projeto) {
    enviar_arquivos(conexao, projeto);
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