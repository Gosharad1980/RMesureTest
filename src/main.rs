// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo

use std::borrow::{Borrow, BorrowMut};
use RMesure::RMesure;


fn main()
{
    let AutreMesure: RMesure = RMesure::loi(12.00, 0.01, 'R');
    let MesureLegal: RMesure = RMesure::loi(2.000, 0.001, 'R');

    //let MesureLabo = AutreMesure.clone() + MesureLegal.clone();

    let MesureLabo = AutreMesure.borrow() + MesureLegal.borrow();
    let MesureOfset = AutreMesure.clone() + 2.0;
    let MesureAdd = 2.0 + MesureLegal.clone();


    
    println!("( {} +/- {} | {}% )", MesureOfset.Val(), MesureOfset.IT(), MesureOfset.Alpha());
    println!("( {} +/- {} | {}% )", MesureLabo.Val(),  MesureLabo.IT(),  MesureLabo.Alpha() ); 
    println!("( {} +/- {} | {}% )", MesureAdd.Val(),   MesureAdd.IT(),   MesureAdd.Alpha()  ); 
}
