use ansi_term::Colour;
use crate::utils::pulisci_schermo;

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
    println!("{}", Colour::Yellow.paint("Scommetti anche tu sullo sport equestre più prestigioso del mondo!"));
    println!("\nIl tuo credito attuale è di {} euro", credito);
}