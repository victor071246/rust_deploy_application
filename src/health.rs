use std::net::TcpStream;
use std::collections::HashMap;
use std::process::Command;
use serde::Deserialize;
use crate::conexao::Conexao;
use crate::progress_animation;
use crate::scanner::Projeto;


#[derive(Deserialize)]
struct Servico {
    ports: Option<Vec<String>>
}

#[derive(Deserialize)]
struct DockerCompose {
    services: HashMap<String, Servico>
}

pub fn checar(conexao: &Conexao, projeto: &Projeto) {
    let spinner = progress_animation::iniciar_animacao_carregamento("\n[-] Verificando serviços".to_string());

    let compose_path = format!("{}/docker-compose.yml", projeto.pasta);
    let conteudo = std::fs::read_to_string(&compose_path).unwrap();
    let compose: DockerCompose = serde_yaml::from_str(&conteudo).unwrap();

    checar_containers(conexao, &compose);
    testar_portas(conexao, &compose);
    testar_http(conexao);

    progress_animation::finalizar_animacao_carregamento(spinner, "\n[-] Arquivos verificados".to_string());
}

fn checar_containers(conexao: &Conexao, compose: &DockerCompose) {
    println!("\n[-] Verificando containers");
    

    let chave_arg = match &conexao.chave {
        Some(chave) => format!("-i {}", chave),
        None => String::new(),
    };

    for (nome, _) in &compose.services {
        let cmd = format!(
            "docker ps --filter name={} --filter status=running --format '{{{{.Names}}}}'",
            nome
        );

        let output = Command::new("ssh")
            .arg(format!("-p {}", conexao.porta))
            .args(chave_arg.split_whitespace())
            .arg(format!("{}@{}", conexao.usuario, conexao.ip))
            .arg(&cmd)
            .output()
            .unwrap();

        let resultado = String::from_utf8_lossy(&output.stdout);

        if resultado.trim().is_empty() {
            println!(" ● {}: ✗", nome);
        } else {
            println!(" ● {}: ✓", nome)
        }
    }    
}

fn testar_portas(conexao: &Conexao, compose: &DockerCompose) {
    println!("\n[-] Portas");

    for(nome, servico) in &compose.services{
        if let Some(ports) = &servico.ports {
            for port in ports {
                let porta_host = port.split(':').next().unwrap();
                let addr = format!("{}:{}", conexao.ip, porta_host);

                match TcpStream::connect(&addr) {
                    Ok(_) => println!(" ● {} ({}): ✓", nome, porta_host),
                    Err(_) => println!("● {} ({}):  ✗", nome, porta_host)
                }
            }
        }
    }
}

fn testar_http(conexao: &Conexao) {
    println!("\n[-] HTTP GET http://{}/", conexao.ip);

    let output = Command::new("curl")
        .arg("-s")
        .arg("--max-time")
        .arg("5")
        .arg(format!("https://{}/", conexao.ip))
        .output()
        .unwrap();

    let body = String::from_utf8_lossy(&output.stdout);

    if body.contains("Welcome to nginx") {
        println!("  ● nginx default page")
    } else if body.contains("404") && body.contains("nginx") {
        println!("  ● 404 nginx");
    } else if !body.is_empty() {
        println!("  ● aplicação respondendo");
    } else {
        println!("  ● sem resposta");
    }
    
}