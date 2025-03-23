use rand::Rng;
use std::{i32, io};
use crate::moduli::utilities::utilities_cordo::SIZE;
use crate::moduli::allenamento_prove::classes::studente_voto_kc2_1::StudenteVoto;

use super::classes::studente_kc2_2::Studente;
use super::classes::esame_kc2_2::{Esame, TipoEsame};

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