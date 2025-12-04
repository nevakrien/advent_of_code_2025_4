fn single_run(input:&str)->(String,usize){
    let lines: Vec<&str> = input.lines().collect();
    let mut new_input = String::new();


    let mut count = 0;
    for (y,l) in lines.iter().enumerate(){

        for (x,c) in l.as_bytes().iter().enumerate(){
            if *c!=b'@'{
                new_input.push_str(".");
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
                new_input.push_str("x");
                count+=1;
            }else{
                new_input.push_str("@")
            }
        }

        new_input.push_str("\n");
    }
    (new_input,count)

}

//part 1
// fn main() {
//     let input = include_str!("input.txt");

//     let (_,count) = single_run(input);
    
//     println!("found {count}");
// }

//part 2
fn main() {
    let mut input = include_str!("input.txt").to_string();
    let mut full_count = 0;
    loop {
        println!("current state:\n{input}");
        let (new_input,count) = single_run(&input);
        input = new_input;
        full_count+=count;

        if count==0{
            break;
        }
    }
    
    println!("found {full_count}");
}