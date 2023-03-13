use std::io;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    write(buffer)
}


fn e_cut(window_1_size: i32, window_2_size:i32) -> f32{
    let d_confidence: f32 = 0.1;

    let m:f32 = 1 as f32/((1 as f32 / window_1_size as f32) as f32 + (1 as f32 / window_2_size as f32 ) as f32);

    let delta:f32 = d_confidence / 100 as f32;

    let e_cut: f32 = (1 as f32/(2 as f32 *m))* (4 as f32 /delta as f32).log(10 as f32);

    e_cut.sqrt()
}




fn write(buf:String) -> io::Result<()> {

    let mut vec: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];
    let window_size :i32 = 10;

    for i in buf.trim().split('\n') {

        let x: i32 = i.trim().parse().expect("Input not an integer");


        if vec.len()as i32 > window_size{

            vec2.push(x);

            let vec1_avg = vec.iter().sum::<i32>() as f32 / vec.len() as f32;
            let vec2_avg = vec2.iter().sum::<i32>() as f32 / vec.len() as f32;

                if (vec1_avg -vec2_avg) > e_cut(vec.len() as i32, vec2.len() as i32) {
                    let out: String = "Found drift at ".to_owned() + &x.to_string() + "\n";
                    io::stdout().write(out.as_ref())?;
                    vec.clear();
                    vec2 = Vec::new();

                }



        }
        vec.push(x);
    }
    Ok(())
}