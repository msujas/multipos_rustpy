use pyo3::prelude::*;


// integration - cbfdir, ponidir, tthmin, tthmax, tthbins, chimin, chimax, chibins, maskfile, pfactor

/// A Python module implemented in Rust.
#[pymodule]
mod multipos_rustpy {
use std::{fs::create_dir, path::Path, time::{Instant}};

use multipos_rust::functions::MultiFile;
use pyo3::prelude::*;

    /// Rust extension function for processing total-scattering patterns from multiple positions
    #[pyfunction]
    fn integrate_rp(cbfdir:String, ponidir:String, tthmin: f64, tthmax: f64, tthbins: usize, chimin: f64, chimax:f64,
                    chibins: usize, pfactor: f64, maskfile:Option<String>, savecakes:bool, outsubdir:String, cakemaskfile:Option<String>){
        let timestamp = Instant::now();
        let tmp:String;
        let mask = match maskfile{
            None => None,
            Some(s) => {tmp = s;
                Some(Path::new(&tmp))},
        };
        let mut cakedir = String::from("");
        if savecakes {
            cakedir = format!("{}/{}", &cbfdir, &outsubdir)
        }
        let avdir = format!("{}/{}", &cbfdir, &outsubdir);
        let _ = create_dir(&avdir);
        println!("collecting ponis");
        let mf = MultiFile::build(&cbfdir, &ponidir, tthmin, tthmax, tthbins, chimin, chimax, chibins, pfactor, mask);
        let t = timestamp.elapsed();
        println!("time elapsed: {}",t.as_secs());
        mf.average_cakes(4., &cakedir, &avdir, cakemaskfile);
        println!("cakes averaged, total time {}", timestamp.elapsed().as_secs());
    }
}
