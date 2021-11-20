# OC_Spray
Say OC-spray, Pistol, Batong randomly but never twice in a row.

## Use in Windows
To compile this in Windows, install Visual Studio C++ tools and then change this line in Cargo.toml:
```soloud = { version = "1", default-features = false, features = ["alsa"] }``` -> 
```soloud = { version = "1", default-features = false, features = ["winmm"] }```


