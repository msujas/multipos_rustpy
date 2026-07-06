use pyo3::prelude::*;


// integration - cbfdir, ponidir, tthmin, tthmax, tthbins, chimin, chimax, chibins, maskfile, pfactor

/// A Python module implemented in Rust.
#[pymodule]
mod multipos_rustpy {
use std::{fs::create_dir, path::Path, time::{Instant}};

use multiposrust::MultiFile;
use pyo3::prelude::*;

    /// Rust extension function for processing total-scattering patterns from multiple positions
    #[pyfunction]
    fn integrate_rp(cbfdir:String, ponidir:String, tthmin: f64, tthmax: f64, tthbins: usize, chimin: f64, chimax:f64,
                    chibins: usize, pfactor: f64, maskfile:Option<String>, savecakes:bool, outsubdir:String, cakemaskfile:Option<String>,
                    maskdir: Option<String>, fluocorrection:bool, fluok0o : Option<f64>){
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
        let mf = MultiFile::build(&cbfdir, &ponidir, tthmin, tthmax, tthbins, chimin, chimax, chibins, pfactor, mask, maskdir);
        let t = timestamp.elapsed();
        println!("time elapsed: {}",t.as_secs());
        if fluocorrection{
            let fluo_k0 = match fluok0o{
                None => 1.,
                Some(f) => f,
            };
            mf.integrate_fluosub(4., &cakedir, &avdir, cakemaskfile, fluo_k0, tthbins*96/100);
        }
        else{
            mf.average_cakes(4., &cakedir, &avdir, cakemaskfile);
        }
        println!("cakes averaged, total time {}", timestamp.elapsed().as_secs());
    }

    #[pyfunction]
    fn calculateflatfield(cbfdir:String,ponidir:String, tthmin: f64,tthmax: f64,tthbins: usize, chimin: f64,chimax:f64,pfactor: f64,maskfile:Option<String>,
                            maskdir: Option<String>, ffmin:f64, ffmax:f64){
        let t0 = Instant::now();
        let tmp: String;
        let mask = match maskfile{
            None => None,
            Some(s) => {tmp = s;
                Some(Path::new(&tmp))},
        };
        let mf = MultiFile::build(&cbfdir, &ponidir, tthmin, tthmax, tthbins, chimin, chimax, 50, pfactor, mask, maskdir);
        mf.calculateflatfield(ffmin,ffmax);
        println!("flat field calculated. Total time {}",t0.elapsed().as_secs());
    }
}
