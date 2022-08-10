mod test;
use std::fs::{File, OpenOptions};
use std::io::{SeekFrom, Read, Write, Seek};
use std::path::PathBuf;
use pe::{Pe, AsOsStr};

pub struct MorpheusEngine{
    file: PathBuf,
    
}
impl MorpheusEngine{
    pub fn modify(&self){
        let mut file=OpenOptions::new().read(true).write(true).truncate(false).append(false).open(&self.file).unwrap();
        let mut buf=vec![];
	    file.read_to_end(&mut buf).unwrap();
	    let pe=Pe::new(&buf).unwrap();
        let sections = pe.get_sections();
        for section in sections.iter() {
            println!("Section: {:?}",section.name.as_os_str());
        }
        



    }
}
