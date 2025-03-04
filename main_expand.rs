#![feature(prelude_import)]
#![no_std]
// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
// Ending ta_gueule_le_compilo


use RMesure::RMesure;
//use ::RMesure::RMesure_MAX;


fn sg_square(periode: f32, kTe: f32) -> RMesure {
    if (kTe % periode) < (periode / 2.0) {
            RMesure::loi(12.00, 0.01, 'R')
        } else { RMesure::loi(0.00, 0.01, 'R') }
}

fn test_incertitude_filtrage_prem_ordre() {
    let periode_square: f32 = 4.0_f32;

    let Meas: RMesure = sg_square(periode_square, 0.0);

    let mut Out_eq1: RMesure = RMesure::loi(00.00, 0.01, 'R');
    let mut Out_eq2: RMesure = Out_eq1.clone();
    let mut Out_eq1_zm1: RMesure = Out_eq1.clone();
    let mut Out_eq2_zm1: RMesure = Out_eq2.clone();

    let Te = 0.04_f32; // 40ms
    let Tf = 1.0_f32; // 1sec
    let fac: RMesure =
        RMesure::from(1.0_f32 - (-2.0_f32 * 3.14159_f32 * Te / Tf).exp());







    // equation 1
    // equation 2






    // test_incertitude_filtrage_prem_ordre();






    { ::std::io::_print(format_args!("Out = {0}\n", Out_eq1)); };
    { ::std::io::_print(format_args!("Meas = {0}\n", Meas)); };
    { ::std::io::_print(format_args!("fac = {0}\n", fac)); };
    {
        ::std::io::_print(format_args!("Out[init].Uc() = {0}\n",
                Out_eq1.Eps()));
    };
    {
        ::std::io::_print(format_args!("Out[init].Uc() = {0}\n",
                Out_eq2.Eps()));
    };
    for NumIteration in 0..100 {
        let Meas: RMesure =
            sg_square(periode_square,
                <i16 as Into<f32>>::into(NumIteration) * Te);
        let Uc_gauche_eq1: f32 =
            ((1.0_f32 - fac.clone()) * Out_eq1_zm1.clone()).Eps();
        let Uc_gauche_eq2: f32 = Out_eq2_zm1.clone().Eps();
        let Uc_droite_eq1: f32 = (fac.clone() * Meas.clone()).Eps();
        let Uc_droite_eq2: f32 =
            (fac.clone() * (Meas.clone() - Out_eq2_zm1.clone())).Eps();
        Out_eq1 =
            ((1.0_f32 - fac.clone()) * Out_eq1_zm1.clone()) +
                (fac.clone() * Meas.clone());
        Out_eq2 =
            (Out_eq2_zm1.clone()) +
                (fac.clone() * (Meas.clone() - Out_eq2_zm1.clone()));
        let Uc_finale_eq1 = Out_eq1.Eps();
        let Uc_finale_eq2 = Out_eq2.Eps();
        {
            ::std::io::_print(format_args!("[{0}]\t{1}\t{2}\t-->  {3}\t||\t{4}\n",
                    NumIteration, Uc_gauche_eq1, Uc_droite_eq1, Uc_finale_eq1,
                    Out_eq1_zm1));
        };
        {
            ::std::io::_print(format_args!("[{0}]\t{1}\t{2}\t-->  {3}\t||\t{4}\n",
                    NumIteration, Uc_gauche_eq2, Uc_droite_eq2, Uc_finale_eq2,
                    Out_eq2_zm1));
        };
        { ::std::io::_print(format_args!("\n")); };
        Out_eq1_zm1 = Out_eq1.clone();
        Out_eq2_zm1 = Out_eq2.clone();
    }
}
fn main() {
    let fc: RMesure = RMesure::scalaire(250_000_f32);
    let Ct: RMesure = RMesure::loi(0.000_000_001_f32, 20.0_f32, 'P');
    let Rt: RMesure = 1.0_f32 / (fc.clone() * Ct.clone());
    { ::std::io::_print(format_args!("fc = {0}\n", fc)); };
    { ::std::io::_print(format_args!("Ct = {0}\n", Ct)); };
    { ::std::io::_print(format_args!("Rt = {0}\n", Rt)); };
    let Rt: RMesure = RMesure::loi(Rt.Val(), 5.0, 'P');
    { ::std::io::_print(format_args!("Rt = {0}\n", Rt)); };
    let ft: RMesure = 1.0_f32 / (Rt.clone() * Ct.clone());
    { ::std::io::_print(format_args!("ft = {0}\n", ft)); };
    {
        ::std::io::_print(format_args!("Rt.log10() = {0}\n",
                Rt.clone().log10()));
    };
    {
        ::std::io::_print(format_args!("Rt.cbrt() = {0}\n",
                Rt.clone().cbrt()));
    };
}
