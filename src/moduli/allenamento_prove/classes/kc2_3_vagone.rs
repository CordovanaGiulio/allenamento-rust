use super::kc2_3_posto::{Classe, Posto};
#[derive (Debug)]
pub struct Vagone {
    seconda_classe: [Posto; 30],
    prima_classe: [Posto; 20]
}
impl Vagone {
    //inizializzazione
    pub fn new()->Vagone{
        Vagone {
            seconda_classe: Self::new_seconda_classe(),
            prima_classe: Self::new_prima_classe(),
        }
    }
    fn new_seconda_classe()->[Posto; 30]{
        let mut seconda_classe: [Posto; 30]=Default::default();
        for i in 0..seconda_classe.len() {
            seconda_classe[i] = Posto{
                classe: Classe::SecondaClasse,
                numero_posizione: i as i32,
                occupato: false,
            }
        }
        return seconda_classe;
    }
    fn new_prima_classe()->[Posto; 20]{
        let mut prima_classe: [Posto; 20]=Default::default();
        for i in 0..prima_classe.len() {
            prima_classe[i] = Posto{
                classe: Classe::PrimaClasse,
                numero_posizione: i as i32,
                occupato: false,
            }
        }
        return prima_classe;
    }
    //stampa
    pub fn stampa_vagone(&self){
        self.stampa_prima_classe();
        self.stampa_seconda_classe();
    }
    pub fn stampa_seconda_classe(&self){
        self.stampa_classe(&self.seconda_classe);
    }
    pub fn stampa_prima_classe(&self){
        self.stampa_classe(&self.prima_classe);
    }
    fn stampa_classe(&self, posto: &[Posto]){
        let mut num=1;
        println!("------------------------------------------------------{:?}",posto[0].classe);
        for i in posto.iter(){
            if (num-1)%5==0 {
                print!("|\t");
            }
            match i.occupato{
                true => print!("X\t"),
                false => print!("O\t"),
            }
            if num%5==0 {
                println!("|\t");
            }
            num+=1;
        }
    }
    //prenotazione e controllo posto
    pub fn has_posto_libero(&self, classe:&Classe)->bool{
        match classe {
            Classe::PrimaClasse => {
                return self.prima_classe.iter().any(|p| p.occupato==false);
            },
            Classe::SecondaClasse => {
                return self.seconda_classe.iter().any(|p| p.occupato==false);
            }
        }
    }
    pub fn wich_posto_libero(&self, classe:&Classe)->Option<Posto>{
        match classe {
            Classe::PrimaClasse => {
                return self.prima_classe.iter().find(|p| p.occupato==false).cloned();
            },
            Classe::SecondaClasse => {
                return self.seconda_classe.iter().find(|p| p.occupato==false).cloned();
            }
        }
    }
    pub fn prenota_posto_libero(&mut self,posto: &Posto){
        match posto.classe {
            Classe::PrimaClasse => {
                if let Some(posto_array) = self.prima_classe.iter_mut().find(|p| p.numero_posizione==posto.numero_posizione) {
                    posto_array.occupato=true;
                }else {
                    println!("Posizione non trovata.");
                };
            },
            Classe::SecondaClasse => {
                if let Some(posto_array) = self.seconda_classe.iter_mut().find(|p| p.numero_posizione==posto.numero_posizione) {
                    posto_array.occupato=true;
                }else {
                    println!("Posizione non trovata.");
                };
            },
        }
    }
}
impl Default for Vagone{
    fn default() -> Self {
        Vagone::new()
    }
}