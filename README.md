Python binding for the rust multi position X-ray total scattering processing library. To install run the build.bat or build.sh scripts, these run:
```
cargo update
cargo build --release
pip install .
```
Also run this to update. This will install the necessary rust libraries and install as a Python package.

Then in the main Python library (https://github.com/msujas/multipositionpdf), the rust extension module can be used
```python
from multipospdf.rustext import runintegration_rp

cbfdir = 'myimagedir'
ponidir = 'ponidir' #doesn't need to contain poni for every postion, just enough for interpolations
maskfile = 'maskpath'
tthmin = 0.75
tthmax = 67.5
tthbins = 5000
chimin = 218
chimax = 335
chibins = chimax-chimin+1
fluocorrection = False

runintegration_rp(cbfdir,ponidir, tthmin,tthmax, tthbins, chimin, chimax, chibins, maskfile = maskfile,
                fluocorrection=fluocorrection)
```