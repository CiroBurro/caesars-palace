use ansi_term::Colour;
use std::io;
use crate::utils::{esci, pulisci_schermo};

pub fn atrio(credito: i32) {
    pulisci_schermo();
    let titolo = r"   _____   __         .__        
  /  _  \_/  |________|__| ____  
 /  /_\  \   __\_  __ \  |/  _ \ 
/    |    \  |  |  | \/  (  <_> )
\____|__  /__|  |__|  |__|\____/ 
        \/                       ";

    println!("{} \n", Colour::Red.paint(titolo));
    println!("{}", Colour::Red.paint("Caesars Palace Casino - Atrio\n"));
    println!("{}", Colour::Yellow.paint("In questo momento si trova nell'atrio. Da qui può spostarsi in tutte le aree del casinò, oppure decidere di uscire."));
    
    println!("\n{}", Colour::Green.paint(format!("Il suo credito attuale è di {} euro", credito)));

    prima_scelta(credito);
}

fn prima_scelta(credito: i32) {
    println!("\nCosa vuole fare?\n1. Giochiamo!\n2. Per questa volta esco\n");

    let mut scelta = String::new();
    io::stdin().read_line(&mut scelta).expect("Errore di lettura");

    match scelta.trim() {
        "1" => scegli_gioco(credito),
        "2" => esci(),
        _ => prima_scelta(credito),
    }
}

fn scegli_gioco(credito: i32) {
    println!("\nScelga il gioco a cui vuole giocare:\n1. Roulette\n2. Blackjack\n3. Corsa dei cavalli\n4. Lancio dei dadi\n5. Torna indietro\n");
    let mut scelta = String::new();
    io::stdin().read_line(&mut scelta).expect("Errore di lettura");

    match scelta.trim() {
        "1" => crate::giochi::roulette::run(credito),
        "2" => crate::giochi::blackjack::run(credito),
        "3" => crate::giochi::cavalli::run(credito),
        "4" => crate::giochi::dadi::run(credito),
        "5" => prima_scelta(credito),
        _ => scegli_gioco(credito),
    }
}

