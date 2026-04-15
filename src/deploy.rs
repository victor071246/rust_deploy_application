use std::process::{Command, Stdio};
use crate::conexao::{self, Conexao};
use crate::scanner::Projeto;
use crate::progress_animation::{self};

pub fn executar(conexao: &Conexao, projeto: &Projeto) {
    garantir_permissao_chave(conexao);
    instalar_docker(conexao);
    instalar_docker_compose(conexao);
    instalar_rsyc(conexao);
    criar_pasta_app(conexao);
    enviar_arquivos(conexao, projeto);
    subir_containers(conexao, projeto);
}

fn instalar_rsyc(conexao: &Conexao) {
    let spinner = progress_animation::iniciar_animacao_carregamento("\n[-] Instalando rsync para transferir arquivos".to_string());



    let mut comando_ssh = Command::new("ssh");
    if let Some(chave) = &conexao.chave {
        comando_ssh.arg("-i").arg(chave);
    }

    let status = comando_ssh
        .arg("-p").arg(&conexao.porta.to_string())
        .arg(format!("{}@{}", conexao.usuario, conexao.ip))
        .arg("which rsync || sudo apt install -y rsync")
        .status()
        .unwrap();

    if !status.success() {
        println!("  ● Erro ao instalar rsync");
        std::process::exit(1);
    }

    

    progress_animation::finalizar_animacao_carregamento(spinner, "rsync instalado com sucesso".to_string());
}

fn enviar_arquivos(conexao: &Conexao, projeto: &Projeto) {
    //rsync -avz -e "ssh -i /caminho/chave -p 22" /pasta/local user@ip:/pasta/remoto

    let spinner = progress_animation::iniciar_animacao_carregamento("\n Enviando arquivos".to_string());

    let ssh_cmd = match &conexao.chave {
        Some(chave) => format!("ssh -i {} -p {}", chave, conexao.porta),
        None => format!("ssh -p {}", conexao.porta)
    };

    let destino = format!("{}@{}:/app/", conexao.usuario, conexao.ip);

    let mut child = Command::new("rsync")
        .arg("-avz")
        .arg("-e")
        .arg(&ssh_cmd)
        .arg("--exclude=target")
        .arg("--exclude=node_modules")
        .arg("--exclude=.git")
        .arg("--exclude=dist")
        .arg("--exclude=build")
        .arg("--exclude=.next")
        .arg("--exclude=.nuxt")
        .arg("--exclude=.output")
        .arg("--exclude=.env")
        .arg("--exclude=.env.local")
        .arg("--exclude=.env.production")
        .arg("--exclude=.env.development")
        .arg("--exclude=__pycache__")
        .arg("--exclude=.pytest_cache")
        .arg("--exclude=.venv")
        .arg("--exclude=venv")
        .arg("--exclude=.idea")
        .arg("--exclude=.vscode")
        .arg("--exclude=*.log")
        .arg("--exclude=*.tmp")
        .arg("--exclude=.DS_Store")
        .arg("--exclude=thumbs.db")
        .arg("--exclude=coverage")
        .arg("--exclude=.nyc_output")
        .arg("--exclude=vendor")
        .arg("--exclude=.cargo")
        .arg(&projeto.pasta)
        .arg(&destino)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = child.wait().unwrap();

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
        "sudo systemctl start docker && cd /app/{} && docker-compose up -d --build",
        std::path::Path::new(&projeto.pasta)
            .file_name()
            .unwrap()
        .to_string_lossy()
    );

    let status = Command::new("ssh")
        .arg(format!("-p {}", conexao.porta))
        .args(chave_arg.split_whitespace())
        .arg("-o").arg("ServerAliveInterval=60")
        .arg("-o").arg("ServerAliveCountMax=10")
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

fn garantir_permissao_chave(conexao: &Conexao) {
    if let Some(chave) = &conexao.chave {
        Command::new("chmod")
            .arg("600")
            .arg(chave)
            .status()
            .unwrap();
    }
}

fn criar_pasta_app(conexao: &Conexao) {

    println!("\n[-] Criando pasta /app");
    let mut criar_pasta = Command::new("ssh");

    if let Some(chave) = &conexao.chave {
        criar_pasta.arg("-i").arg(chave);
    }

    criar_pasta
        .arg("-p").arg(&conexao.porta.to_string())
        .arg(format!("{}@{}", conexao.usuario, conexao.ip))
        .arg("sudo mkdir -p /app && sudo chown -R $USER:$USER /app")
        .status()
        .unwrap();
}

fn instalar_docker_compose(conexao: &Conexao) {
    println!("\n[-] Instalando docker compose caso não exista");
    let mut cmd = Command::new("ssh");
    if let Some(chave) = &conexao.chave {
        cmd.arg("-i").arg(chave);
    }
    cmd.arg("-p").arg(&conexao.porta.to_string())
    .arg(format!("{}@{}", conexao.usuario, conexao.ip))
    .arg("which docker-compose || (curl -fsSL https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m) -o /tmp/docker-compose && sudo mv /tmp/docker-compose /usr/local/bin/docker-compose && sudo chmod +x /usr/local/bin/docker-compose)")
    .status()
    .unwrap();
}

fn instalar_docker(conexao: &Conexao) {
    println!("\n[-] Instalando docker caso não exista");
    let mut cmd = Command::new("ssh");
    if let Some(chave) = &conexao.chave {
        cmd.arg("-i").arg(chave);
    }
    cmd.arg("-p").arg(&conexao.porta.to_string())
        .arg(format!("{}@{}", conexao.usuario, conexao.ip))
        .arg("which docker || (curl -fsSL https://get.docker.com | sudo sh)")
        .status()
        .unwrap();
}