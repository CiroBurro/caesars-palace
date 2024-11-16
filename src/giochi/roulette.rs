use std::io;
use std::thread;
use std::time::Duration;
use ansi_term::Colour;
use rand::{seq::IteratorRandom, thread_rng};
use crate::utils::pulisci_schermo;
use crate::atrio::atrio;

// struttture necessarie
#[derive(Debug, Clone)]
enum Puntata {
    Numero(i32),
    Colore(String),
    PariDispari(String),
    PrimaMeta(String),
    SecondaMeta(String),
    PrimaRiga(String),
    SecondaRiga(String),
    TerzaRiga(String),
    Terzina([i32; 3]),
}

#[derive(Debug, Clone)]
struct Giocata {
    scommessa: i32,
    puntata: Puntata,
    scelta: String,
}


//funzione principale del gioco
pub fn run(credito: i32) {
    pulisci_schermo();
    let titolo = r"__________             .__          __    __          
\______   \ ____  __ __|  |   _____/  |__/  |_  ____  
 |       _//  _ \|  |  \  | _/ __ \   __\   __\/ __ \ 
 |    |   (  <_> )  |  /  |_\\  ___/|  |  |  | \  ___/ 
 |____|_  /\____/|____/|____/\\___  >__|  |__|  \\___  >
        \/                       \/                \/ ";

    println!("{} \n", Colour::Red.paint(titolo));
    println!("{}", Colour::Red.paint("Caesars Palace Casino - Roulette\n"));
    println!("{}", Colour::Yellow.paint("La cara e vecchia roulette, il gioco di casinò più iconico al mondo non perde mai il suo fascino!"));


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

    
    // loop main del gioco, tra una partita e l'altra si ritorna qua e si può decidere di uscire o di giocare ancora
    fn main(cred: i32) {
        let time = Duration::new(3, 0);
        let mut credito = cred;
        loop {
            credito = gioca(credito);
            if credito <= 0 {
                println!("Ha perso tutto! La accompagniamo all'uscita");
                thread::sleep(time/2 );
                atrio(credito);
            } else {
                println!("\nVuole continuare a giocare? (s/n)");
                let mut scelta = String::new();
                io::stdin().read_line(&mut scelta).expect("Inserisca una scelta valida: s/n");
                match scelta.trim().to_lowercase().as_str() {
                    "s" => {println!("\nScelta corretta, più si gioca più si vince!")},
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


    // funzione di una singola partita (un singolo giro di ruota), il giocatore qui sceglie quante puntate fare e vede il risultato, finita la partita si ritorna al main
    fn gioca(cred: i32) -> i32{
        pulisci_schermo();
        let mut credito = cred;
        let mut giocate:Vec<Giocata> = Vec::new();

        loop {
            let giocata = crea_giocata(credito);
            credito -= giocata.scommessa;
            giocate.push(giocata);
            println!("Vuole effettuare un'altra puntata? (s/n)");
            let mut scelta:String = String::new();
            io::stdin().read_line(&mut scelta).expect("Inserisca una scelta valida: s/n");
            match scelta.trim().to_lowercase().as_str() {
                "s" => (),
                "n" => break,
                _ => {
                    println!("Scelta non valida.");
                    println!("Vuole effettuare un'altra puntata? (s/n)");
                    let mut scelta:String = String::new();
                    io::stdin().read_line(&mut scelta).expect("Inserisca una scelta valida: s/n");
                    if scelta.trim().to_lowercase().as_str() == "s" {
                        ()
                    } else {
                        break
                    }
                }
            }
        }

        let scommessa_tot: i32 = giocate.iter().map(|giocata| giocata.scommessa).sum();
        let giocate_clone = giocate.clone();

        for giocata in giocate {
            match giocata.puntata {
                Puntata::Numero(valore) => {
                    println!("\nHai puntato {} euro su {:#?}.", giocata.scommessa, valore);
                    
                }
                Puntata::Colore(valore) => {println!("\nHa puntato {} euro su {:#?}.", giocata.scommessa, valore.trim_matches('"').replace("\n", ""));}
                Puntata::PariDispari(valore) => {println!("\nHa puntato {} euro sui numeri {:#?}.", giocata.scommessa, valore.trim_matches('\"').replace("\n", ""));}
                Puntata::PrimaMeta(_valore) => {println!("\nHa puntato {} euro sulla prima metà.", giocata.scommessa);}
                Puntata::SecondaMeta(_valore) => {println!("\nHa puntato {} euro sulla seconda metà.", giocata.scommessa);}
                Puntata::PrimaRiga(_valore) => {println!("\nHa puntato {} euro sulla prima riga.", giocata.scommessa);}
                Puntata::SecondaRiga(_valore) => {println!("\nHa puntato {} euro sulla seconda riga.", giocata.scommessa);}
                Puntata::TerzaRiga(_valore) => {println!("\nHa puntato {} euro sulla terza riga.", giocata.scommessa);}
                Puntata::Terzina(valore) => {println!("\nHa puntato {} euro su {:?}.", giocata.scommessa, valore);}
            }
        }
        println!("\n{}\n", Colour::Green.paint(format!("In totale ha puntato {} euro", scommessa_tot)));

        let numero_uscito = ruota();
        credito = vincita(numero_uscito, giocate_clone, credito);
        println!("\n{}", Colour::Green.paint(format!("Il suo credito finale è di {} euro", credito)));
        return credito
    }

    // funzione per creare una puntata, ogni volta che il giocatore decide di effetturare una puntata viene chiamata questa funzione che gli permette di scegliere su cosa puntare e quanto
    fn crea_giocata(mut credito: i32) -> Giocata {
        loop {
            println!("\n{}", Colour::Green.paint(format!("Il suo credito attuale è di {} euro, quanto vuole scommettere? €", credito)));
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Errore di lettura");
            let scommessa = input.trim().parse::<i32>().unwrap();

            match Some(scommessa) {
                Some(x) => {
                    if x > credito {
                        println!("La sua scommessa è troppo alta. Riprovi.");
                    } else {
                        credito -= scommessa;
                        println!("\nCosa vuole scommettere? \n 1. numero singolo \n 2. colore \n 3. pari o dispari \n 4. prima o seconda meta \n 5. prima, seconda o terza riga \n 6. terzina\n \nScelta: ");
                        let mut scelta = String::new();
                        io::stdin().read_line(&mut scelta).expect("Errore di lettura");
                        match scelta.trim()  {
                            "1" => {
                                loop {
                                    println!("\nSu che numero vuole puntare? (0 - 36)");
                                    let mut numero = String::new();
                                    io::stdin().read_line(&mut numero).expect("Errore di lettura");
                                    match numero.trim().parse::<i32>() {    
                                        Ok(x) => {
                                            if x >= 0 && x <= 36 {
                                                let puntata: Puntata = Puntata::Numero(x);
                                                println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro su un numero singolo: {:?}", scommessa, x)));
                                                let giocata:Giocata = Giocata {
                                                    scommessa: scommessa,
                                                    puntata: puntata,
                                                    scelta: scelta,
                                                };
                                                return giocata;
                                            } else {
                                                println!("Inserisca un numero valido.");
                                            }
                                        }
                                        Err(_) => {
                                        println!("\nInserisca un numero valido.");
                                        }
                                    }
                                }
                            },
                            "2" => {
                                loop {
                                    println!("\nSu quale colore vuole puntare? (rosso/nero)");
                                    let mut colore = String::new();
                                    io::stdin().read_line(&mut colore).expect("Errore di lettura");
                                    match colore.trim().to_lowercase().as_str() {
                                        "rosso" => {
                                            let puntata: Puntata = Puntata::Colore(colore);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro su un colore: rosso", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "nero" => {
                                            let puntata: Puntata = Puntata::Colore(colore);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro su un colore: nero", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        _ => {
                                            println!("Inserisca un colore valido.");
                                        }
                                    }
                                }
                            },
                            "3" => {
                                loop {
                                    println!("\nVuole puntare sui pari o sui dispari? (pari/dispari)");
                                    let mut pari_dispari = String::new();
                                    io::stdin().read_line(&mut pari_dispari).expect("Errore di lettura");
                                    match pari_dispari.trim().to_lowercase().as_str() {
                                        "pari" => {
                                            let puntata: Puntata = Puntata::PariDispari(pari_dispari);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sui numeri pari", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "dispari" => {
                                            let puntata: Puntata = Puntata::PariDispari(pari_dispari);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sui numeri dispari", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        _ => {
                                            println!("Inserisca pari o dispari.");
                                        }
                                    }
                                }
                            },
                            "4" => {
                                loop {
                                    println!("\nVuole puntare sulla prima o sulla seconda metà? (prima/seconda)");
                                    let mut prima_seconda_meta = String::new();
                                    io::stdin().read_line(&mut prima_seconda_meta).expect("Errore di lettura");
                                    match prima_seconda_meta.trim().to_lowercase().as_str() {
                                        "prima" => {
                                            let puntata: Puntata = Puntata::PrimaMeta(prima_seconda_meta);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla prima metà", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "seconda" => {
                                            let puntata: Puntata = Puntata::SecondaMeta(prima_seconda_meta);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla seconda metà", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        _ => {
                                            println!("Inserisca prima o seconda metà.");
                                        }
                                    }
                                }
                            },
                            "5" => {
                                loop {
                                    println!("\nVuole puntare sulla prima, seconda o terza riga? (prima/seconda/terza)");
                                    let mut prima_seconda_terza = String::new();
                                    io::stdin().read_line(&mut prima_seconda_terza).expect("Errore di lettura");
                                    match prima_seconda_terza.trim().to_lowercase().as_str() {
                                        "prima" => {
                                            let puntata: Puntata = Puntata::PrimaRiga(prima_seconda_terza);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla prima riga", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "seconda" => {
                                            let puntata: Puntata = Puntata::SecondaRiga(prima_seconda_terza);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla seconda riga", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "terza" => {
                                            let puntata: Puntata = Puntata::TerzaRiga(prima_seconda_terza);
                                            println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla terza riga", scommessa)));
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        _ => {
                                            println!("Inserisca prima, seconda o terza riga.");
                                        }
                                    }
                                }
                            },
                            "6" => {
                                loop {
                                    println!("\nSu quale terzina vuole puntare? (1 - 12)");
                                    let mut terzina = String::new();
                                    io::stdin().read_line(&mut terzina).expect("Errore di lettura");
                                    match terzina.trim().parse::<i32>() {
                                        Ok(x) => {
                                            if x >= 1 && x <= 12 {

                                                match x {
                                                    1 => {
                                                        let puntata: Puntata = Puntata::Terzina([1, 2, 3]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla prima terzina: 1, 2, 3", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    2 => {
                                                        let puntata: Puntata = Puntata::Terzina([4, 5, 6]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla seconda terzina: 4, 5, 6", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    3 => {
                                                        let puntata: Puntata = Puntata::Terzina([7, 8, 9]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla terza terzina: 7, 8, 9", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    4 => {
                                                        let puntata: Puntata = Puntata::Terzina([10, 11, 12]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla quarta terzina: 10, 11, 12", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    5 => {
                                                        let puntata: Puntata = Puntata::Terzina([13, 14, 15]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla quinta terzina: 13, 14, 15", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    6 => {
                                                        let puntata: Puntata = Puntata::Terzina([16, 17, 18]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla sesta terzina: 16, 17, 18", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    7 => {
                                                        let puntata: Puntata = Puntata::Terzina([19, 20, 21]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla settima terzina: 19, 20, 21", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    8 => {
                                                        let puntata: Puntata = Puntata::Terzina([22, 23, 24]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sull'ottava terzina: 22, 23, 24", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    9 => {
                                                        let puntata: Puntata = Puntata::Terzina([25, 26, 27]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla nona terzina: 25, 26, 27", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    10 => {
                                                        let puntata: Puntata = Puntata::Terzina([28, 29, 30]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla decima terzina: 28, 29, 30", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    11 => {
                                                        let puntata: Puntata = Puntata::Terzina([31, 32, 33]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sull'undicesima terzina: 31, 32, 33", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    12 => {
                                                        let puntata: Puntata = Puntata::Terzina([34, 35, 36]);
                                                        println!("\n{}", Colour::Green.paint(format!("Ha scommesso {} euro sulla dodicesima terzina: 34, 35, 36", scommessa)));
                                                        let giocata:Giocata = Giocata {
                                                            scommessa: scommessa,
                                                            puntata: puntata,
                                                            scelta: scelta,
                                                        };
                                                        return giocata;
                                                    }
                                                    _ => {println!("Inserisca un numero valido")}

                                                }
                                            
                                            } else {
                                                println!("Inserisca un numero valido.");
                                            }
                                        }
                                        Err(_) => {
                                        println!("\nInserisca un numero valido.");
                                        }
                                    };
                                }
                            },
                            _ => {
                                let time = Duration::new(2, 0);
                                println!("Scelta non valida, giocata annullata. Ricominci per favore.");
                                thread::sleep(time);
                            },
                        }
                    }
                },
                None => {
                    println!("Inserisca un numero valido.");
                }
            }
        }
    }
}


//funzioni ausiliarie per il gioco principale

fn pari_dispari() -> (Vec<i32>, Vec<i32>) {
    let mut pari:Vec<i32> = Vec::new();
    let mut dispari:Vec<i32> = Vec::new();
    for i in 1..37 {
        if i % 2 == 0 {
            pari.push(i);
        } else if i % 2 != 0 {
            dispari.push(i);
        }
    }
    (pari, dispari)
}

//animazione della ruota che gira, estrae un numero casuale tra 0 e 36
fn ruota() -> i32 {
    let time = Duration::new(1, 0);
    let mut gen = thread_rng();
    let numero:i32 = (0..37).choose(&mut gen).unwrap().clone();
    thread::sleep(time*2);
    println!("{}", Colour::Yellow.paint("Audentes fortuna iuvat! ~ Enea\n"));
    thread::sleep(time*3/2);
    println!("{}", Colour::Yellow.paint("La ruota sta girando..."));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("3"));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("2"));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("1"));
    thread::sleep(time);
    println!("{}", Colour::Yellow.paint("0!\n"));
    println!("{}", Colour::Red.paint(format!("Il numero uscito dalla ruota e' {}", numero)));

    numero
}

//funzione per confrontare il numero estratto dalla ruota con le puntate del giocatore, calcola la vincita totale
fn vincita(numero:i32, giocate:Vec<Giocata>, mut credito: i32) -> i32{
    let (pari, dispari) = pari_dispari();
    let _primi_12:Vec<i32> = (1..13).into_iter().collect();
    let _secondi_12:Vec<i32> = (13..25).into_iter().collect();
    let _terzi_12:Vec<i32> = (25..37).into_iter().collect();
    let prima_meta:Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
    let seconda_meta:Vec<i32> = (19..37).into_iter().collect();
    let num_rossi:[i32; 18] = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
    let num_neri:[i32; 18] = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];
    let prima_riga:[i32; 12] = [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34];
    let seconda_riga:[i32; 12] = [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35];
    let terza_riga:[i32; 12] = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36];

    for giocata in giocate {
        let scelta = giocata.scelta.replace("\n", "");
        let puntata = giocata.puntata;
        let scommessa = giocata.scommessa;
        if scelta == "1" {
             match puntata {
                Puntata::Numero(valore) => {
                    if numero == valore {
                        println!("\nHa vinto la puntata di {} euro: numero singolo ({})", scommessa, valore);
                        credito += giocata.scommessa*36;
                    } else {
                        println!("\nHa perso la puntata di {} euro: numero singolo ({})", scommessa, valore);
                    }
                }
                _ => println!("Inserisca un numero valido"),
            }
        }else if scelta == "2" {
            match puntata {
                Puntata::Colore(colore) => {
                    if colore == "rosso\n" {
                        if num_rossi.contains(&numero) {
                            println!("\nHa vinto la puntata di {} euro: colore (rosso)", scommessa);
                            credito += giocata.scommessa*2;
                        } else {
                            println!("\nHa perso la puntata di {} euro: colore (rosso)", scommessa);
                        }
                    } else if colore == "nero\n" {
                        if num_neri.contains(&numero) {
                            println!("\nHa vinto la puntata di {} euro: colore (nero)", scommessa);
                            credito += giocata.scommessa*2;
                        } else {
                            println!("\nHa perso la puntata di {} euro: colore (nero)", scommessa);
                        }
                    }
                }
                _ => println!("Inserisca un numero valido"),
            }
        }else if scelta == "3" {
            match puntata {
                Puntata::PariDispari(paridispari) => {
                    if paridispari == "pari\n" {
                        if pari.contains(&numero) {
                            println!("\nHa vinto la puntata di {} euro: pari/dispari (pari)", scommessa);
                            credito += giocata.scommessa*2;
                        } else {
                            println!("\nHa perso la puntata di {} euro: pari/dispari (pari)", scommessa);
                        }
                    } else if paridispari == "dispari\n" {
                        if dispari.contains(&numero) {
                            println!("\nHa vinto la puntata di {} euro: pari/dispari (dispari)", scommessa);
                            credito += giocata.scommessa*2;
                        } else {
                            println!("\nHa perso la puntata di {} euro: pari/dispari (dispari)", scommessa);
                        }
                    }
                }
                _ => println!("Inserisca un numero valido"),
            }
        }else if scelta == "4" {
            match puntata {
                Puntata::PrimaMeta(_primameta) => {
                    if prima_meta.contains(&numero) {
                        println!("\nHa vinto la puntata di {} euro: prima metà", scommessa);
                        credito += giocata.scommessa*2;
                    }else{
                        println!("\nHa perso la puntata di {} euro: prima metà", scommessa);
                    }
                    
                }
                Puntata::SecondaMeta(_secondameta) => {
                    if seconda_meta.contains(&numero) {
                        println!("\nHa vinto la puntata di {} euro: seconda metà", scommessa);
                        credito += giocata.scommessa*2;
                    }else{
                        println!("\nHa perso la puntata di {} euro: seconda metà", scommessa);
                    }
                }
                _ => println!("Inserisca un numero valido"),
            }
        }else if scelta == "5" {
            match puntata {
                Puntata::PrimaRiga(_primariga) => {
                    if prima_riga.contains(&numero) {
                        println!("\nHa vinto la puntata di {} euro: prima riga", scommessa);
                        credito += giocata.scommessa*3;
                    } else {
                        println!("\nHa perso la puntata di {} euro: prima riga", scommessa);
                    }
                }
                Puntata::SecondaRiga(_secondariga) => {
                    if seconda_riga.contains(&numero) {
                        println!("\nHa vinto la puntata di {} euro: seconda riga", scommessa);
                        credito += giocata.scommessa*3;
                    } else {
                        println!("\nHa perso la puntata di {} euro: seconda riga", scommessa);
                    }
                }
                Puntata::TerzaRiga(_terzariga) => {
                    if terza_riga.contains(&numero) {
                        println!("\nHa vinto la puntata di {} euro: terza riga", scommessa);
                        credito += giocata.scommessa*3;
                    } else {
                        println!("\nHa perso la puntata di {} euro: terza riga", scommessa);
                    }
                }
                _ => println!("Inserisca un numero valido"),
            }
        }else if scelta == "6"{
            match puntata {
                Puntata::Terzina(terzina) => {
                    if terzina.contains(&numero) {
                        println!("\nHa vinto la puntata di {} euro: terzina", scommessa);
                        credito += giocata.scommessa*12;
                    } else {
                        println!("\nHa perso la puntata di {} euro: terzina", scommessa);
                    }
                }
                _ => println!("Inserisca un numero valido"),
            }
        }else{
            println!("Inserisca un numero valido");
        }
    }

    credito
}