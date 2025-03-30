mod moduli;
use moduli::allenamento_prove::primo_allenamento as PrimoAll;

fn main() {
    PrimoAll::prove();
}

//ESERCIZIO 1 - prende array e inverte
// let mut array_primo_esercizio: [i8; 10]=[1;10];
// for i in 0..9 {
//     array_primo_esercizio[i]=random_generator.gen();
//     array_primo_esercizio[i]=array_primo_esercizio[i].abs();
// }
// println!("{:?}", array_primo_esercizio);
// let mut max_primo_esercizio=array_primo_esercizio[0];
// let mut min=array_primo_esercizio[0];
// let mut tot:i32=0;
// for &item in array_primo_esercizio.iter() {
//     if item>max_primo_esercizio{
//         max_primo_esercizio=item;
//     }
//     if item<min{
//         min = item;
//     }
//     tot+=item as i32;
// }
// tot/=array_primo_esercizio.len() as i32;
// print!("min: {}", min);
// print!("\nmax: {}", max_primo_esercizio);
// print!("\nmedia: {}", tot);
// println!("\n------------------------------------------------------------------------------------------------------------------------------");

//ESERCIZIO 2- Crea un array di 10 numeri casuali positivi, calcola e stampa min, max e media degli elementi.
// let mut array_secondo_esercizio=vec![0; 10];
// for i in 0..10 {
//     array_secondo_esercizio[i] = i;
// }
// println!("{:?}", array_secondo_esercizio);
// let mut len=array_secondo_esercizio.len()-1;
// for i in 0..len/2{
//     array_secondo_esercizio[i]+=array_secondo_esercizio[len-i];
//     array_secondo_esercizio[len-i]=array_secondo_esercizio[i]-array_secondo_esercizio[len-i];
//     array_secondo_esercizio[i]-=array_secondo_esercizio[len-i];
// }
// println!("{:?}", array_secondo_esercizio);
// println!("------------------------------------------------------------------------------------------------------------------------------");

//ESERCIZIO 3- Crea un array di 10 numeri casuali tra 1 e 9, quindi copia gli elementi maggiori o uguali a un numero inserito dall'utente in un altro array.
// let mut array_terzo_esercizio1=vec![0; 10];
// len = array_terzo_esercizio1.len();
// let mut array_terzo_esercizio2=vec![0; len];
// let numero_inserito_utente: i32 = 6;
// for i in 0..len{
//     array_terzo_esercizio1[i]=random_generator.gen_range(1..10);
// }
// let mut j =0;
// for i in 0..len{
//     if array_terzo_esercizio1[i]>=numero_inserito_utente{
//         array_terzo_esercizio2[j]=array_terzo_esercizio1[i];
//         j += 1;
//     }
// }
// println!("{:?}", array_terzo_esercizio1);
// println!("{:?}", array_terzo_esercizio2);
// println!("------------------------------------------------------------------------------------------------------------------------------");

//ESERCIZIO 4 - // Crea un array di 10 numeri casuali tra 0 e 9, trova il massimo, e stampa una rappresentazione grafica in formato "bar chart" per ogni valore dell'array.
// let mut array_quarto_esercizio = Vec::new();
// let mut max: i32=0;
// let mut len=10;
// for _ in 0..len{
//     array_quarto_esercizio.push(random_generator.gen_range(0..10));
// }
// for &item in array_quarto_esercizio.iter() {
//     if item>max{
//         max=item;
//     }
// }
// let max_const=max as usize;
// for _ in 0..max_const{
//     max -= 1;
//     for i in 0..len{
//         if array_quarto_esercizio[i]>max {
//             print!("*\t");
//         }else{
//             print!("\t");
//         }
//     }
//     println!();
// }
// for &i in array_quarto_esercizio.iter(){
//     print!("{}\t",i);
// }
// println!("\n------------------------------------------------------------------------------------------------------------------------------");