// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo

use RMesure::RMesure;


fn main()
{
    let U1: RMesure = RMesure::loi(12.05, 0.01, 'R');
    let U2: RMesure = RMesure::loi(12.10, 0.01, 'R');
    let U3: RMesure = RMesure::loi(11.93, 0.01, 'R');

    let moyenne = (U1 + U2 + U3) / 3.0;

    println!("moyenne = {moyenne}");
}
