// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo

use RMesure::RMesure;
//use ::RMesure::RMESURE_MAX;


fn main()
{
    let     Mes: RMesure = RMesure::loi(12.00, 0.01, 'R');
    let     Mdl: RMesure = RMesure::loi(00.00,12.0, 'R');
    let mut Out: RMesure = RMesure::from(0.0);
    
    let K: f64 = 0.382;
    let mut prop: f64 = 0.0;

    println!("Out = {Out}");

    for i in 0..100
    {
        
        Out = prop * Mes.clone() + (1.0 - prop) * Mdl.clone();
        println!("Out[{i}][{prop}] = {Out} --> {}", Out.clone() == Mes.clone());
        prop += K * (1.0 - prop);
    }
}