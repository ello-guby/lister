use std::process::exit;

fn main() {
    println!("-------------START:-------------");

    let mut datas: Vec<Vec<String>> = vec![vec![]];

    let mut diter: usize = 0;
    
    {
        let mut first = true;
        let args = std::env::args();
        if args.len() == 1 {
            println!("usage : lister COL1 COL2 ... - ROW1_COL1 ROW1_COL2 ... - ROW2_COL1 ROW2_COL2");
            println!("output: +---------+---------+---+");
            println!("        |COL1     |COL2     |...|");
            println!("        +---------+---------+---+");
            println!("        |ROW1_COL1|ROW1_COL2|...|");
            println!("        |ROW2_COL1|ROW2_COL2|...|");
            println!("        |...      |...      |...|");
            println!("        +---------+---------+---+");
        }
        for arg in args {

            if first {
                first = false;
                continue;
            }

            if arg != "-" {
                datas[diter].push(arg);
            } else {
                check_out_of_bound(&datas, diter);
                
                datas.push(vec![]);
                diter += 1;
            }

        }
    }

    println!("{:?}", get_len_data(&datas));

    println!("-------------DEBUG:-------------\ndatas:\n{:#?}\n/datas:\nditer: {}", datas, diter);
}

fn check_out_of_bound(datas: &Vec<Vec<String>>, diter: usize) {
    if datas[diter].len() < datas[0].len() {
        println!(
            "Error: Row '{:?}' has less then column '{:?}', fill it with something.",
            datas[diter],
            datas[0]
        );
        exit(20)
    } else if datas[diter].len() > datas[0].len() {
        println!(
            "Error: Row '{:?}' has more then column '{:?}'.",
            datas[diter],
            datas[0]
        );
        exit(21);
    }
}

fn get_len_data(datas: &Vec<Vec<String>>) -> Vec<usize> {
    let mut lends: Vec<usize> = vec![]; // len datas

    let mut len: usize = 0;

    for i in 0..datas[0].len() {
        for it in 0..datas.len() {
            if datas[it][i].len() > len {
                len = datas[it][i].len()
            }
        }
        lends.push(len);
        len = 0; 
    }

    return lends;
}