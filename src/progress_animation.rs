use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn iniciar_animacao_carregamento(mensagem: String) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    spinner.set_message(mensagem);
    spinner.enable_steady_tick(Duration::from_millis(30));
    spinner
}

pub fn finalizar_animacao_carregamento(spinner: ProgressBar, mensagem: String) {
    spinner.finish_with_message(mensagem);
}