use dialoguer::{Input, Select};
use crate::conexao;
use crate::scanner;

pub fn fluxo_docker() {
    let conexao = conexao::pedir_conexao();

    println!("\n[-] Conexão");
    println!(" ● Usuário: {}", conexao.usuario);
    println!(" ● IP:      {}", conexao.ip);
    println!(" ● Porta:   {}", conexao.porta);

    let projeto = scanner::escanear();
    scanner::exibir_projeto(&projeto);
}