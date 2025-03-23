use chrono::{NaiveDate, TimeZone, Utc};
use rand::Rng;

use crate::moduli::allenamento_prove::giocatore::Giocatore;
use crate::moduli::allenamento_prove::giocatore::TurnoGiocatore;

use super::giocatore::VittoriaGiocatore;

// kc1.1
// Scrivere un programma che calcola il MCD di due numeri positivi immessi attraverso Scanner.
pub fn calcola_mcd(){
    let mut num1: i32=24;
    let mut num2: i32=324;
    
    if num1>num2{
        let temp:i32=num1;
        num1=num2;
        num2=temp;
    }

    for i in (2..num1).rev(){
        if num1%i==0 && num2%i==0{
            print!("{}\t", i);
            return;
        }
    }
    print!("{}\t", 1);
}

// kc1.2
// Scrivere un programma che stampa i numeri primi minori di 100
pub fn stampa_primi(){
    let mut num: i32=1;
    while num<100{
        let mut flag_primo=true;
        for i in 2..num{
            if num%i==0 {
                flag_primo=false;
            }
        }
        if flag_primo {
            print!("{}\t", num);
        }
        num += 1;
    }
}

// kc1.4
// Scrivete un programma che legga una sequenza di al massimo 100 date, nella forma dd/mm/yyyy, che non è considerata parte della sequenza.
// Alla fine il programma deve stampare la data più recente.
pub fn date_100 (){
    let date_formattate=genera_date();
    let mut date:Vec<NaiveDate>= Vec::new();

    for data in date_formattate{
        let data_parsed=NaiveDate::parse_from_str( &data, "%d/%m/%Y");
        match data_parsed {
            Ok(data_parsed) => date.push(data_parsed),
            Err(_) => println!("non posso parsare questa stringa : {:?}", data),
        }
    }
    let mut min=*date.get(0).unwrap();
    for data in date.iter(){
        if data.le(&min){
            min=*data;
        }
    }
    println!("la data piu recente è : {:?}", min);

}
fn genera_date()->Vec<String>{
    let mut date_res: Vec<String>=Vec::new();
    let mut rand = rand::rng();
    let num =rand.random_range(0..100);
    for _ in 0..num{
        let random_seconds: i64 = rand.random_range(0..=1_000_000_000);
        let random_date= Utc.timestamp_opt(random_seconds, 0).unwrap().format("%d/%m/%Y").to_string();
        println!("{}",random_date);
        date_res.push(random_date);
    }
    return date_res;
}

// kc1.5
// Ogni giocatore lancia due dadi.
// Se la somma al primo tiro è 7 o 11 il giocatore ha vinto.
// Se la somma al primo tiro è 2,3 o 12 il giocatore ha perso.
// Se la somma al primo tiro è 4,5,6,8,9 o 10 questa somma diventa il punteggio.
// Per vincere è necessario continuare a tirare i dati finchè non si totalizza il punteggio realizzato con il primo
// tiro.
// Il giocatore perde se totalizza 7 prima di raggiungere il punteggio del primo tiro
pub fn gioco_dadi (){
    let mut giocatore1= Giocatore::new(TurnoGiocatore::Primo);
    let mut giocatore2= Giocatore::new(TurnoGiocatore::Secondo);
    while giocatore1.vittoria == VittoriaGiocatore::Tbd && giocatore2.vittoria == VittoriaGiocatore::Tbd {
        giocatore1.tiro_dadi();
        giocatore2.tiro_dadi();
        println!("Turno finito\ngiocatore1: {:?}\ngiocatore2: {:?}", giocatore1,giocatore2);
    }
    if giocatore1.vittoria == VittoriaGiocatore::Vinto {
        println!("Ha vinto il primo giocatore");
    }else if giocatore2.vittoria == VittoriaGiocatore::Vinto {
        println!("Ha vinto il secondo giocatore");
    }else if giocatore1.vittoria == VittoriaGiocatore::Perso {
        println!("Ha vinto il secondo giocatore");
    }else if giocatore2.vittoria == VittoriaGiocatore::Perso {
        println!("Ha vinto il primo giocatore");
    }else{
        panic!("giocatore1: {:?},giocatore2: {:?}", giocatore1,giocatore2);
    }
}
