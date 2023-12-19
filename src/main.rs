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
    let      In: RMesure = RMesure::loi(12.00, 0.01, 'R');
    let mut Out: RMesure = RMesure::loi(00.00, 0.01, 'R');

    let K: RMesure = RMesure::from(0.382);

    println!("Out = {Out}");

    for i in 0..100
    {
        Out += K.clone() * (In.clone() - Out.clone());
        println!("Out {i} = {Out}");
    }

}
