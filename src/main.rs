// Entering ta_gueule_le_compilo
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
// Ending ta_gueule_le_compilo


use RMesure::RMesure as RMesure;

//use ::RMesure::RMesure_MAX;


fn sg_square(periode: f64, kTe: f64) -> RMesure
{
	if (kTe % periode) < (periode/2.0) { RMesure::loi(12.00,0.01, 'R') }
	else                               { RMesure::loi(0.00,0.01, 'R')  }
}

fn test_incertitude_filtrage_prem_ordre()
{
	let periode_square: f64 = 4.0_f64;

	let mut Meas: RMesure = sg_square(periode_square, 0.0);

	let mut Out_eq1: RMesure = RMesure::loi(00.00,0.01, 'R');
	let mut Out_eq2: RMesure = Out_eq1;
	let mut Out_eq1_zm1: RMesure = Out_eq1;
	let mut Out_eq2_zm1: RMesure = Out_eq2;

	let Te = 0.04_f64; // 40ms
	let Tf = 1.0_f64; // 1sec
	let fac: RMesure = RMesure::from(1.0_f64 - (-2.0_f64 * 3.14159_f64 * Te/Tf).exp());


	println!("Out = {Out_eq1}");
	println!("Meas = {Meas}");
	println!("fac = {fac}");

	println!("Out[init].Uc() = {}", Out_eq1.Eps());
	println!("Out[init].Uc() = {}", Out_eq2.Eps());

	println!();
	println!();
	println!("======= -->     ======================================  =====================================================");
	println!("[i]     -->     (Mesure +/- IT               | 95.45%)  (Out filtre        +/- IT                   | 95.45%)");
	println!("======= -->     ======================================  =====================================================");
	println!();

	for NumIteration in 0..100
	{
		Meas = sg_square(periode_square, <i16 as Into<f64>>::into(NumIteration) * Te);
		
		// version distribuée --> incertitude de Out converge
		Out_eq1 = ((1.0_f64 - fac) * Out_eq1_zm1) + (fac *  Meas);

		// version factorisée --> incertitude de Out diverge
		Out_eq2 = (Out_eq2_zm1) + (fac * (Meas - Out_eq2_zm1));


		println!("[{NumIteration}]\t-->\t{Meas}\t{Out_eq1_zm1}");
		println!("[{NumIteration}]\t-->\t{Meas}\t{Out_eq2_zm1}");
		println!();

		Out_eq1_zm1 = Out_eq1;
		Out_eq2_zm1 = Out_eq2;
	}
}

fn test_incertitude_U_egal_RI()
{
	let fc: RMesure = 250_000_f64.into();
	let Ct: RMesure = RMesure::loi(0.000_000_001_f64, 10.0_f64,'P');


	let Rt: RMesure = 1.0_f64 / (fc * Ct);


	println!("fc = {fc}");
	println!("Ct = {Ct}");
	println!("Rt = {Rt}");

	let R1: RMesure = RMesure::loi(Rt.Val(), 5.0,'P');
	println!("R1 = {R1}");

	let ft: RMesure = 1.0_f64 / (R1 * Ct);
	println!("ft = {ft}");
}

fn main()
{
	//test_incertitude_filtrage_prem_ordre();
	test_incertitude_U_egal_RI();
}