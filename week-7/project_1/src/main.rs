 use std::io;
 fn main() {
    

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();
    let mut input12 = String::new();

    

    println!("Welcome to MATH101 class, what would you like to calculate?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    if input1.trim() == "Area of Trapezium" {
        println!("What is the height of the Trapezium");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let height:f64 = input2.trim().parse().expect("Failed to read input");

        println!("What is the value of base1?");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let base1:f64 = input3.trim().parse().expect("Failed to read input");

        println!("What is the value of base2?");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let base2:f64 = input4.trim().parse().expect("Failed to read input");

        let sum = base1 + base2;
        let Area_of_Trapezium:f64 = sum * height / 2.0;
        println!("Area of Trapezium is {}", Area_of_Trapezium);

    }
    else if input1.trim() =="Area of rhombus"{
        println!("What is the value of diagonal1");
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let diagonal1:f64 = input5.trim().parse().expect("Failed to read input");

        println!("What is the value of diagonal2");
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let diagonal2:f64 = input6.trim().parse().expect("Failed to read input");
        let Area_of_rhombus:f64 = 0.5 * diagonal1 * diagonal2;
        println!("Area of rhombus is {}",Area_of_rhombus);

    }
    else if input1.trim() =="Area of Parallelogram"{
        println!("What is the value of the base");
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let base:f64 = input7.trim().parse().expect("Failed to read input");

        println!("What is the value of the altitude");
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let altitude:f64 = input8.trim().parse().expect("Failed to read input");
        let Area_of_Parallelogram:f64 = base * altitude;
        println!("Area of Parallelogram is {}",Area_of_Parallelogram);
    } 
    else if input1.trim() == "Area of Cube"{
        println!("What is the lenght of the side");
        io::stdin().read_line(&mut input9).expect("Failed to read input");
        let lenght_of_side:f64 = input9.trim().parse().expect("Failed to read input");
        let Area_of_Cube:f64 = 6.0 * lenght_of_side * lenght_of_side;
        println!("Area of Cube is {}",Area_of_Cube);
    }    
    else if input1.trim() =="Volume of Cylinder"{
        println!("What is the value of the radius");
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let radius:f64 = input10.trim().parse().expect("Failed to read input");

         println!("What is the height of the cylinder");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let height:f64 = input2.trim().parse().expect("Failed to read input");
        let Volume_of_cylinder:f64 = 3.142 * height * radius * radius ;
        println!("Volume of the cylinder is {}",Volume_of_cylinder);
    } 
    else {
        println!("Invalid Input");
    }


    
}
