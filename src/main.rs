// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo

use RMesure::RMesure;

fn main()
{
    let UneMesure : RMesure = RMesure { valeur : 0.618, epsilon : 0.001, alpha : 95.0 };
    println!("( {} +/- {} | {}% )", UneMesure.Val(), UneMesure.IT(), UneMesure.Alpha());
}
