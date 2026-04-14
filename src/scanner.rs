use walkdir::WalkDir;
use dialoguer::Input;
use crate::progress_animation;

pub struct Projeto {
    pub pasta: String,
    pub possui_docker_compose: bool,
    pub dockerfiles: Vec<String>,
}

pub fn escanear() -> Projeto {
    let pasta: String = Input::new()
        .with_prompt("\n[-]Insira o caminho da pasta do projeto")
        .interact_text()
        .unwrap();

    let spinner = progress_animation::iniciar_animacao_carregamento("Buscando arquivos".to_string());
    
    let mut dockerfiles = Vec::new();
    let mut possui_docker_compose = false;

    for entry in WalkDir::new(&pasta).max_depth(2) {
        let entry = entry.unwrap();
        let nome = entry.file_name().to_string_lossy();

        if nome == "docker-compose.yml" {
            possui_docker_compose = true;
        } else if nome == "Dockerfile" {
            dockerfiles.push(entry.path().to_string_lossy().to_string());
        }
    }

    progress_animation::finalizar_animacao_carregamento(spinner, "\n[-]Projeto escaneado".to_string());
    Projeto {pasta, possui_docker_compose, dockerfiles}
}

pub fn exibir_projeto(projeto: &Projeto) {
    println!("\n[-]Projeto escaneado");
    println!("  ● Pasta:              {}", projeto.pasta);
    println!("  ● arquivo docker-compose: {}", if projeto.possui_docker_compose {"encontrado"} else {"não encontrado"});

    println!("\n[-] Arquivos docker encontrados");
    for dockerfile in &projeto.dockerfiles {
        println!("  ● {}", dockerfile)
    }
}