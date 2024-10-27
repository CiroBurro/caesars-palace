use ansi_term::Colour;
use crate::utils::pulisci_schermo;

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
    println!("\nIl tuo credito attuale è di {} euro", credito);
}