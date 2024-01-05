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
    let     Mes: RMesure = RMesure::loi(12.00,0.01, 'R');
    //let mut Out: RMesure = RMesure::loi(00.00,12.0, 'R');
    let mut Out: RMesure = RMesure::from(0.0);
    let mut Out_zm1: RMesure = Out.clone();
    
    let Te = 0.04_f64; // 40ms
    let Tf = 1.0_f64; // 1sec
    let K: f64 = 1.0_f64 - (-2.0_f64 * 3.14159_f64 * Te/Tf).exp();

    println!("Out = {Out}");
    println!("Mes = {Mes}");
    println!("K   = {K}");

    for i in 0..33
    {
        Out = K * Mes.clone() + (1.0_f64 - K) * Out_zm1.clone();
        println!("Out[{i}] = {Out} --> {}", Out.clone() == Mes.clone());
        Out_zm1 = Out.clone();
    }
}