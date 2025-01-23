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

    calc_parameters(&tokens);
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
    let mut sum: isize = 0;
    let equal_sign_pos = tokens[0][0];

    assert!(tokens.len()-1 == params.len());
    
    for i in 1..tokens.len() {
        let j = i - 1;
        let p = params[j];

        for t in &tokens[i] {
            if i < equal_sign_pos {
                sum += (p * t) as isize;
            } else {
                sum -= (p * t) as isize;
            }
        }
    }

    return sum == 0;
}

fn calc_parameters(tokens: &Vec<Vec<usize>>) {
    let len = tokens.len()-1;
    let params = vec![1,2,3,4,5,6,7,8,9];

    for perm in params.iter().cloned().permutations(len) {
        if resolved(tokens, &perm) {
            println!("FOUND: --- {:?} ---", perm);
        }
    }
}
