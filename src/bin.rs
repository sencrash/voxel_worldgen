extern crate image;
extern crate noise;

mod simplex_normalized;
use simplex_normalized::normalize_simplex;

mod layer;
use layer::{ LayerLCG };

use image::{ ImageBuffer };

use layer::GenL;
use layer::{ GenLayer, GenReduceOcean, GenIsland, GenZoom, ZoomType, GenSimpleFn };
use layer::{ GenSimplex, SimplexNoiseType };
use layer::{ GenSimpleFnMixer };

mod generators;



mod analysis;
use analysis::simplex_binning::{divide, transform_simplex};
use noise::{Seed, open_simplex2};

extern crate time;
fn main() {
    /*let mut lcg = LayerLCG::new(12);
    lcg.seed_world(82);
    lcg.seed_pos(23, 92);

    for _ in 0..100 {
        println!("{:?}", lcg.next_int(10));
    }*/

    /*let mut src: Box<GenLayer> = GenIsland::new(1, 2);
    src = GenZoom::new(2, ZoomType::FUZZY, src);
    src = GenZoom::new(3, ZoomType::MAJORITY, src);
    let mut src1: Box<GenLayer> = GenIsland::new(2, 6);
    src = GenBlend::new(BlendMode::Or, src1, src);
    src = GenZoom::new(4, ZoomType::MAJORITY, src);
    src = GenReduceOcean::new(6, 2, src);
    let mut dst = GenZoom::new(5, ZoomType::MAJORITY, src);*/

    //divide(1000, 20_000_000);
    
    //let seed1 = Seed::new(1);

    println!("start");
    let start = time::precise_time_ns();
    let mut dst = test();
    let buf = dst.gen(10, 0, 0, 1024, 1024);
    let end = time::precise_time_ns();
    println!("end {:?} {:?}", buf[3432], end - start);

    let img = ImageBuffer::from_fn(1024, 1024, |x, y| {
        /*if buf[(x + y * 1024) as usize] {
            image::Luma([255])
        } else {
            image::Luma([0])
        }*/
        let (num, tum) = buf[(x + y * 1024) as usize];
        //image::Rgb([(num / 8) * 16, (num % 8) * 16, 0])
        image::Rgb([num*16, tum*16, 0])
        /*let val = if x < 512 {
            normalize_simplex(open_simplex2(&seed, &[x as f32 / 32.0, y as f32 / 32.0]))
        } else {
            open_simplex2(&seed, &[x as f32 / 32.0, y as f32 / 32.0])
        };
        let val_int = (((val + 1.0) / 2.0) * 255.0) as u8;
        image::Luma([val_int])*/
    });

    img.save("test.png").unwrap();

}
