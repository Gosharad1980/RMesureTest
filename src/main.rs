// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo

use RMesure::RMesure;
//use ::RMesure::RMesure_MAX;

fn sg_square(periode: f64, kTe: f64) -> RMesure
{
    if (kTe % periode) < (periode/2.0) { return RMesure::loi(12.00,0.01, 'R') }
    else                               { return RMesure::loi(0.00,0.01, 'R')  }
}

fn main()
{
    let periode_square: f64 = 4.0_f64;
    
    let Meas: RMesure = sg_square(periode_square, 0.0);

    let mut Out_eq1: RMesure = RMesure::loi(00.00,0.01, 'R');
    let mut Out_eq2: RMesure = Out_eq1.clone();
    let mut Out_eq1_zm1: RMesure = Out_eq1.clone();
    let mut Out_eq2_zm1: RMesure = Out_eq2.clone();
    
    let Te = 0.04_f64; // 40ms
    let Tf = 1.0_f64; // 1sec
    let fac: RMesure = RMesure::from(1.0_f64 - (-2.0_f64 * 3.14159_f64 * Te/Tf).exp());


    println!("Out = {Out_eq1}");
    println!("Meas = {Meas}");
    println!("fac = {fac}");

    println!("Out[init].Uc() = {}", Out_eq1.Uc());
    println!("Out[init].Uc() = {}", Out_eq2.Uc());

    for k in 0..25
    {
        let Meas: RMesure = sg_square(periode_square, <i32 as Into<f64>>::into(k) * Te);

        let Uc_gauche_eq1: f64 = ((1.0_f64 - fac.clone()) * Out_eq1_zm1.clone()).Uc();
        let Uc_gauche_eq2: f64 =                            Out_eq2_zm1.clone().Uc();

        let Uc_droite_eq1: f64 = (fac.clone() *  Meas.clone()                       ).Uc();
        let Uc_droite_eq2: f64 = (fac.clone() * (Meas.clone() - Out_eq2_zm1.clone())).Uc();

        Out_eq1 = ((1.0_f64 - fac.clone()) * Out_eq1_zm1.clone()) + (fac.clone() *  Meas.clone()                       ); // equation 1
        Out_eq2 = (                          Out_eq2_zm1.clone()) + (fac.clone() * (Meas.clone() - Out_eq2_zm1.clone())); // equation 2

        let Uc_finale_eq1 = Out_eq1.Uc();
        let Uc_finale_eq2 = Out_eq2.Uc(); 
        

        println!("[{k}]\t{Uc_gauche_eq1}\t{Uc_droite_eq1}\t-->  {Uc_finale_eq1}\t||\t{Out_eq1_zm1}");
        println!("[{k}]\t{Uc_gauche_eq2}\t{Uc_droite_eq2}\t-->  {Uc_finale_eq2}\t||\t{Out_eq2_zm1}");
        println!();

        Out_eq1_zm1 = Out_eq1.clone();
        Out_eq2_zm1 = Out_eq2.clone();
    }

    let R1: RMesure = RMesure::loi(1000.0, 1000.0 * 5.0 / 100.0, 'C');
    let R2: RMesure = RMesure::loi(1000.0, 1000.0 * 5.0 / 100.0, 'C');

    let Gain: RMesure = 1.0_f64 + (R2 / R1);

    println!("Gain = {Gain}");
}