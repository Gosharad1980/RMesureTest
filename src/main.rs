// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo

use RMesure::RMesure;


fn main()
{
    let AutreMesure: RMesure = RMesure::loi(12.00, 0.01, 'R');
    let MesureLegal: RMesure = RMesure::loi(2.000, 0.001, 'R');

    let MesureLabo = AutreMesure + MesureLegal;
    let MesureOfset = AutreMesure + 2.0;
    let MesureAdd = 2.0 + MesureLegal;


    println!("MesureLabo = {MesureLabo}");
    println!("MesureOfset = {MesureOfset}");
    println!("MesureAdd = {MesureAdd}"); 


    let MesureLabo = AutreMesure - MesureLegal;
    let MesureOfset = AutreMesure - 2.0;
    let MesureAdd = 2.0 - MesureLegal;


    println!("MesureLabo = {MesureLabo}");
    println!("MesureOfset = {MesureOfset}");
    println!("MesureAdd = {MesureAdd}"); 
}
