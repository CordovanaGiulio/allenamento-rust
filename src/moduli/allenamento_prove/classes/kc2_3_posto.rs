#[derive (Debug, Clone)]
pub struct Posto{
    pub classe: Classe,
    pub numero_posizione: i32,
    pub occupato: bool,
}
impl Posto{
    pub fn new()->Posto{
        Posto { 
            classe: Classe::PrimaClasse, 
            numero_posizione: -1,
            occupato: false
        }
    } 
}
impl Default for Posto {
    fn default() -> Self {
        Posto::new()
    }
}
#[derive (Debug, Clone)]
pub enum Classe{
    PrimaClasse,
    SecondaClasse,
}
