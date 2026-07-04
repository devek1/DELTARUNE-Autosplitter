use std::fs::{read};
use std::os::wasi;
use asr::{future::next_tick, settings::Gui, PointerSize, Process, deep_pointer::DeepPointer, watcher::{Watcher,Pair}, Address};
use asr::string::{ArrayCString, ArrayString};
use md5::Digest;

asr::async_main!(stable);

#[derive(Gui)]
struct Settings {
    /// My Setting
    #[default = true]
    my_setting: bool,
    // TODO: Change these settings.
}

static null : &[u64] = &[0];

pub struct VarTrack<T: Clone + bytemuck::Pod> {
    pub pointer : Option<DeepPointer<16>>,
    pub watcher : Option<Watcher<T>>,
}
impl<T: Clone + bytemuck::Pod> VarTrack<T> {
    pub fn disabled() -> VarTrack<T> {
        VarTrack {
            watcher: None,
            pointer: None,
        }
    }
    pub fn new(module_base : Address, pointer_size: PointerSize, offsets : &[u64]) -> VarTrack<T> {
        VarTrack {
            watcher: Some(Watcher::<T>::new()),
            pointer: match offsets {
                &[0] => None,
                x => Some(DeepPointer::new(module_base, pointer_size, x)),
            }
        }
    }
    pub fn update_value(&mut self, process: &Process) -> Pair<T> {
        if self.pointer.is_none() || self.watcher.is_none() {
            return Pair {
                old: T::zeroed(),
                current: T::zeroed(),
            }
        }
        let value: Option<T> = match self.pointer.unwrap().deref(&process) {
            Ok(val) => Some(val),
            Err(_e) => Some(T::zeroed()),
        };
        return *self.watcher.unwrap().update_infallible(value.unwrap());
    }
}


async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

    let mut version : &str = "invalid";

    asr::print_message("Hello, World!");

    loop {
        let process = Process::wait_attach("DELTARUNE.exe").await;
        let Ok(mut path) = process.get_module_path("DELTARUNE.exe") else {
            next_tick().await;
            continue;
        };

        process
            .until_closes(async {

                let (DELTARUNE, module_size) = process.wait_module_range("DELTARUNE.exe").await;


                path = path.replace("DELTARUNE.exe", "data.win");
                let md5 = &format!("{:X?}", md5::compute(read(path).unwrap_or_default()));
                asr::timer::set_variable("MD5", md5);
                version = match md5.to_uppercase().as_str() {
                    "DDEDBBD10FF129B49C64DBEFAA763C6A" | //v244 vanilla
                    "4A9C69B42E442B673395B3253F292F17" | //v244 30tbps
                    "42B66B41B6CEA12FB54219E9D31E5DC8" | //v244 Item Tracker
                    "D0420C09A5DEBD6176EA24A1FE1EE3E3" => "CH1-5 v244", //v244 OST tracker
                    "B5EF0EEC9554C491777D6C4E93E0DF76" | //v1.02 vanilla
                    "40A8185886A8A1A2BE996BC57DE3D916" => "CH1-4 v1.02", //v1.02 30tbps
                    "7AD299A8B33FA449E20EDFE0FEDEDDB2" | //demo 1.19 vanilla
                    "FD0857E6A3AF3AA74E5E00F15AEA5224" => "Demo v1.19", //demo 1.19 30tbps
                    "ED4568BAB864166BFD6322CEEB3FB544" | //demo 1.15 vanilla
                    "6BD6D1381C194C0F456B184CB48D132D" => "Demo v1.15", //demo 1.15 30tbps
                    "5FBE01F2BC1C04F45D809FFD060AC386" | //demo 1.10 vanilla Itch
                    "A37C77A4310D2D6A6C2AF18294AAAE7A" | //demo 1.10 30tbps Itch
                    "CD77A63D7902990DBC704FE32B30700A" | //demo 1.10 vanilla Steam
                    "758C8862F22F778FDEAFE25FBCD1F4EC" => "Demo v1.10", //demo 1.10 30tbps Steam
                    "616C5751AC9FC584AF250F1B04474AFD" | //demo 1.09 vanilla Itch
                    "05689183497E58838E99B897F2E0E6AC" | //demo 1.09 30tbps Itch
                    "267A8ABE468D824222810201F00003BE" | //demo 1.09 vanilla Steam
                    "272A16964597ED6DC8D2393ED051D3CE" => "Demo v1.09",
                    "A88A2DB3A68C714CA2B1FF57AC08A032" | //SP-EN vanilla
                    "047C11435B1C592EC731BFF3B9C5B0CF" | //SP-EN 30tbps
                    "22008370824A37BAEF8948127963C769" | //SP-JP vanilla
                    "E05433FE679BC91E3809C1138E3A8EA1" => "SURVEY_PROGRAM", //SP-JP 30tbps
                    _ => "invalid",
                };
                asr::timer::set_variable("version", version);

                if version == "invalid" { loop { next_tick().await; } }

                let ps = match version {
                    "SURVEY_PROGRAM" |
                    "Demo v1.09" |
                    "Demo v1.10" |
                    "Demo v1.15" => PointerSize::Bit32,
                    _ => PointerSize::Bit64
                };

                let mut chapter = 0;

                if ps == PointerSize::Bit64
                { //the directory only changes with change_game which starts a whole new process for the autosplitter to attach to, so we only need to read it once per process attached
                    let _dir = process.read_pointer_path::<ArrayCString<256>>(DELTARUNE, ps, match version {
                        "CH1-5 v244" => &[0x8BA818,0],
                        "CH1-4 v1.02" => &[0x8B2818,0],
                        "Demo v1.19" => &[0x8D06E0,0],
                        _ => unreachable!(),
                    }).unwrap_or_default();
                    let dir = _dir.validate_utf8().unwrap_or_default();
                    asr::timer::set_variable("dir", dir);
                    if dir.ends_with("chapter1_windows\\") { chapter = 1 }
                    else if dir.ends_with("chapter2_windows\\") { chapter = 2 }
                    else if dir.ends_with("chapter3_windows\\") { chapter = 3 }
                    else if dir.ends_with("chapter4_windows\\") { chapter = 4 }
                    else if dir.ends_with("chapter5_windows\\") { chapter = 5 }
                }

                let mut old_chapter_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x24D8, 0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0x2F34, 0x80],
                    _ => null
                });

                let mut plot_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "CH1-4 v1.02" => match chapter {
                        3 => &[0x6A1CA8, 0x48,  0x10,   0x1000, 0x250],
                        4 => &[0x6A1CA8, 0x48,  0x10,   0x2F40, 0x30],
                        _ => null
                    },
                    "CH1-5 v244" => match chapter {
                        3 => &[0x6A9CA8, 0x48,  0x10,   0x1000, 0x250],
                        4 => &[0x6A9CA8, 0x48,  0x10,   0x2F70, 0x30],
                        _ => null
                    },
                    _ => null
                });

                let mut choicer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x28,  0x40],
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x18C0, 0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0xBA0,  0xC0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0x0],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0x0],
                        _ => unreachable!()
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x10],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x7870, 0x0],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x10],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x7870, 0x0],
                        5 => &[0x6A9CA8, 0x48,  0x10,   0x150,  0x20],
                        _ => null
                    }
                    _ => null,
                });






                // TODO: Load some initial information from the process.
                loop {
                    settings.update();

                    if ps == PointerSize::Bit32 && version != "SURVEY_PROGRAM" {}
                    asr::timer::set_variable_int("Chapter",chapter);


                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
