use std::io;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    write(buffer)
}


fn e_cut(window_1_size: i32, window_2_size:i32, window_size: i32) -> f32{
    let d_confidence: f32 = 0.9;

    let m:f32 = 1 as f32/((1 as f32 / window_1_size as f32) as f32 + (1 as f32 / window_2_size as f32 ) as f32);

    let delta:f32 = d_confidence / window_size as f32;

    let e_cut: f32 = (1 as f32/(2 as f32 *m))* (4 as f32 /delta as f32).log(10 as f32);

    e_cut.sqrt()
}




fn write(buf:String) -> io::Result<()> {


    let mut vec: Vec<i32> = vec![];
    let window_size :i32 = 100;
    let mut window: Vec<i32> = vec![];

    for i in buf.trim().split('\n') {

        let x: i32 = i.trim().parse().expect("Input not an integer");

        window.push(x);


        if window.len()as i32 == window_size{

            let mut vec2: Vec<i32> = window.clone();

            for i in &window{

                vec.push(*i);

                let vec1_avg = vec.iter().sum::<i32>() as f32 / vec.len() as f32;

                let vec2_avg = vec2.iter().sum::<i32>() as f32 / vec2.len() as f32;

                let cut: f32 = e_cut(vec.len() as i32, vec2.len() as i32, window_size);


                    if (vec1_avg -vec2_avg).abs() > cut {
                        let out: String = "Found drift at ".to_owned() + &x.to_string() + " with first window average " + &vec1_avg.to_string() + " and second window average " + &vec2_avg.to_string() + " greater than " + &cut.to_string() + "\n";
                        io::stdout().write(out.as_ref())?;
                        vec = Vec::new();
                        vec2 = Vec::new();

                    }

            }
            window.clear();
            vec.clear();

        }

    }
    Ok(())
}