Pure native gui by [egui](https://github.com/emilk/egui).

Easy for game developer to use headless tools to reduce poor chose in project.


Current support `pm` type: 

* binary 

* mobile lib(gradle `pm` for android and `pod` for ios)

* static file 


## Dev 

> cargo run 

`notice:`log is depend on `RUST_LOG` environment, so if u want to see more level log, add env parans in command like(:3 windows is also NO.1): 

>  $ENV:RUST_LOG="debug";cargo run

or (cargo-watch NO.1)

> $ENV:RUST_LOG="debug";cargo watch -x run

## Release


Step1.
Make sure u have the 

> node scripts/release.mjs


