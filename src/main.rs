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
    let mut U: RMesure = RMesure::loi(12.0, 0.01, 'R');
    let R: RMesure = RMesure::loi(1000.0, 1000.0*0.05, 'N');
    let mut I: RMesure = RMesure::zero();

    let a: RMesure = RMesure::loi(0.5, 0.1, 'N');
    let b: RMesure = RMesure::loi(1.0, 0.1, 'N');
    let c: RMesure = RMesure::loi(1.1, 0.1, 'N');
    let d: RMesure = RMesure::loi(1.5, 0.1, 'N');

    U += RMesure::scalaire(2.0);

    I = U / R;

    println!("-U = {}", -U);
    println!("R = {R}");
    println!("I = {I}");

    println!("BoxPlot I = {:?}", I.BoxPlot());


    println!("\n");
    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");
    println!("d = {d}");

    println!("");


    println!("a == d [ FAUX ] => {}", a==d); // significativement diff�rents
	println!("b == c [ VRAI ] => {}", b==c); // insignificativement diff�rents
    println!("");

	println!("a != d [ VRAI ] => {}", a!=d); // significativement diff�rents
	println!("b != c [ FAUX ] => {}", b!=c); // insignificativement diff�rents
    println!("");

	println!("a < d  [ VRAI ] => {}", a<d); // a.valeur < d.valeur et a!=d
	println!("d < a  [ FAUX ] => {}", d<a); // d.valeur > a.valeur et d!=a
	println!("c < b  [ FAUX ] => {}", c<b); // insignificativement diff�rents
	println!("b < c  [ FAUX ] => {}", b<c); // insignificativement diff�rents
    println!("");

	println!("a > d  [ FAUX ] => {}", a>d); // a.valeur < d.valeur et a!=d
	println!("d > a  [ VRAI ] => {}", d>a); // d.valeur > a.valeur et d!=a
	println!("c > b  [ FAUX ] => {}", c>b); // insignificativement diff�rents
	println!("b > c  [ FAUX ] => {}", b>c); // insignificativement diff�rents
    println!("");

	println!("a <= d [ VRAI ] => {}", a<=d); // a.valeur < d.valeur et a!=d
	println!("d <= a [ FAUX ] => {}", d<=a); // d.valeur > a.valeur et d!=a
	println!("c <= b [ VRAI ] => {}", c<=b); // insignificativement diff�rents
	println!("b <= c [ VRAI ] => {}", b<=c); // insignificativement diff�rents
    println!("");

	println!("a >= d [ FAUX ] => {}", a>=d); // a.valeur < d.valeur et a!=d
	println!("d >= a [ VRAI ] => {}", d>=a); // d.valeur > a.valeur et d!=a
	println!("c >= b [ VRAI ] => {}", c>=b); // insignificativement diff�rents
	println!("b >= c [ VRAI ] => {}", b>=c); // insignificativement diff�rents

}
