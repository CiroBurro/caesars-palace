use ansi_term::Colour;
use crate::utils::{pulisci_schermo, Cavallo, genera_cavalli};
use crate::atrio::atrio;
use std::io;
use std::thread;
use std::time::Duration;

pub fn run(credito: i32) {
    pulisci_schermo();
    let titolo = r"_________                                   .___     .__                             .__  .__  .__ 
\_   ___ \  ___________  ___________      __| _/____ |__|   ____ _____ ___  _______  |  | |  | |__|
/    \  \/ /  _ \_  __ \/  ___/\__  \    / __ |/ __ \|  | _/ ___\\__  \\  \/ /\__  \ |  | |  | |  |
\     \___(  <_> )  | \/\___ \  / __ \_ / /_/ \  ___/|  | \  \___ / __ \\   /  / __ \|  |_|  |_|  |
 \______  /\____/|__|  /____  >(____  / \____ |\___  >__|  \___  >____  /\_/  (____  /____/____/__|
        \/                  \/      \/       \/    \/          \/     \/           \/              ";
    
    println!("{} \n", Colour::Red.paint(titolo));
    println!("{}", Colour::Red.paint("Caesars Palace Casino - Corsa dei cavalli\n"));
    println!("{}", Colour::Yellow.paint("Scommetta anche lei sullo sport equestre più prestigioso del mondo!"));

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
                println!("\n{}", Colour::Green.paint("Un'altra gara è in arrivo, vuole continuare a giocare? (s/n)"));
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
    
    
    fn gioca(cred:i32) -> i32{
        pulisci_schermo();
        let mut credito = cred;
        let cavalli = genera_cavalli();

        loop {
            println!("\n{}", Colour::Green.paint(format!("Il suo credito attuale è di {} euro, quanto vuole scommettere? €", credito)));
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Errore di lettura");
            let scommessa = input.trim().parse::<i32>().unwrap();
            
            match Some(scommessa) {
                Some(x) => {
                    if x > credito{
                        println!("La sua scommessa è troppo alta. Riprovi.");
                    }else {
                        credito -= scommessa;
                        println!("{}", Colour::Green.paint("Oggi al nostro ippodromo gareggeranno i seguenti cavalli:\n"));
                        for (i, cavallo) in cavalli.iter().enumerate() {
                            println!("{}. Nome: {}, Colore: {}", i+1, cavallo.nome, cavallo.colore);
                        }
                        println!("{}", Colour::Green.paint("\nQuale preferisce?"));
                        let mut scelta = String::new();
                        io::stdin().read_line(&mut scelta).expect("Inserisca una scelta valida");
                        match scelta.trim().to_lowercase().as_str() {
                            "1" => {
                                let cavallo_scelto = cavalli[0].clone();
                                let vincita = vincita(cavallo_scelto, cavalli, x);
                                credito += vincita;
                                return credito;
                            }
                            "2" => {
                                let cavallo_scelto = cavalli[1].clone();
                                let vincita = vincita(cavallo_scelto, cavalli, x);
                                credito += vincita;
                                return credito;
                            }
                            "3" => {
                                let cavallo_scelto = cavalli[2].clone();
                                let vincita = vincita(cavallo_scelto, cavalli, x);
                                credito += vincita;
                                return credito;
                            }
                            "4" => {
                                let cavallo_scelto = cavalli[3].clone();
                                let vincita = vincita(cavallo_scelto, cavalli, x);
                                credito += vincita;
                                return credito;
                            }
                            "5" => {
                                let cavallo_scelto = cavalli[4].clone();
                                let vincita = vincita(cavallo_scelto, cavalli, x);
                                credito += vincita;
                                return credito;
                            }
                            _ => ()
                        }
                    }
                }
                None => {
                    println!("Inserisca un numero valido.");
                }
            }
        }

        
            
    }
}

fn vincita(cavallo_scelto:Cavallo, cavalli: Vec<Cavallo>, scommessa:i32) ->i32{
    let mut velocità = Vec::new();
    let time = Duration::new(1, 5);
    let _pista_lunghezza = 50;
    let _frame_delay = Duration::from_millis(200);

    for cavallo in cavalli.iter() {
        velocità.push(cavallo.velocità);
    }

    animazione(cavalli);
    
    if let Some(max) = velocità.iter().max() {
        if &cavallo_scelto.velocità == max {
            println!("\n{} ha vinto, complimenti!", cavallo_scelto.nome);
            thread::sleep(time);
            scommessa*2
        }else {
            println!("\nMi dispiace, hai perso!");
            thread::sleep(time);
            0
        }
    }else {
        println!("Errore nel calcolo della velocità dei cavalli");
        0
    }
}

fn animazione(cav:Vec<Cavallo>) {
    let time = Duration::new(1, 0);
    let pista_lunghezza = 80; // Lunghezza della pista
    let frame_delay = Duration::from_millis(200); // Velocità dell'animazione
    let mut cavalli = cav;

    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("Che la gara abbia inizio!"));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("3"));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("2"));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("1"));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("0!\n"));

    loop {
        for cavallo in &mut cavalli {
            cavallo.posizione += cavallo.velocità as usize / 2;
            if cavallo.posizione > pista_lunghezza {
                cavallo.posizione = pista_lunghezza;
            }
        }

        print!("\x1B[2J\x1B[1;1H");

        for cavallo in &cavalli {
            // Fissa una larghezza totale per ogni riga
            let pista = format!(
                "{: <10} |{}🏇{}|",
                cavallo.nome,
                ".".repeat(cavallo.posizione),
                " ".repeat(pista_lunghezza - cavallo.posizione)
            );
            println!("{}", pista);
        }

        if let Some(_vincitore) = cavalli.iter().find(|c| c.posizione >= pista_lunghezza) {
            break;
        }

        thread::sleep(frame_delay);
    }

}

