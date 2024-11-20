use rand::prelude::IteratorRandom;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::process::{Command, exit};

#[derive(Debug)]
pub enum Carta<'a> {
    Asso {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Due {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Tre {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Quattro {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Cinque {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Sei {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Sette {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Otto {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Nove {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Dieci {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Jack {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Donna {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Re {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
}

#[derive(Debug, Clone)]
pub enum Puntata {
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
pub struct Giocata {
    pub scommessa: i32,
    pub puntata: Puntata,
    pub scelta: String,
}

#[derive(Debug, Clone)]
pub struct Cavallo {
    pub nome:String,
    pub colore:String,
    pub velocità:i32,
    pub posizione: usize
}

pub fn genera_mazzo() -> Vec<Carta<'static>> {
    const SEMI: [&str; 4] = ["cuori", "quadri", "picche", "fiori"];
    let mut mazzo: Vec<Carta> = Vec::new();
    for seme in SEMI {
        mazzo.push(Carta::Asso {
            nome: "asso",
            valore: 1,
            seme: seme,
        });
        mazzo.push(Carta::Due {
            nome: "due",
            valore: 2,
            seme: seme,
        });
        mazzo.push(Carta::Tre {
            nome: "tre",
            valore: 3,
            seme: seme,
        });
        mazzo.push(Carta::Quattro {
            nome: "quattro",
            valore: 4,
            seme: seme,
        });
        mazzo.push(Carta::Cinque {
            nome: "cinque",
            valore: 5,
            seme: seme,
        });
        mazzo.push(Carta::Sei {
            nome: "sei",
            valore: 6,
            seme: seme,
        });
        mazzo.push(Carta::Sette {
            nome: "sette",
            valore: 7,
            seme: seme,
        });
        mazzo.push(Carta::Otto {
            nome: "otto",
            valore: 8,
            seme: seme,
        });
        mazzo.push(Carta::Nove {
            nome: "nove",
            valore: 9,
            seme: seme,
        });
        mazzo.push(Carta::Dieci {
            nome: "dieci",
            valore: 10,
            seme: seme,
        });
        mazzo.push(Carta::Jack {
            nome: "jack",
            valore: 10,
            seme: seme,
        });
        mazzo.push(Carta::Donna {
            nome: "donna",
            valore: 10,
            seme: seme,
        });
        mazzo.push(Carta::Re {
            nome: "re",
            valore: 10,
            seme: seme,
        });
    }
    mazzo
}

pub fn pesca_carta(mazzo: &mut Vec<Carta<'static>>) {
    let mut generatore = thread_rng();

    if let Some(indice) = (0..mazzo.len()).choose(&mut generatore) {
        let carta = mazzo.remove(indice);
        match carta {
            Carta::Asso { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Due { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Tre { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Quattro { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Cinque { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Sei { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Sette { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Otto { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Nove { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Dieci { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Jack { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Donna { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
            Carta::Re { nome, valore, seme } => {
                println!("{} di {} con valore {}", nome, seme, valore);
            }
        }
    } else {
        println!("Non ci sono più carte nel mazzo");
    }
}

pub fn genera_cavalli() -> Vec<Cavallo>{
    let mut generatore = thread_rng();

    let mut nomi = vec!["Pegasus", "Saetta", "Hurricane", "Blitz", "Tornado", "Aaron", "Atlantis", "Dorian", "Enea", "Fiona", "Gerard", "Daphne", "Ermes", "Bob", "Gaia", "Hector", "Kaos", "Leonard", "Lupin", "Megan", "Nebbia"];
    nomi.shuffle(&mut generatore);
    let mut colori = vec!["nero", "bianco", "marrone", "grigio", "sauro", "bruno"];
    colori.shuffle(&mut generatore);
    let mut velocità = vec![2, 4, 6, 9, 11];
    velocità.shuffle(&mut generatore);

    let mut cavalli = Vec::new();
    for i in 0..5 {
        let cavallo = Cavallo {
            nome: nomi[i].to_string(),
            colore: colori[i].to_string(),
            velocità: velocità[i],
            posizione: 0
        };
        cavalli.push(cavallo);
    }
    cavalli
}

pub fn pulisci_schermo() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

pub fn esci() {
    println!("\nGrazie per esserci venuta a trovare qui al Caesars, la aspettiamo per un'altra giocata quando vuole!\nArrivederci");
    exit(0);
}