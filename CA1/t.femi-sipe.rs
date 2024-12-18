use std::io;
fn main() {
	println!("\nClients Sibling Data");
	let mut number_of_siblings = String::new();
	let mut first_name of sibling = String::new();
	let mut age_of_sibling = String::new();
	let mut gender_of_sibling = String::new();
	let mut country_of_residence = String::new();
	let mut marital_status = String::new();
	let mut number_of_children = String::new();
	let mut name_of_child = String::new();
	let mut age_of_child = String::new();
	let mut school_or_daycare_name = String::new();
	let mut do_you_have_children = String::new();
	
	println!("\nNumber of siblings");
	io::stdin().read_line(&mut number_of_siblings).expect("Not a valid string");
	let number of siblings:f64 = number_of_siblings.trim().parse().expect("Not a valid number");
	if number of siblings > 1.0;
	loop{
		println!("Enter first name of sibling");
		io::stdin().read_line(&mut first_name).expect("Not a valid string");

		println!("\n Enter age of sibling ");
		io::stdin().read_line(&mut age_of_sibling).expect("Not a valid string");
		let age of sibling:f64 = age_of_sibling.trim().parse().expect("Not a valid number");
		if age of sibling >=18{
			println!("Is the sibling married or single or in a relationship?");
			io::stdin().read_line(&mut marital_status).expect("Please enter his/her marital,relationship or single status");
			let marital_status = marital_status.trim();
		}if marital_status == "married"{
			println!("Do you have children? yes or no");
			io::stdin().read_line(&mut do_you_have_children).expect(Not a valid string);
			if do_you_have_children == yes{
			println!("Enter name of child ");
			io::stdin().read_line(&mut name_of_child).expect("Not a valid string");

			println!("Enter age of child ");
			io::stdin().read_line(&mut age_of_child).expect("Not a valid string");
			let age_of_child:f64 = age_of_child.trim().parse().expect("Not a valid number");

			println!("Enter school or daycare name");
			io::stdin().read_line(&mut school_or_daycare_name).expect(Not a valid string);}


		}


		println!("Enter gender of sibling ");
		io::stdin().read_line(&mut gender_of_sibling).expect("Not a valid string");

		println!("Enter country of residence");
		io::stdin().read_line(&mut country_of_residence).expect("Not a valid string");

	}
	
	


}