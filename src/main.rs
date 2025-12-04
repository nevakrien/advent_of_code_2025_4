fn main() {
    let input = include_str!("input.txt");

    let lines: Vec<_> = input.lines().collect();
    let mut count = 0;
    for (y,l) in lines.iter().enumerate(){
        for (x,c) in l.as_bytes().iter().enumerate(){
            if *c!=b'@'{
                print!(".");
                continue;
            }


            let mut adjecent = 0;
            for other_y in y.saturating_sub(1)..=y+1{
                let Some(other) = lines.get(other_y) else{
                    continue;
                };

                let other = other.as_bytes();

                for other_x in x.saturating_sub(1)..=x+1{                    
                    if other.get(other_x) == Some(&b'@'){
                        adjecent+=1;
                    }
                }

            }


            if adjecent-1<4 {
                print!("x");
                count+=1;
            }else{
                print!("@")
            }
        }
        println!("")
    }

    println!("found {count}");
}
