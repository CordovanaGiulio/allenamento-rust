use chrono::NaiveDate;

use crate::moduli::utilities::utilities_cordo::read_num_positivo;

use super::classes::kc3_1::stampa_mese;

// kc3.1
// Creare un programma calendario. Il programma deve dare la possibilità di inserire il mese e l'anno attuale
// per mostrare le informazioni relative al mese evidenziando i giorni del mese e il giorno attuale.Usare le
// classi calendario dell'API.
pub fn calendario(){
    let mut giorno :u32=32;
    let mut mese :u32=13;
    let anno :u32;
    while giorno>31 {
        println!("Inserisci un giorno: (Valori 1..31)");
        giorno = read_num_positivo();
    }
    while mese<1 || mese>12 {
        println!("Inserisci un mese: (Valori 1..12)");
        mese = read_num_positivo();
    }

    println!("Inserisci un anno: ");
    anno = read_num_positivo();

    let data: NaiveDate;
    match NaiveDate::from_ymd_opt(anno.try_into().unwrap(), mese, giorno){
        Some(new) => data=new,
        None => panic!("Data non valida"),
    };
    stampa_mese(data);


}