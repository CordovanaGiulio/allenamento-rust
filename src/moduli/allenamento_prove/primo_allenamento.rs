// use crate::moduli::utilities::utilities_cordo as Utilies;
use crate::moduli::allenamento_prove::kc1 as kc1;
use crate::moduli::allenamento_prove::kc2 as kc2;
use crate::moduli::allenamento_prove::kc3 as kc3;
pub fn prove(){
    kc1::stampa_primi();
    println!("\n------------------------------------------------------------------------------------------------------------------------------");
    kc1::calcola_mcd();
    println!("\n------------------------------------------------------------------------------------------------------------------------------");
    kc1::date_100();
    println!("\n------------------------------------------------------------------------------------------------------------------------------");
    kc1::gioco_dadi();
    println!("\n------------------------------------------------------------------------------------------------------------------------------");
    kc2::studenti_voti_matrice();
    println!("\n------------------------------------------------------------------------------------------------------------------------------");
    // kc2::matricole();
    // println!("\n------------------------------------------------------------------------------------------------------------------------------");
    // kc2::il_treno();
    // println!("\n------------------------------------------------------------------------------------------------------------------------------");
    kc3::calendario();
    println!("\n------------------------------------------------------------------------------------------------------------------------------");
}


// fn cambia_riferimento_mut(s: &mut String){
//     let mut chr:Vec<char>= s.chars().collect();
//     chr[3]='a';
//     *s=chr.iter().collect();
// }

// fn stampa_riferimento_immut(s :&String){
//     print!("{}", s)
// }

// fn stampa_stringa_heap (stringa1 : String)-> String{
//     print!("{}",stringa1);
//     return stringa1;
// }

// fn stampa_quadrato(num : i32){
//     for colonna in -num+1..num {
//         for riga in -num+1..num {
//             if riga.abs()>colonna.abs(){
//                 print!("{}\t",riga.abs()+1);
//             }else {
//                 print!("{}\t",colonna.abs()+1);
//             }
//         }
//         println!();
//     }
// }

// fn gradi_lettere(str: [char; Utilies::SIZE]){
//     let mut min_g: i32=str.len() as i32;
//     let mut max_g: i32=0;
//     let mut conteggio:i32;
//     for &i in str.iter(){
//         conteggio =conta_lettere(str, i);
//         if conteggio<min_g{
//             min_g=conteggio;
//         }else if conteggio>max_g{
//             max_g=conteggio;
//         }
//     }
//     println!("max_g: {}, min_g: {}", max_g, min_g);
// }

// fn conta_lettere(str: [char; Utilies::SIZE], lettera:char)->i32{
//     let mut res=0;
//     for &i in str.iter(){
//         if i==lettera{
//             res+=1;
//         }
//     }
//     return res;
// }

// fn is_palindroma(str1: [char; Utilies::SIZE])->bool{
//     let len=Utilies::SIZE/2;
//     for i in 0..len{
//         if str1[i]!=str1[Utilies::SIZE-i-1] {
//             return false;
//         }
//     }
//     return true;
// }

// fn concatena_vec_char(str1: [char; Utilies::SIZE],str2: [char; Utilies::SIZE])->[char; Utilies::SIZE*2]{
//     let mut concat: [char; Utilies::SIZE*2]= ['a'; Utilies::SIZE*2];
//     for i in 0..Utilies::SIZE{
//         concat[i]=str1[i];
//     }
//     for i in Utilies::SIZE..Utilies::SIZE*2{
//         concat[i]=str2[i-Utilies::SIZE];
//     }
//     return concat;
// }

// fn ordina_array(vec_original:[i32; Utilies::SIZE])->[i32; Utilies::SIZE]{
//     let len=vec_original.len();
//     let mut vec=vec_original.clone();
//     let mut temp;
//     for i in 0..len {
//         for j in i..len{
//             if vec[i]>vec[j]{
//                 temp=vec[i];
//                 vec[i]=vec[j];
//                 vec[j]=temp;
//             }
//         }
//     }
//     return vec;
// }
