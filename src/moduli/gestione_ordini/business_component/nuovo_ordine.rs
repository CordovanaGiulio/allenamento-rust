use std::io;

use chrono::NaiveDate;

use crate::moduli::gestione_ordini::business_component::model::ordine::Ordine;

pub fn get_ordine()-> Ordine{
    let dati_ordine: Vec<String>;
    let mut input;
    loop {
        input= String::new();
        println!("Inserisci Username del cliente, data, numero di prodotti, prezzo dei prodotti separati da spazi:");
        io::stdin()
            .read_line(&mut input)
            .expect("Non sono riuscito a prendere in input i dati dati dell'ordine username e data");
        match is_valid_input_ordine(&input){
            true => {
                dati_ordine= input.split_whitespace().map(|s| s.to_string()).collect();
                break;
            },
            false => println!("Dati non validi"),
        }
    }

    return Ordine{
        username_cli: dati_ordine[0].to_string(),
        data_ordine: NaiveDate::parse_from_str(&dati_ordine[1], "%d/%m/%Y").unwrap(),
        num_prodotti: dati_ordine[2].parse::<i32>().unwrap(),
        prezzo: dati_ordine[3].parse::<i32>().unwrap(),
    };
}

fn is_valid_input_ordine(input:&str)->bool{
    if input.is_empty(){
        return false;
    }
    let datas: Vec<&str> = input.split_whitespace().collect();
    if datas.len()!=4{
        return false
    }
    // let username = datas[0];
    // if !username_exists(username){
    //     return false;
    // }
    let data = NaiveDate::parse_from_str(datas[1], "%d/%m/%Y");
    if data.is_err(){
        return false;
    }
    //numero di prodotti
    if datas[2].parse::<i32>().is_err() {
        return false
    }
    //prezzo
    if datas[3].parse::<i32>().is_err() {
        return false
    }
    return true;

}
