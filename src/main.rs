use usvg::SystemFontDB;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage:\n\tminimal <in-svg> <out-png>");
        return;
    }

    let mut opt = usvg::Options::default();
    opt.path = Some(args[1].clone().into());
    opt.fontdb.load_system_fonts();
    println!("loaded fonts: {}", opt.fontdb.len());

    let rtree = usvg::Tree::from_file(&args[1], &opt).unwrap();
    println!("parsed svg: {}", rtree.to_string(usvg::XmlOptions::default()));

    let img = resvg::render(&rtree, usvg::FitTo::Original, None).unwrap();
    img.save_png(&args[2]).unwrap();
}
