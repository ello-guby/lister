use std::process::exit;

fn main() {

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
            exit(22);
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

    {
        // output
        let len_data = get_len_data(&datas);
        println!("{}", mk_sepver(&len_data));
        println!("{}", mk_sephor(&datas[0], &len_data));
        println!("{}", mk_sepver(&len_data));
        for i in 1..datas.len() {
            println!("{}", mk_sephor(&datas[i], &len_data))
        }
        println!("{}", mk_sepver(&len_data));
    }
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

fn mk_sepver(len_data: &Vec<usize>) -> String {

    let sepver = '-';
    let cross = '+';
    let mut s = String::from(cross);

    for len in len_data {
        for _count in 0..=*len - 1 {
            s.push(sepver);
        }
        s.push(cross);
    }

    s
}

fn mk_sephor(data: &Vec<String>, len_data: &Vec<usize>) -> String {

    let sephor = '|';
    let mut s = String::from(sephor);

    for i in 0..=data.len() - 1 {
        s.push_str(data[i].as_str());

        if len_data[i] > data[i].len() {
            for _it in 0..len_data[i] - data[i].len() { // get space length
                s.push(' ');
            }
        }

        s.push(sephor);
    }

    s
}
