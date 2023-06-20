Pure native gui by [egui](https://github.com/emilk/egui).

Easy for game developer to use headless tools to reduce poor chose in project.


Current support `pm` type: 

* binary 

* mobile lib(gradle `pm` for android and `pod` for ios)

* static file 


## Dev Require 
* node  10+
* cargo 1.6+
  
## Dev 

> cargo run 

`notice:`log is depend on `RUST_LOG` environment, so if u want to see more level log, add env parans in command like(:3 Windows11 NO.1, Other's just use set http_proxy=$u-proxy): 

>  $ENV:RUST_LOG="debug";cargo run

or (cargo-watch NO.1)

> $ENV:RUST_LOG="debug";cargo watch -x run

## Release


Step1.
Make sure u have the *Dev Require*



Step2.
Update the changelog in file *UPDATE_LOG.md*
make sure u have the newest change information like vers before


Step3.
Run the command and wait the push success output info in console.
> node scripts/release.mjs

Step4.
Always u will see the action runnning, and new tag waiting the new ver's file to compile and upload.
