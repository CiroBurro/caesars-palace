use ansi_term::Colour;
use crate::utils::pulisci_schermo;
use crate::atrio::atrio;
use std::{thread, io};
use std::time::Duration;

pub fn run(credito: i32) {
    pulisci_schermo();
    let titolo = r".__                       .__             .___     .__      .___           .___.__ 
|  | _____    ____   ____ |__| ____     __| _/____ |__|   __| _/____     __| _/|__|
|  | \__  \  /    \_/ ___\|  |/  _ \   / __ |/ __ \|  |  / __ |\__  \   / __ | |  |
|  |__/ __ \|   |  \  \___|  (  <_> ) / /_/ \  ___/|  | / /_/ | / __ \_/ /_/ | |  |
|____(____  /___|  /\___  >__|\____/  \____ |\___  >__| \____ |(____  /\____ | |__|
          \/     \/     \/                 \/    \/          \/     \/      \/     ";
    
    println!("{} \n", Colour::Red.paint(titolo));
    println!("{}", Colour::Red.paint("Caesars Palace Casino - Lancio dei dadi\n"));
    println!("{}", Colour::Yellow.paint("Una partita a dadi non fa mai male, è il gioco di fortuna più antico e diffuso della storia!"));

    println!("\n{}", Colour::Green.paint(format!("Il suo credito attuale è di {} euro", credito)));

    // scegliere se giocare entrando nel main o di non farlo tornando all'atrio
    println!("\nVuole iniziare a giocare? (s/n)");
    let mut scelta = String::new();
    io::stdin().read_line(&mut scelta).expect("Errore di lettura");
    match scelta.trim().to_lowercase().as_str() {
        "s" => {
            let time = Duration::new(1, 0);
            println!("\n{}", Colour::Green.paint("Allora via alle danze!"));
            thread::sleep(time);
            main(credito);
        },
        "n" => atrio(credito),
        _ => {
            println!("Scelta non valida.");
            println!("\nVuole iniziare a giocare? (s/n)");
            let mut scelta = String::new();
            io::stdin().read_line(&mut scelta).expect("Errore di lettura");
            if scelta.trim().to_lowercase().as_str() == "s" {
                let time = Duration::new(2, 0);
                println!("\n{}", Colour::Green.paint("Allora via alle danze!"));
                thread::sleep(time);
                main(credito);
            } else {
                atrio(credito);
            }
        },
    }

    fn main(cred: i32) {
        let time = Duration::new(3, 0);
        let mut credito = cred;
        loop {
            credito = gioca(credito);
            if credito <= 0 {
                println!("Ha perso tutto! La accompagniamo all'uscita");
                thread::sleep(time );
                atrio(credito);
            }else {
                println!("\n{}", Colour::Green.paint("Desidera fare un'altra partita? (s/n)"));
                let mut scelta = String::new();
                io::stdin().read_line(&mut scelta).expect("Inserisca una scelta valida: s/n");
                match scelta.trim().to_lowercase().as_str() {
                    "s" => {println!("\nScelta corretta, la grande vincita sta per arrivare!")},
                    "n" => {
                        println!("La ringraziamo per aver giocato, torni all'atrio per giocare a qualcos'altro o per uscire");
                        thread::sleep(time);
                        atrio(credito); 
                    }
                    _ => println!("Scelta non valida."),
                }
            }
        }
    }

    fn gioca(cred:i32) -> i32 {
        cred
    }
}