use supercut::masked_cut;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() !=4 {
        println!("{:?}", args);
        println!("Usage: scut [image] [mask image] [output file name]");
        return;
    }

    let image = image::open(&args[1]).unwrap();
    let mask = image::open(&args[2]).unwrap();

    masked_cut(&image, &mask).save(&args[3]).unwrap();
}
