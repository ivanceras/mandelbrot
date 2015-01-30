use std::io::File;


struct Color{
	r:u8,
	g:u8,
	b:u8,
}

fn main(){
	
	let iteration:u64 = 100;
	let magnify:f64 = 0.25;
	let hxres = 500;
	let hyres = 500;
	
	
	
	let total = (hxres)*(hyres);
	println!("total: {}", total);
	let mut pixels:Vec<Color> =Vec::new();// should be 3 * total
	for t in range(0, total){
	    pixels.push(Color{r:0,g:0,b:0});
	}
	
	let mut palette:Vec<u8> = Vec::new();
	for pa in range (0, iteration){
		palette.push(((pa+100) * (100/iteration)) as u8);
	}
	let index:usize = 1;
	//Here N=2^8 is chosen as a reasonable bailout radius.
	let bailout = (1 << 16);
	for si in range (0, iteration){
		for hy in range(0,hyres) {
			for hx in range(0,hxres) {
				let cx:f64 = (hx as f64 / hxres as f64 - 0.5) / magnify ;
				let cy:f64 = (hy as f64 / hyres as f64 - 0.5) / magnify ;
				let mut x:f64 = 0.0;
				let mut y:f64 = 0.0;
				let mut escaped:bool = false;
				let index = hy*hyres+hx;
				for i in range(0,si) {
					let xx:f64 = x*x - y*y + cx;
					y = 2.0 * x*y + cy;
					x = xx;
					if  x*x + y*y > bailout as f64 { 
						escaped = true;
					}
				}
				if ! escaped { 
					if si % 2 == 0 {
						pixels[index] = Color{r:0,g:palette[si as usize],b:0};
					}
					else{
						pixels[index] = Color{r:0,g:0,b:palette[si as usize]};
					}
				}
				else{
				    //will be black, but leave it to preserve previous iteration color values
				} 
			}
		}
	}
	
	save_to_file(pixels, hxres, hyres);
	
	
}

fn save_to_file(pixels:Vec<Color>, hxres:usize, hyres:usize){

	let mut file = File::create(&Path::new("man.ppm"));
	let header = String::from_str(format!("P6\n# CREATOR: lee\n").as_slice());
	file.write(header.into_bytes().as_slice());

	let size = String::from_str(format!("{} {}\n255\n", hxres, hyres).as_slice());
	file.write(size.into_bytes().as_slice());

	for p in range(0,pixels.len()){
		file.write_u8(pixels[p].r);
		file.write_u8(pixels[p].g);
		file.write_u8(pixels[p].b);
	}
}


