fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let _n:f64 = 3.0;
	let d:f64 = 1.0 - (r / 100.0);
	let a:f64 = d * (3.0);
	let _s:f64 = p * a;
	println!("depreciation is {}", d);  
}