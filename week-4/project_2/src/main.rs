use std::io;
fn main() {
    println!("Employee Annual Incentive");
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the experience of employee in years");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the age of employee in years");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f64 = input2.trim().parse().expect("Not a valid number");

    let incentive1 = 1560000;
    let incentive2 = 1480000;
    let incentive3 = 1300000;
    let incentive4 = 100000;

    if experience >=5.0 && age >=40.0
    {
        println!("Incentive is:{}",incentive1);
    }
    else if experience >=5.0 && age >=30.0 && age < 40.0
    {
        println!("Incentive is:{}",incentive2);
    }
    else if experience >=5.0 && age < 28.0
    {
        println!("Incentive is:{}",incentive3);
    }
    else if experience < 5.0
    {
        println!("Incentive is:{}",incentive4);

    }

}
