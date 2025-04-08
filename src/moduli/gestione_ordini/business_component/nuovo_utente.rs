use std::io;

use crate::moduli::gestione_ordini::business_component::model::utente::Utente;

pub fn get_utente()-> Utente{
    let dati_utente: Vec<String>;
    let mut input;
    loop {
        input= String::new();
        println!("Inserisci Username Nome Cognome Eta separati da spazi:");
        io::stdin()
            .read_line(&mut input)
            .expect("Non sono riuscito a prendere in input l'utente");
        match is_valid_input_utente(&input){
            true => {
                dati_utente=input.split_whitespace().map(|s| s.to_string()).collect();
                break
            },
            false => {
                println!("Dati non validi")
            },
        }

    }
    return Utente{
        username: dati_utente[0].to_string(),
        nome: dati_utente[1].to_string(),
        cognome: dati_utente[2].to_string(),
        eta: dati_utente[3].parse::<i32>().unwrap(),
        totale:0,
    };
}

fn is_valid_input_utente(input:&str)->bool{
    if input.is_empty(){
        return false;
    }
    let datas: Vec<&str> = input.split_whitespace().collect();
    if datas.len()!=4{
        return false;
    }
    let username = datas[0];
    // if username_exists(){
    //     //controllo esistenza su file 
    //     return false;
    // }
    let nome = datas[1];
    if !nome.chars().all(|c| c.is_alphabetic()){
        return false;
    }
    let cognome = datas[2];
    if !cognome.chars().all(|c| c.is_alphabetic()){
        return false;
    }
    
    let eta = datas[3];
    return eta.parse::<i32>().is_ok();
}