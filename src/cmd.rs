use std::process::Command;


// pub fn sync_run(cmd: &str, arg: Vec<&str>) -> (bool, String) {
//     let child = Command::new(cmd).args(arg).spawn().output();

//     match child {
//         Ok(c) => {
//             let ret = String::from_utf8_lossy(&c.stdout).into_owned();
//             if c.status.success() {
//                 return (true, ret);
//             }
//             println!("{}", String::from_utf8_lossy(&c.stdout).into_owned());
//             return (false, String::from_utf8_lossy(&c.stderr).into_owned());
//         }
//         Err(e) => {
//             return (false, e.to_string());
//         }
//     }
// }
