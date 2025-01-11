use std::io;
use itertools::Itertools;

fn main() {
    let mut inp = String::new();

    io::stdin() 
        .read_line(&mut inp)
        .unwrap();

    let eq = inp.trim().to_string();
    println!("Your equation is: {}", inp);

    let tokens = tokenize(&eq);
    println!("{:?}", tokens);

    let result = calc_parameters(&tokens);
    println!("The solution is: {:?}", result);
}

fn tokenize(eq: &String) -> Vec<Vec<usize>> {
    let l = eq.chars().count();
    let v_eq: Vec<_> = eq.chars().collect();

    let mut ans = vec![vec![]];
    let mut chunk = vec![];

    for i in 0..l {
        if v_eq[i] == '+' || v_eq[i] == '=' {
            ans.push(chunk.clone());
            chunk = vec![];
        }
        else {
            if v_eq[i].to_string().parse::<usize>().is_ok() {
                chunk.push(v_eq[i].to_string().parse::<usize>().unwrap());
            }

            if i != l-1 {
                if v_eq[i].is_ascii_uppercase() && (v_eq[i+1].is_ascii_uppercase() || v_eq[i+1].to_string() == " ") {
                    chunk.push(1);
                }
                if v_eq[i].is_ascii_lowercase() && !v_eq[i+1].to_string().parse::<usize>().is_ok() {
                    chunk.push(1);
                }
            }
            else {
                if !v_eq[i].to_string().parse::<usize>().is_ok() {
                    chunk.push(1);
                }
            }
        }

        if v_eq[i] == '=' {
            ans.push(vec![0]);
        }
    }

    let mut zero_index = 0;
    for i in 0..ans.len() {
        if ans[i] == vec![0] {
           zero_index = i;
           break;
        }
    }

    ans.push(chunk.clone());
    ans.remove(zero_index);
    ans[0] = vec![zero_index];

    ans
}

fn resolved(tokens: &Vec<Vec<usize>>, params: &Vec<usize>) -> bool {
    let mut sum = 0;
    let mut after_equal = false;

    assert!(tokens.len()-1 == params.len());
    
    for j in 1..tokens.len() {
        let p = params[j-1];

        for i in tokens[j].clone() {
            if i == 0 {
                after_equal = true;
            }

            if !after_equal {
                sum += p * i
            }
            else {
                sum -= p * i
            }
        }
    }
    
    return sum == 0
}

fn calc_parameters(tokens: &Vec<Vec<usize>>) -> Vec<usize> {
    let len = tokens.len()-1;
    let mut params = vec![1; len];

    for i in 2..10 {
        for j in 0..len {
            params[j] = i;

            for perm in params.iter().cloned().permutations(params.len()).unique() {
                println!("{:?}", perm);

                if resolved(tokens, &perm) {
                    println!("\nEquations solved!");
                    return perm;
                }
            }
        }
    }

    params 
}
