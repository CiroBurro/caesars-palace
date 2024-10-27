use ansi_term::Colour;
use crate::utils::pulisci_schermo;

pub fn run(credito: i32) {
    pulisci_schermo();
    let titolo = r"__________.__                 __         __               __    
\______   \  | _____    ____ |  | __    |__|____    ____ |  | __
 |    |  _/  | \__  \ _/ ___\|  |/ /    |  \__  \ _/ ___\|  |/ /
 |    |   \  |__/ __ \\  \___|    <     |  |/ __ \\  \___|    < 
 |______  /____(____  /\___  >__|_ \/\__|  (____  /\___  >__|_ \
        \/          \/     \/     \/\______|    \/     \/     \/";
    
    println!("{} \n", Colour::Red.paint(titolo));
    println!("{}", Colour::Red.paint("Caesars Palace Casino - Blackjack\n"));
    println!("{}", Colour::Yellow.paint("Blackjack? Ottima scelta, d'altronde è il gioco di carte più popolare nei casinò americani!"));
    println!("\nIl tuo credito attuale è di {} euro", credito);
}