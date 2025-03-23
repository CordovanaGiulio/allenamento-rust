use rand::Rng;

static mut LISTA_MATRICOLE: Vec<String>=Vec::new();

#[derive (Debug)]
pub struct Studente{
    pub matricola:String,
    pub nome:String,
    pub cognome:String,
}
impl Studente{
    pub fn new()->Studente{
        Studente{
            matricola:genera_matricola(),
            nome:"".to_string(),
            cognome:"".to_string(),
        }
    }
}
impl Default for Studente {
    fn default() -> Self {
        Studente::new()
    }
}
fn genera_matricola()->String{
    let mut rand = rand::rng();
    let mut matricola: String= String::new();
    loop {
        for _ in 0..10{
            let num: u32= rand.random_range(0..9);
            matricola.push(char::from_digit(num,10).unwrap());
        }
        if is_valid_matricola(&matricola){
            unsafe {LISTA_MATRICOLE.push(matricola.clone())};
            return matricola.to_string();
        }
    }
}
fn is_valid_matricola(matricola:&String)->bool{
    return !unsafe { LISTA_MATRICOLE.contains(matricola) }
}