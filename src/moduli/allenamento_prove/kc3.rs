use crate::moduli::utilities::utilities_cordo::read_num_positivo;

use super::classes::kc3_1::stampa_mese;

// kc3.1
// Creare un programma calendario. Il programma deve dare la possibilità di inserire il mese e l'anno attuale
// per mostrare le informazioni relative al mese evidenziando i giorni del mese e il giorno attuale.Usare le
// classi calendario dell'API.
pub fn calendario(){
    let mut mese :u32=12;
    let mut anno :u32=2002;
    
    mese = read_num_positivo();
    anno = read_num_positivo();
    stampa_mese(12, 2002);


}