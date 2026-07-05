use std::fmt::UpperHex;
use std::fs::{read};
use std::os::wasi;
use asr::{future::next_tick, settings::Gui, PointerSize, Process, deep_pointer::DeepPointer,
    watcher::{Watcher,Pair}, Address, signature::Signature};
use asr::file_format::pe;
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

const ps64: PointerSize = PointerSize::Bit64;
const ps32: PointerSize = PointerSize::Bit32;

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

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
                let module_size2 = pe::read_size_of_image(&process,DELTARUNE).unwrap_or_default();
                //let module_size3 = process.get_module_size("DELTARUNE.exe").unwrap_or_default();
                asr::timer::set_variable("Module Address",format!("{:X}",DELTARUNE.value()).as_str());
                //asr::timer::set_variable_int("Module Address",DELTARUNE.value());


                path = path.replace("DELTARUNE.exe", "data.win");
                asr::timer::set_variable("Path", path.as_str());
                let md5 = &format!("{:X?}", md5::compute(read(path).unwrap_or_default()));
                asr::timer::set_variable("MD5", md5);
                let version = match md5.to_uppercase().as_str() {
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
                    "Demo v1.15" => ps32,
                    _ => ps64
                };

                let mut chapter = 0;

                if ps == ps64
                { //the directory only changes with change_game which starts a whole new process for the autosplitter to attach to, so we only need to read it once per process attached
                    let mut _dir : ArrayCString<256>;
                    loop {
                        _dir = process.read_pointer_path::<ArrayCString<256>>(DELTARUNE, ps, match version {
                            "CH1-5 v244" => &[0x8BA818,0],
                            "CH1-4 v1.02" => &[0x8B2818,0],
                            "Demo v1.19" => &[0x8D06E0,0],
                            _ => null,
                        }).unwrap_or_default();
                        if _dir != ArrayCString::<256>::default() {
                            break;
                        }
                        next_tick().await;
                    }
                    let dir = _dir.validate_utf8().unwrap_or("invalid UTF8");
                    asr::timer::set_variable("dir", dir);
                    if dir.ends_with("chapter1_windows\\") { chapter = 1 }
                    else if dir.ends_with("chapter2_windows\\") { chapter = 2 }
                    else if dir.ends_with("chapter3_windows\\") { chapter = 3 }
                    else if dir.ends_with("chapter4_windows\\") { chapter = 4 }
                    else if dir.ends_with("chapter5_windows\\") { chapter = 5 }
                }
                asr::timer::set_variable_int("Chapter",chapter);


                //rooms

                static array_sig64 : Signature<13> = Signature::new(&"74 0C 48 8B 05 ?? ?? ?? ?? 48 8B 04 D0");
                static array_sig32 : Signature<8> = Signature::new(&"8B 3D ?? ?? ?? ?? 2B EF");

                static id_sig64 : Signature<23> = Signature::new(&"48 ?? ?? ?? 3B 35 ?? ?? ?? ?? 41 ?? ?? ?? 49 ?? ?? E8 ?? ?? ?? ?? FF");
                static id_sig32 : Signature<16> = Signature::new(&"FF 35 ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 C4 04 50 68");

                let mut room_array_addr = match ps {
                    ps64 => array_sig64.wait_scan_process_range(&process, (DELTARUNE, module_size2 as u64)).await.add(5),
                    _ => process.read_pointer(array_sig32.wait_scan_process_range(&process,(DELTARUNE,module_size)).await,ps).unwrap_or_default().add(2)
                };
                room_array_addr = match ps {
                    ps64 => room_array_addr.add(process.read::<i32>(room_array_addr).unwrap_or_default() as u64 + 4),
                    _ => room_array_addr
                };
                asr::timer::set_variable("Room Array Address",format!("{:X}",room_array_addr.value()).as_str());

                /*let room_id_addr = match ps {
                    PointerSize::Bit64 => {
                        let addr1 = id_sig64.wait_scan_process_range(&process,(DELTARUNE,module_size)).await.add(6);
                        return addr1.add(process.read::<i32>(addr1).unwrap_or_default() as u64 + 4);
                    },
                    _ => process.read_pointer(id_sig32.wait_scan_process_range(&process,(DELTARUNE,module_size)).await,ps).unwrap_or_default().add(2)
                };*/
                let mut room_id_addr = match ps {
                    ps64 => id_sig64.wait_scan_process_range(&process, (DELTARUNE, module_size2 as u64)).await.add(6),
                    _ => process.read_pointer(id_sig32.wait_scan_process_range(&process,(DELTARUNE,module_size)).await,ps).unwrap_or_default().add(2)
                };
                asr::print_message("passed room array part 1");
                room_id_addr = match ps {
                    ps64 => room_id_addr.add(process.read::<i32>(room_id_addr).unwrap_or_default() as u64 + 4),
                    _ => room_id_addr
                };
                asr::timer::set_variable("Room ID Address",format!("{:X}",room_id_addr.value()).as_str());

                let mut room_watch = Watcher::<i32>::new();
                let mut room_name_watch = Watcher::<ArrayCString<64>>::new();


                //DEVICE_NAMER.EVENT

                let mut namer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" => &[0x6EF220, 0xD4, 0x5C,  0x20, 0x24,  0x10, 0x9C,  0x0],
                    "Demo v1.10" => &[0x6EF220, 0xD4, 0x5C,  0x20, 0x24,  0x10, 0x2F4, 0x0],
                    "Demo v1.15" => &[0x6F0B48, 0xD4, 0x5C,  0x20, 0x24, 0x10, 0xFC,  0x0],
                    "Demo v1.19" => &[0x8B2790, 0x178, 0x70,  0x38, 0x48,  0x10, 0x3B0, 0x0],
                    "CH1-4 v1.02" => match chapter {
                        2 | 3 => &[0x8B2790, 0x178, 0x70, 0x38, 0x48, 0x10, 0x60, 0x0],
                        4 => &[0x8B2790, 0x178, 0x70, 0x38, 0x48, 0x10, 0x280, 0x0],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        2 => &[0x8BA790, 0x178, 0x70,  0x38,   0x48,  0x10,  0x90,  0x0],
                        3 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x120, 0x0],
                        4 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48,  0x10,  0x40,  0x0],
                        5 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x170, 0x0],
                        _ => null
                    }
                    _ => null
                });

                //Global variables
                //(note: for global.flag[N] values, the last offset is the only difference between different flags' locations, and is equal to 16x the flag's index number - which you can get either by directly multiplying by 16 and putting it in as a decimal number, or by converting to hex then adding a trailing zero.)

                let mut snd_ptr = VarTrack::<ArrayCString<256>>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x4E0794, 0x58, 0xC0,  0x40, 0x0],
                    "Demo v1.15" => &[0x4E20B4, 0x58, 0xC0,  0x40, 0x0],
                    "Demo v1.19" | "CH1-4 v1.02" => &[0x6A3818, 0x60, 0xD0, 0x58, 0x0],
                    "CH1-5 v244" => &[0x6AB818, 0x60, 0xD0, 0x58, 0x0],
                    _ => null
                });

                let mut mus_ptr = VarTrack::<ArrayCString<256>>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x4DFF58, 0x0,  0x44,  0x0],
                    "Demo v1.15" => &[0x4E1878, 0x0,  0x0,   0x0],
                    "Demo v1.19" | "CH1-4 v1.02" => &[0x6A2F90, 0x0,  0x0,  0x0],
                    "CH1-5 v244" => &[0x6AAF90, 0x0,  0x0,  0x0],
                    _ => null
                });

                let mut old_chapter_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x24D8, 0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0x2F34, 0x80],
                    _ => null
                });

                let mut filechoice_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x488, 0x4D0],
                    _ => null
                });

                let mut fighting_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    "SURVEY_PROGRAM" => null,
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x4F8,  0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0xA758, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0x710],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0xBB0],
                        _ => null
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x720],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x100,  0x0],
                        3 => &[0x6A1CA8, 0x48,  0x10,   0x1190, 0x370],
                        4 => &[0x6A1CA8, 0x48,  0x10,   0x72B0, 0x370],
                        _ => null
                    }
                    "CH1-5 v1.02" => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x740],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x100,  0x0],
                        3 => &[0x6A9CA8, 0x48,  0x10,   0x1190, 0x370],
                        4 => &[0x6A9CA8, 0x48,  0x10,   0x72B0, 0x370],
                        5 => &[0x6A9CA8, 0x48,  0x10,   0x820,  0x70],
                        _ => null
                    }
                    _ => null,
                });

                let mut plot_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x488, 0x500],
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
                        _ => null
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

                let mut msc_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x28,  0x140],
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x354C, 0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0x17AC, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0xF0],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0x130],
                        _ => null
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x100],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x7310, 0x0],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x100],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x7310, 0x0],
                        _ => null
                    }
                    _ => null
                });

                //globals with chapter-specific relevance (e.g. flags)

                let mut knight_result_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        "CH1-4 v1.02" => &[0x6A1CA8, 0x48,  0x10,   0x6A70, 0x0,  0x90, 0x4170],
                        "CH1-5 v244" => &[0x6A9CA8, 0x48,  0x10,   0x6A70, 0x0,  0x90, 0x4170],
                        _ => null
                    }
                    _ => null
                });

                let mut pink_coins_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    5 => match version {
                        "CH1-5 v244" => &[0x6A9CA8, 0x48,  0x10,   0x6BB0, 0x0,  0x90, 0x5200],
                        _ => null
                    }
                    _ => null
                });




                //recurring objects across chapters

                let mut text_ptr1 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" |
                    "Demo v1.10" => &[0x6FCE4C, 0x8,  0x144, 0x24, 0x10, 0x5A0, 0x0, 0x0, 0x0],
                    "Demo v1.15" => &[0x6FE774, 0x8,  0x144, 0x24, 0x10, 0x0, 0x0, 0x0, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0xF0, 0x0, 0x0, 0x0],
                        2 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x5F0, 0x0, 0x0, 0x0],
                        _ => null
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x8C2008, 0x10, 0x1A0, 0x48,   0x10,  0x390, 0x0, 0x0, 0x0],
                        2 => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x6F0, 0x0,   0x0,  0x0],
                        4 => &[0x8C2008, 0x10,  0x1A0,  0x48,   0x10,  0x300, 0x0,   0x0, 0x0],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x8CE220, 0x10, 0x1A0, 0x48,   0x10,  0x390, 0x0, 0x0, 0x0],
                        2 => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x6F0, 0x0,   0x0,  0x0],
                        4 => &[0x8CE220, 0x10,  0x1A0,  0x48,   0x10,  0x310, 0x0,   0x0,  0x0],
                        _ => null
                    }
                    _ => null
                });

                let mut text_ptr2 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        "Demo v1.19" => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x6D0, 0x0, 0x0, 0x0],
                        "CH1-4 v1.02" => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x700, 0x0,   0x0,  0x0],
                        "CH1-5 v244" => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x700, 0x0,   0x0,  0x0],
                        _ => null
                    }
                    _ => null
                });
                let mut text_ptr3 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        "Demo v1.19" => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x6F0, 0x0, 0x0, 0x0],
                        "CH1-4 v1.02" => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x710, 0x0,   0x0,  0x0],
                        "CH1-5 v244" => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x710, 0x0,   0x0,  0x0],
                        _ => null
                    }
                    _ => null
                });
                let mut text_ptr4 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        "CH1-4 v1.02" => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x7E0, 0x0,   0x0,  0x0],
                        "CH1-5 v244" => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x7E0, 0x0,   0x0,  0x0],
                        _ => null
                    }
                    _ => null
                });


                let mut susie_sprite_ptr = VarTrack::<i32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x1008, 0x50,   0x158, 0x10,  0xBC],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1018, 0x50,   0x158, 0x10,  0xBC],
                        _ => null
                    }
                    _ => null
                });

                let mut player_x_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x198,  0x0,    0x50,  0x158, 0x10,  0xE8],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1A8,  0x0,    0x50,  0x158, 0x10,  0xE8],
                        _ => null
                    }
                    _ => null
                });

                let mut player_y_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x198,  0x0,    0x50,  0x158, 0x10,  0xEC],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1A8,  0x0,    0x50,  0x158, 0x10,  0xEC],
                        _ => null
                    }
                    _ => null
                });



                //Ch1 objects

                let mut great_door_con_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0xC,  0x60, 0x10, 0x10,  0x0],
                    "Demo v1.09" | "Demo v1.10" => &[0x6EF220, 0x84, 0x24,  0x10, 0x18,  0x0],
                    "Demo v1.15" => &[0x6F0B48, 0x84, 0x24,  0x10, 0x18, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x8B2790, 0xE0,  0x48,  0x10, 0x0,   0x0],
                        _ => null
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x8B2790, 0xE0, 0x48,  0x10,   0x30,  0x0],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x8BA790, 0xE0, 0x48,  0x10,   0x30,  0x0],
                        _ => null
                    }
                    _ => null
                });

                let mut king_pos_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x6AEB80, 0x4, 0x178, 0x80, 0xC8, 0x8, 0xB4],
                    "Demo v1.09" | "Demo v1.10" => &[0x6F1394, 0x4, 0x140, 0x68, 0x3C, 0x8, 0xB0],
                    "Demo v1.15" => &[0x6F2CBC, 0x4, 0x140, 0x68, 0x3C, 0x8, 0xB0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x69FA98, 0x0, 0x530, 0x50, 0x158, 0x10, 0xE8],
                        _ => null
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x69FA98, 0x0,  0x560, 0x50,   0x158, 0x10,  0xE8],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x6A7A98, 0x0,  0x560, 0x50,   0x158, 0x10,  0xE8],
                        _ => null
                    }
                    _ => null
                });



                //SP-specific object checks

                let mut jevil_dance_ptr1 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x78, 0x60, 0x10, 0x10,  0x0],
                    _ => null
                });
                let mut jevil_dance_ptr2 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x7C, 0x60, 0x10, 0x10,  0x0],
                    _ => null
                });
                let mut final_textbox_ptr1 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x98, 0x60, 0x10, 0x274, 0x0],
                    _ => null
                });
                let mut final_textbox_ptr2 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x9C, 0x60, 0x10, 0x274, 0x0],
                    _ => null
                });



                //Ch2 objects

                let mut loaded_disk_bg_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" => &[0x6EF220, 0x84, 0x24,  0x10, 0x3D8, 0x0],
                    "Demo v1.10" => &[0x6EF220, 0x84, 0x24,  0x10, 0x87C, 0x0],
                    "Demo v1.15" => &[0x6F0B48, 0x84, 0x24,  0x10, 0x0,  0x0],
                    "Demo v1.19" => match chapter {
                        2 => &[0x8B2790, 0xE0,  0x48,  0x10, 0x3C0, 0x0],
                        _ => null
                    }
                    "Ch1-4 v1.02" => match chapter {
                        2 => &[0x8B2790, 0xE0,  0x48,  0x10,   0xC70, 0x0],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        2 => &[0x8BA790, 0xE0,  0x48,  0x10,   0xCA0, 0x0],
                        _ => null
                    }
                    _ => null
                });

                let mut snowgrave_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x6EF220, 0xF4, 0x27C, 0x6C, 0x5C,  0x20, 0x144, 0x24, 0x10, 0xC0, 0x0],
                    "Demo v1.15" => &[0x6F0B48, 0xF4, 0x27C, 0x6C, 0x5C, 0x20, 0x144, 0x24, 0x10, 0x120, 0x0],
                    "Demo v1.19" => match chapter {
                        2 => &[0x8B2790, 0x1A0, 0x3B0, 0x88, 0x70,  0x38, 0x1A0, 0x48, 0x10, 0x3D0, 0x0],
                        _ => null
                    }
                    "CH1-4 v1.02" => match chapter {
                        2 => &[0x8B2790, 0x1A0, 0x3B0, 0x88,   0x70,  0x38,  0x1A0, 0x48, 0x10, 0xA0, 0x0],
                        _ => null
                    }
                    "CH1-5 v244" => match chapter {
                        2 => &[0x8BA790, 0x1A0, 0x3B0, 0x88,   0x70,  0x38,  0x1A0, 0x48, 0x10, 0x80, 0x0],
                        _ => null
                    }
                    _ => null
                });





                //Ch3 objects

                let mut egg_timer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        "CH1-4 v1.02" => &[0x8B2790, 0x1E8, 0x530,  0x38,   0x48, 0x10, 0x290, 0x0],
                        "CH1-5 v244" => &[0x8BA790, 0x1E8, 0x40,   0x38,   0x48, 0x10, 0x330, 0x0],
                        _ => null
                    }
                    _ => null
                });
                let mut mantle_outro_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x19B0, 0x18,   0x50, 0x10, 0xD0],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x19B0, 0x18,   0x50, 0x10, 0xD0],
                        _ => null
                    }
                    _ => null
                });



                //Ch4 objects

                let mut mike_action_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x8B2790, 0x1A0, 0x2F0,  0x90,   0x78,  0x38,  0x198, 0x48, 0x10, 0x140, 0x0],
                        "CH1-5 v244" => &[],
                        _ => null
                    }
                    _ => null
                });


                //Ch5 objects

                let mut crt_start_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    5 => match version {
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1910, 0x8,    0x18, 0x68, 0x10,  0xE4],
                        _ => null
                    }
                    _ => null
                });







                // TODO: Load some initial information from the process.
                loop {
                    settings.update();


                    asr::timer::set_variable_int("Chapter",chapter);

                    let room = room_watch.update(process.read::<i32>(room_id_addr).ok())
                        .unwrap_or(&Pair { old: -1i32, current: -1i32 });
                    asr::timer::set_variable_int("room",room.current);


                    //let room_name_addr0 = room_array_addr.add(process.read::<i32>(room_id_addr.value() * match ps { ps64 => 8, _ => 4}).unwrap_or_default() as u64);
                    let room_name_addr0 = process.read_pointer(room_array_addr,ps).unwrap_or_default().add(room.current as u64 * match ps { ps64 => 8, _ => 4});
                    let room_name_addr = process.read_pointer(room_name_addr0,ps).unwrap_or_default();
                    let room_name = room_name_watch.update_infallible(
                        process.read::<ArrayCString<64>>
                        (room_name_addr).unwrap_or_default());
                    asr::timer::set_variable("Room Name Pointer Address",format!("{:X}",room_name_addr0.value()).as_str());
                    asr::timer::set_variable("Room Name Address",format!("{:X}",room_name_addr.value()).as_str());
                    asr::timer::set_variable("room_name",room_name.current.validate_utf8().unwrap_or_default());


                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
