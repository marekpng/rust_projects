use corporum::Corporeum;
use std::path::Path;
use std::fs;


fn main() {
    let path = Path::new("../europarl.msgpack");
    let subor = Corporeum::load(&path);
   let mut destination = Path::new("output.json");
   
   subor.save_as_json(&mut destination, true);
    
    // let save_json = Corporeum::save_as_json(&subor,&mut destination, true);
}
