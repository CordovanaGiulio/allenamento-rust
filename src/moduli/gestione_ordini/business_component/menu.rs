use std::{error::Error, io};


#[derive (Debug, Clone)]
pub enum Scelta{
    NuovoOrdine,
    NuovoRecord,
    CalcoloOrdiniTot,
    Riepilogo,
    Error
}

pub fn get_scelta()-> Scelta{
    let mut input= String::new();
    while !is_valid_input_scelta(&input){
        input= String::new();
        println!("Inserisci un valore tra: 1(Nuovo Ordine), 2 (NuovoRecord), 3 (Calcolo Ordini Totale), 4 (Riepilogo)");
        io::stdin()
            .read_line(&mut input)
            .expect("Non sono riuscito a prendere in input il la scelta di menu");
    }
    return match input.as_str() {
        "1" => Scelta::NuovoOrdine,
        "2" => Scelta::NuovoRecord,
        "3" => Scelta::CalcoloOrdiniTot,
        "4" => Scelta::Riepilogo,
        _ => Scelta::Error
    };
}

fn is_valid_input_scelta(input:&str)->bool{
    if input.is_empty(){
        return false;
    }
    match input {
        "1" | "2" | "3" | "4" => {
            return true;
        },
        _ => {
            return false;
        }
    }
}