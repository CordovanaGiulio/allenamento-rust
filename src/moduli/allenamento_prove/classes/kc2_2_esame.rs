#[derive (Debug)]
pub struct Esame{
    #[allow(dead_code)]
    matricola:String,
    #[allow(dead_code)]
    esame:TipoEsame,
    pub voto:i32,
}
#[derive (Debug)]
pub enum TipoEsame{
    Analisi,
    Informatica,
    Chimica
}
impl Esame{
    pub fn new(matricola:String, esame :TipoEsame)->Esame{
        Esame{
            matricola,
            esame,
            voto:0,
        }
    }
}