pub mod conexao;
pub mod scanner;
pub mod docker;
pub mod binario;
pub mod progress_animation;
pub mod deploy;
pub mod health;
use dialoguer::Select;

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


}