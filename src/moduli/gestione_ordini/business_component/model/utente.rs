pub struct Utente{
    pub username: String,
    pub nome: String,
    pub cognome: String,
    pub eta: i32,
    pub totale:i32,
}
impl Utente{
    pub fn new()->Utente{
        Utente{
            username : String::new(),
            nome: String::new(),
            cognome: String::new(),
            eta: 0,
            totale:0,
        }
    }
}
