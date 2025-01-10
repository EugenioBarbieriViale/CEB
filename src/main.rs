use std::io;

fn main() {
    let mut inp = String::new();

    io::stdin() 
        .read_line(&mut inp)
        .unwrap();

    println!("Your equation is: {}", inp);
    
    let eq = inp.trim().replace(" ", "");

    // let tokenized_eq = tokenize(&eq)
    // println!("{:?}", tokenized_eq);
    let v = assign_values(&eq);
    println!("{:?}", v);
}

fn tokenize(eq: &String) -> Vec<String> {
    let l = eq.chars().count();
    let v_eq: Vec<_> = eq.chars().collect();
    let mut ans: Vec<String> = vec![];

    for i in 0..l {
        if v_eq[i].is_ascii_uppercase() {
            if v_eq[i+1].is_ascii_lowercase() {
                let mut c = v_eq[i].to_string();
                c.push(v_eq[i+1]);
                
                if v_eq[i+2].to_string().parse::<usize>().is_ok() {
                    c.push(v_eq[i+2]);
                }

                ans.push(c);
            }
            else {
                let mut c = v_eq[i].to_string();
                if v_eq[i+1].to_string().parse::<usize>().is_ok() {
                    c.push(v_eq[i+1]);
                }
                ans.push(c.to_string());
            }
        }

        if v_eq[i] == '+' || v_eq[i] == '=' {
            ans.push(v_eq[i].to_string());
        }
    }
    ans
}

// fn assign_values(eq: &Vec<String>) -> Vec<usize> {
// fn assign_values(eq: &String) -> Vec<usize> {
//     let mut ans = vec![];
    
//     for c in eq.chars() {
//         if c.to_string().parse::<usize>().is_ok() || c == '=' {
//             ans.push(c.to_string().parse::<usize>().unwrap());
//         }
//     }

//     ans
// }
