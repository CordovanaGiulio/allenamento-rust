use rand::Rng;
use core::panic;
use std::{i32, io};
use crate::moduli::allenamento_prove::classes::kc2_3_posto::Classe;
use crate::moduli::utilities::utilities_cordo::SIZE;
use crate::moduli::allenamento_prove::classes::kc2_1_studente_voto::StudenteVoto;

use super::classes::kc2_2_studente::Studente;
use super::classes::kc2_2_esame::{Esame, TipoEsame};
use super::classes::kc2_3_vagone::Vagone;

// kc2.1
// In una matrice sono presenti i seguenti dati:
// riga: nome studente
// colonna: valutazione studente
// Calcolare le seguenti statistiche:
// • voto più basso
// • voto più alto
// • media per studente
// • stampa complessiva dei voti per sudente ordinando da studente più meritevole a meno meritevole
pub fn studenti_voti_matrice(){
    let mut rand = rand::rng();
    let matrice_studenti: [[StudenteVoto; 2]; SIZE]= [
        [StudenteVoto::Studente("Alberto".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Bruno".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Carlo".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Dario".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Ezio".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Fabrizio".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Giovanni".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Ilaria".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
        [StudenteVoto::Studente("Laura".to_string()), StudenteVoto::Voto(rand.random_range(0..=10))],
    ];
    let mut voto_basso=matrice_studenti[0].clone();
    for i in 0..SIZE{
        println!("{:?}",matrice_studenti[i]);
        let voto=matrice_studenti[i][1].clone().get_voto().unwrap();
        if voto<voto_basso[1].clone().get_voto().unwrap(){
            voto_basso=matrice_studenti[i].clone();
        }
    }
    println!("Il voto piu basso è: {:?}",voto_basso);

    let mut voto_alto=matrice_studenti[0].clone();
    for i in 0..SIZE{
        let voto=matrice_studenti[i][1].clone().get_voto().unwrap();
        if voto>voto_alto[1].clone().get_voto().unwrap(){
            voto_alto=matrice_studenti[i].clone();
        }
    }
    println!("Il voto piu alto è: {:?}",voto_alto);

    let mut media: f32=0.0;
    for i in 0..SIZE{
        media+=matrice_studenti[i][1].clone().get_voto().unwrap() as f32;
    }
    println!("La media della classe è: {:.2}",media/SIZE as f32);
}

// kc2.2
// Nel primo array (dimensione 3) vengono memorizzati: matricola, nome, cognome.
// Non devono esistere due studenti con la stessa matricola. (controllare tale constraint)
// Nel secondo array (dimensione 7) vengono memorizzati: <matricola, esame, voto>.
// Più record matricola possono essere associati a diversi esami.
// Dopo aver inserito i dati tramite linea di comando nelle due strutture creare una voce di riepilogo per
// visualizzare gli studenti e i rispettivi esami.
#[allow(dead_code)]
pub fn matricole(){
    let mut studenti:[Studente; 3]=Default::default();
    let mut esami: [Esame;7]=[
        Esame::new(studenti[0].matricola.clone(), TipoEsame::Analisi),
        Esame::new(studenti[2].matricola.clone(), TipoEsame::Informatica),
        Esame::new(studenti[0].matricola.clone(), TipoEsame::Chimica),
        Esame::new(studenti[1].matricola.clone(), TipoEsame::Analisi),
        Esame::new(studenti[2].matricola.clone(), TipoEsame::Analisi),
        Esame::new(studenti[0].matricola.clone(), TipoEsame::Chimica),
        Esame::new(studenti[1].matricola.clone(), TipoEsame::Informatica),
    ];
    inserisci_studenti(&mut studenti);
    inserisci_esami(&mut esami);
    visualizza_studenti(studenti);
    visualizza_esami(esami);
}
fn inserisci_studenti(studenti:&mut [Studente; 3]){
    studenti.iter_mut().for_each(|s|{
        println!("Inserire nome e cognome per: {:?}", s);
        println!("Inserire nome");
        io::stdin()
            .read_line(&mut s.nome)
            .expect("Non sono riuscito a prendere in input il nome");
        s.nome=s.nome.trim().to_string();
        println!("Inserire cognome");
        io::stdin()
            .read_line(&mut s.cognome)
            .expect("Non sono riuscito a prendere in input il cognome");
        s.cognome=s.nome.trim().to_string();
    });
}
fn inserisci_esami(esami:&mut [Esame; 7]){
    esami.iter_mut().for_each(|e|{
        println!("Inserire voto per: {:?}", e);
        loop {
            let mut input:String=String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Non sono riuscito a prendere in input il voto");
            match input.trim().parse::<i32>() {
                Ok(num)=>{
                    e.voto=num;
                    break;
                },
                Err(_) => {
                    println!("Input non valido. Inserisci un numero intero.");
                },
            }
        }
    });
}
fn visualizza_studenti(studenti:[Studente; 3]){
    studenti.iter().for_each(|s| println!("{:?}",s));
}
fn visualizza_esami(esami:[Esame;7]){
    esami.iter().for_each(|e| println!("{:?}",e));
}

// kc2_3
// Scrivere un programma di assegnamento posti. (capienza massima vagone: 50 posti, 20 prima classe 30 seconda classe. Totale vagoni 3.)
//     Visualizzare le seguenti opzioni di prenotazione:
//     1 - per la prima classe
//     2 - per la seconda classe
//     Dopo la pronatozione visualizzare una schematica piantina con il posto assegnato.
//     I vagoni vanno occupati progressivamente.
//     Quando la sezione prima classe (per tutti i vagoni) è terminata chiedere al passeggero se vuole passare in
//     seconda e viceversa.
//     Se i posti sono tutti occupati visualizzare il rispettivo messaggio.
pub fn il_treno(){
    let mut treno:[Vagone; 3]=Default::default();
    loop{
        treno.iter().for_each(|v| v.stampa_vagone());
        println!("Vuoi prenotare un posto?s/n");
        let mut input="".to_string();
        io::stdin()
            .read_line(&mut input).expect("input non valido");
        if !(input.trim().eq("s") || input.trim().eq("S")){
            break;
        }
        input.clear();
        println!("In che classe vuoi prenotare?P=Prima Classe/S=Seconda Classe");
        let classe:Classe;
        loop{
            match io::stdin().read_line(&mut input){
                Ok(_) => if input.trim().eq("P") || input.trim().eq("p") || input.trim().eq("S") || input.trim().eq("s"){
                    if  input.trim().eq("S") || input.trim().eq("s"){
                        classe=Classe::SecondaClasse;
                    }else{
                        classe=Classe::PrimaClasse;
                    }
                    break;
                }else{
                    print!("Gli unici input validi sono: P\tp\tS\ts");
                },
                Err(_) => panic!("input non valido"),
            }
        }
        input.clear();
        let res:bool=treno.iter().all(|v| v.has_posto_libero(&classe));
        if res {//caso treno con posto libero
            prenota_posto_treno(&mut treno,classe);
        }else{
            println!("La tua classe è occupata. Vuoi prenotare un posto nell'altra?s/n");
            io::stdin()
                .read_line(&mut input).expect("input non valido");
            if input.trim().eq("s") || input.trim().eq("S"){
                let classe2;
                match classe{
                    Classe::PrimaClasse => classe2=Classe::SecondaClasse,
                    Classe::SecondaClasse => classe2=Classe::PrimaClasse,
                }
                prenota_posto_treno(&mut treno,classe2);
            }
        }
    }
}
fn prenota_posto_treno(treno: &mut [Vagone; 3], classe: Classe) {
    for vagone in treno.iter_mut() {
        if let Some(posto) = vagone.wich_posto_libero(&classe) {
            vagone.prenota_posto_libero(&posto);
            println!("Hai prenotato il posto: {:?}", posto);
            return;
        }
    }
    println!("Non ci sono posti liberi nella classe richiesta.");
}