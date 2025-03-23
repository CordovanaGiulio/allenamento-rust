
#[derive (Debug, Clone)]
pub enum StudenteVoto{
    Studente(String),
    Voto(i32)
}
impl StudenteVoto {
    pub fn get_voto(self)->Result<i32, String>{
        match self {
            StudenteVoto::Voto(voto) => {
                return Ok(voto);
            },
            StudenteVoto::Studente(nome) => {
                return Err(nome);
            }
        }
    }
}