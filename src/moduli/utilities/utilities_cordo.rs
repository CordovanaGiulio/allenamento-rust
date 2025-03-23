pub const SIZE: usize = 9;
// pub const ALFABETO: [char; 62]=['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','1','2','3','4','5','6','7','8','9','0'];
// use rand::Rng;

// pub fn genera_stringa()->[char; SIZE]{
//     let mut rand = rand::rng();
//     let mut res:[char; SIZE]=['a';SIZE];
//     for i in 0..SIZE{
//         res[i]=ALFABETO[rand.random_range(0..62)];
//     }
//     return res;
// }
// pub fn genera_array_i32(range:i32)->[i32; SIZE]{
//     let mut rand = rand::rng();
//     let mut vec=[0; SIZE];
//     for i in 0..SIZE{
//         vec[i]=rand.random_range(0..range);
//     }
//     return vec;
// }
// pub fn print_arr<T: std::fmt::Display>(vec: [T; SIZE]) {
//     for i in 0..SIZE {
//         print!("{}", vec[i])
//     }
//     print!("\n");
// }