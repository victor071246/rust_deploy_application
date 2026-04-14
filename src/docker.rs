use dialoguer::{Input, Select};
use crate::conexao;
use crate::scanner;
use crate::deploy;
use crate::health;

pub fn fluxo_docker() {
    let conexao = conexao::pedir_conexao();

    println!("\n[-] Conexão");
    println!(" ● Usuário: {}", conexao.usuario);
    println!(" ● IP:      {}", conexao.ip);
    println!(" ● Porta:   {}", conexao.porta);

    let projeto = scanner::escanear();
    scanner::exibir_projeto(&projeto);
    deploy::executar(&conexao, &projeto);
    health::checar(&conexao, &projeto);
    
}