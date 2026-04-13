pub mod conexao;
pub mod scanner;
pub mod docker;
pub mod binario;
use dialoguer::{Select, Input};

fn main() {
    println!("\n[-] rust_deploy\n");

    let opcoes = vec!["Docker (docker-compose)", "Binário/Applicação"];
    let tipo = Select::new()
        .with_prompt("Selecione um tipo de projeto: ")
        .items(&opcoes)
        .default(0)
        .interact()
        .unwrap();

    match tipo {
        0 => docker::fluxo_docker(),
        1 => todo!("modo binário em breve"),
        _ => unreachable!(),
    }

    // fn fluxo_docker() {
    //     let conexao = pedir_conexao_ssh();
    //     println!("\n ● Conexão: {}", conexao)
    // }

    // fn pedir_conexao_ssh() -> String{
    //     let opcoes = vec![
    //         "Linha de conexão (ex: ssh -i /chave user@ip",
    //         "Manual (usuário, IP, chave, porta)",
    //     ];

    //     let modo = Select::new()
    //         .with_prompt("Modo de conexão SSH")
    //         .items(&opcoes)
    //         .default(0)
    //         .interact()
    //         .unwrap();

    //     match modo {
    //         0 => {
    //             Input::new()
    //                 .with_prompt("Cole a linha de conexão")
    //                 .interact_text()
    //                 .unwrap()
    //         }
    //         1 => todo!("modo manual em breve"),
    //         _ => unreachable!()
    //     }
    // }
}