use std::{vec, io};
use std::thread;
use std::time::Duration;
use ansi_term::Colour;
use rand::{seq::IteratorRandom, thread_rng};
use crate::utils::pulisci_schermo;
use crate::atrio::atrio;

#[derive(Debug)]
enum Puntata {
    Numero(i32),
    Colore(String),
    PariDispari(String),
    PrimaMeta(String),
    SecondaMeta(String),
    PrimaRiga(String),
    SecondaRiga(String),
    TerzaRiga(String),
    Terzina(String),
}
#[derive(Debug)]
struct Giocata {
    scommessa: i32,
    puntata: Puntata,
    scelta: String,
}

#[allow(unused_variables, unused_mut)]
pub fn run(credito: i32) {
    let time = Duration::new(1, 0);

    pulisci_schermo();
    let titolo = r"__________             .__          __    __          
\______   \ ____  __ __|  |   _____/  |__/  |_  ____  
 |       _//  _ \|  |  \  | _/ __ \   __\   __\/ __ \ 
 |    |   (  <_> )  |  /  |_\  ___/|  |  |  | \  ___/ 
 |____|_  /\____/|____/|____/\___  >__|  |__|  \___  >
        \/                       \/                \/ ";

    println!("{} \n", Colour::Red.paint(titolo));
    println!("{}", Colour::Red.paint("Caesars Palace Casino - Roulette\n"));
    println!("{}", Colour::Yellow.paint("La cara e vecchia roulette, il gioco di casinò più iconico al mondo non perde mai il suo fascino!"));
    
    //let mut credito: u32 = credito;
    println!("\nIl suo credito attuale è di {} euro", credito);

    println!("\nVuole iniziare a giocare? (s/n)");
    let mut scelta = String::new();
    io::stdin().read_line(&mut scelta).expect("Errore di lettura");
    match scelta.trim().to_lowercase().as_str() {
        "s" => main(credito),
        "n" => atrio(credito),
        _ => println!("Scelta non valida."),
    }

    // raggruppamento dei numeri
    let numeri:Vec<u32>= (0..37).into_iter().collect();
    let (pari, dispari) = pari_dispari();
    let primi_12:Vec<u32> = (1..13).into_iter().collect();
    let secondi_12:Vec<u32> = (13..25).into_iter().collect();
    let terzi_12:Vec<u32> = (25..37).into_iter().collect();
    let prima_meta:Vec<u32> = (1..19).into_iter().collect();
    let seconda_meta:Vec<u32> = (19..37).into_iter().collect();
    let num_rossi: Vec<i32> = vec![1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
    let num_neri: Vec<i32> = vec![2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];
    let prima_riga: Vec<i32> = vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34];
    let seconda_riga: Vec<i32> = vec![2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35];
    let terza_riga: Vec<i32> = vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36];
    let terzina_1: Vec<i32> = vec![1, 2, 3];
    let terzina_2: Vec<i32> = vec![4, 5, 6];
    let terzina_3: Vec<i32> = vec![7, 8, 9];
    let terzina_4: Vec<i32> = vec![10, 11, 12];
    let terzina_5: Vec<i32> = vec![13, 14, 15];
    let terzina_6: Vec<i32> = vec![16, 17, 18];
    let terzina_7: Vec<i32> = vec![19, 20, 21];
    let terzina_8: Vec<i32> = vec![22, 23, 24];
    let terzina_9: Vec<i32> = vec![25, 26, 27];
    let terzina_10: Vec<i32> = vec![28, 29, 30];
    let terzina_11: Vec<i32> = vec![31, 32, 33];
    let terzina_12: Vec<i32> = vec![34, 35, 36];


    fn main(cred: i32) {
        let time = Duration::new(3, 0);
        println!("Allora via alle danze!");
        loop {
            let mut credito = gioca(cred);
            if credito <= 0 {
                println!("Ha perso tutto! La accompagniamo all'uscita");
                atrio(credito);
            } else {
                println!("\nVuole continuare a giocare? (s/n)");
                let mut scelta = String::new();
                io::stdin().read_line(&mut scelta).expect("Errore di lettura");
                match scelta.trim().to_lowercase().as_str() {
                    "s" => (),
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

    fn gioca(cred: i32) -> i32{
        let mut credito = cred;
        let numero_uscito = ruota();
        



        return credito
    }


    fn giocata(mut credito: i32) -> Giocata {
        println!("\nIl suo credito attuale è di {} euro, quanto vuole scommettere? €", credito);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Errore di lettura");
        let scommessa = input.trim().parse::<i32>().unwrap();
        loop {
            match Some(scommessa) {
                Some(x) => {
                    if x > credito {
                        println!("La sua scommessa è troppo alta. Riprova.");
                    } else {
                        credito -= scommessa;
                        println!("Cosa vuole scommettere? \n 1. numero singolo \n 2. colore \n 3. pari o dispari \n 4. prima o seconda meta \n 5. prima, seconda o terza riga \n 6. terzina \n scelta: ");
                        let mut scelta = String::new();
                        io::stdin().read_line(&mut scelta).expect("Errore di lettura");
                        match scelta.trim()  {
                            "1" => {
                                loop {
                                    println!("Su che numero vuole puntare? (0 - 36)");
                                    let mut numero = String::new();
                                    io::stdin().read_line(&mut numero).expect("Errore di lettura");
                                    match numero.trim().parse::<i32>() {
                                        Ok(x) => {
                                            if x >= 0 && x <= 36 {
                                                let mut puntata: Puntata = Puntata::Numero(x);
                                                println!("Ha scommesso {} euro su un numero singolo: {:?}", scommessa, puntata);
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
                                    };
                                }
                            }
                            "2" => {
                                loop {
                                    println!("Su quale colore vuole puntare? (rosso/nero)");
                                    let mut colore = String::new();
                                    io::stdin().read_line(&mut colore).expect("Errore di lettura");
                                    match colore.trim().to_lowercase().as_str() {
                                        "rosso" => {
                                            let mut puntata: Puntata = Puntata::Colore(colore);
                                            println!("Ha scommesso {} euro su un colore: {:?}", scommessa, puntata);
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "nero" => {
                                            let mut puntata: Puntata = Puntata::Colore(colore);
                                            println!("Ha scommesso {} euro su un colore: {:?}", scommessa, puntata);
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
                            }
                            "3" => {
                                loop {
                                    println!("Vuole puntare sui pari o sui dispari? (pari/dispari)");
                                    let mut pari_dispari = String::new();
                                    io::stdin().read_line(&mut pari_dispari).expect("Errore di lettura");
                                    match pari_dispari.trim().to_lowercase().as_str() {
                                        "pari" => {
                                            let mut puntata: Puntata = Puntata::PariDispari(pari_dispari);
                                            println!("Ha scommesso {} euro su pari o dispari: {:?}", scommessa, puntata);
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "dispari" => {
                                            let mut puntata: Puntata = Puntata::PariDispari(pari_dispari);
                                            println!("Ha scommesso {} euro su pari o dispari: {:?}", scommessa, puntata);
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
                            }
                            "4" => {
                                loop {
                                    println!("Vuole puntare sulla prima o sulla seconda meta? (prima/seconda)");
                                    let mut prima_seconda_meta = String::new();
                                    io::stdin().read_line(&mut prima_seconda_meta).expect("Errore di lettura");
                                    match prima_seconda_meta.trim().to_lowercase().as_str() {
                                        "prima" => {
                                            let mut puntata: Puntata = Puntata::PrimaMeta(prima_seconda_meta);
                                            println!("Ha scommesso {} euro su prima o seconda meta: {:?}", scommessa, puntata);
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        "seconda" => {
                                            let mut puntata: Puntata = Puntata::SecondaMeta(prima_seconda_meta);
                                            println!("Ha scommesso {} euro su prima o seconda meta: {:?}", scommessa, puntata);
                                            let giocata:Giocata = Giocata {
                                                scommessa: scommessa,
                                                puntata: puntata,
                                                scelta: scelta,
                                            };
                                            return giocata;
                                        }
                                        _ => {
                                            println!("Inserisca prima o seconda meta.");
                                        }
                                    }
                                }
                            }
                            "5" => {}
                            "6" => {}
                            _ => (),
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
 
fn pari_dispari() -> (Vec<u32>, Vec<u32>) {
    let mut pari:Vec<u32> = Vec::new();
    let mut dispari:Vec<u32> = Vec::new();
    for i in 1..37 {
        if i % 2 == 0 {
            pari.push(i);
        } else if i % 2 != 0 {
            dispari.push(i);
        }
    }
    (pari, dispari)
}

fn ruota() -> u32 {
    let mut gen = thread_rng();
    let numero:u32 = (0..37).choose(&mut gen).unwrap().clone();
    numero
}