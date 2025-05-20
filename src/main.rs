use std::process::exit;

fn main() {

    let mut datas: Vec<Vec<String>> = vec![vec![]];

    let mut diter: usize = 0;

    let mut use_boxdraw: bool = false;
    
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
            println!("options:");
            println!("    -d: flip between box drawing display or ascii display.");
            exit(0);
        }
        for arg in args {

            if first {
                first = false;
                continue;
            }

            if arg != "-" {
                if arg == "-d" {
                    use_boxdraw = !use_boxdraw;
                } else {
                    datas[diter].push(arg);
                }
            } else {
                check_out_of_bound(&datas, diter);
                
                datas.push(vec![]);
                diter += 1;
            }

        }
    }

    check_out_of_bound(&datas, diter);

    {
        // output
        let len_data = get_len_data(&datas);
        println!("{}", mk_sepver(&len_data, &use_boxdraw, Some(true)));
        println!("{}", mk_sephor(&datas[0], &len_data, &use_boxdraw));
        println!("{}", mk_sepver(&len_data, &use_boxdraw, None));
        for i in 1..datas.len() {
            println!("{}", mk_sephor(&datas[i], &len_data, &use_boxdraw))
        }
        println!("{}", mk_sepver(&len_data, &use_boxdraw, Some(false)));
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

fn mk_sepver(len_data: &Vec<usize>, use_boxdraw: &bool, is_top: Option<bool>) -> String {
    let start_corner: char;
    let trail: char;
    let cross_corner: char;
    let end_corner: char;
    {
        let horizontal = '─';

        let cross_mid_corner = '┼';

        let cross_top_corner = '┬';
        let cross_bottom_corner = '┴';

        let start_mid_corner = '├';
        let end_mid_corner = '┤';

        let start_top_corner = '┌';
        let end_top_corner = '┐';

        let start_bottom_corner = '└';
        let end_bottom_corner = '┘';

        let minus = '-';
        let plus = '+';

        start_corner = if *use_boxdraw {
            match is_top {
                Some(yes) => {
                    if yes {
                        start_top_corner
                    } else {
                        start_bottom_corner
                    }
                },
                None => start_mid_corner
            }
        } else {
            plus
        };

        trail = if *use_boxdraw {
            horizontal
        } else {
            minus
        };

        cross_corner = if *use_boxdraw {
            match is_top {
                Some(yes) => {
                    if yes {
                        cross_top_corner
                    } else {
                        cross_bottom_corner
                    }
                },
                None => cross_mid_corner
            }
        } else {
            plus
        };

        end_corner = if *use_boxdraw {
            match is_top {
                Some(yes) => {
                    if yes {
                        end_top_corner
                    } else {
                        end_bottom_corner
                    }
                },
                None => end_mid_corner
            }
        } else {
            plus
        };
    }
    let mut s = String::from(start_corner);

    for len in len_data {
        for _count in 0..=*len - 1 {
            s.push(trail);
        }
        s.push(cross_corner);
    }

    s.pop();
    s.push(end_corner);

    s
}

fn mk_sephor(data: &Vec<String>, len_data: &Vec<usize>, use_boxdraw: &bool) -> String {

    let split: char;
    {
        let vertical = '│';
        let pipe = '|';
        split = if *use_boxdraw {
            vertical
        } else {
            pipe
        };
    }
    let mut s = String::from(split);

    for i in 0..=data.len() - 1 {
        s.push_str(data[i].as_str());

        if len_data[i] > data[i].len() {
            for _it in 0..len_data[i] - data[i].len() { // get space length
                s.push(' ');
            }
        }

        s.push(split);
    }

    s
}
