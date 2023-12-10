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
    let AutreMesure: RMesure = RMesure::new(0.618, 0.001, 95.45);
    let MesureLegal: RMesure = RMesure::loi(0.382, 0.001, 'R');


    println!("( {} +/- {} | {}% )", MesureLegal.Val(), MesureLegal.IT(), MesureLegal.Alpha());
    
}
