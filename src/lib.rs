#![allow(nonstandard_style)]

use std::{fs::read};
use asr::{
    Address, PointerSize, Process, file_format::pe, future::{next_tick, retry}, settings::Gui,
    signature::Signature, string::ArrayCString, time_util::Instant,
    timer::{self, TimerState}, watcher::{Pair, Watcher}
};
use std::collections::{HashMap, HashSet};
use self::{EngineVersion::*, GameVersion::*};

mod settings;
mod helpers;
mod item_tracking;

use settings::*;
use helpers::*;
use item_tracking::{*,ItemType::{Weapon,Armor,KeyItem,ItemLW}};

asr::async_main!(stable);

enum GameVersion {
    Invalid,
    SP, //original Ch1-only release, 
    D109, //version with Save Wrong Warping
    D110, //last non-beta Demo version on Steam
    D115, //last 32-bit release, last release of pre-change_game beta branch on Steam
    D119, //latest Demo version on Steam and only version on Itch.io
    Ch4_v102, //for Ch3 and hypothetical patch-swap runs (Tenna fast static)
    Ch5_v244, //for All Items and literal-ASC (JusticeAxe exploit. Note that the ASC category on the boards doesn't allow using this exploit)
    Ch5_v247 //currently latest version
}

//NOTE: LTS 2022.0 branched from Monthly 2022.9, so LTS 2022.0.__ is strictly newer than Monthly 2022.2
//specific version numbers taken from OpenGM's GitHub page
//Also, GameMaker used the "GameMaker Studio 2" branding up to and including version 2022.2, with 2022.3 changing the name of the engine back to simply "GameMaker"
#[derive(Copy,Clone)]
enum EngineVersion {
    GMS2_v2_2_0, //SURVEY_PROGRAM
    GMS2_2022_1, //Demo 1.09 and 1.10 (presumably also 1.08)
    GMS2_2022_2, //Demo 1.15 (presumably the whole 1.12-1.15 branch)
    GM_LTS2022_0_3_99, //Demo 1.19 and Ch1-4 v1.02 (everything from Demo 1.19 [potentially even back to 1.17 or 1.16] to Ch1-4 v1.04)
    GM_LTS2022_0_3_104, //Ch1-5 v244 and v247 (everything from Ch1-4 v1.05 beta to now)
}



const n : &[u64] = &[0];

const ps64: PointerSize = PointerSize::Bit64;
const ps32: PointerSize = PointerSize::Bit32;

const IL_Pauses : [&str;5] = ["ch1_ending","ch2_ending_il","ch3_ending","ch4_ending_il","ch5_ending_src"];
const AC_Pauses : [&str;5] = ["ch1_ending","ch2_ending_ac","ch3_ending","ch4_ending","ch5_ending_src"]; //Ch5 ending in this set will change after Ch6 release
const OST_Pauses : [&str;5] = ["ch1_ending_ost","ch2_ending_ac","ch3_ending_ost","ch4_ending_ost","ch5_ending_src"];
const OST_LateCh2_Pauses : [&str;5] = ["ch1_ending_ost","ch2_ending_ost","ch3_ending","ch4_ending_ost","ch5_ending_src"];



async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();
    let mut splits = HashSet::<String>::new();
    let item_map = HashMap::from(item_map_init_array());
    let mut item_tracker = HashSet::<Item>::new();
    asr::set_tick_rate(60.0);

    asr::print_message("Hello, World!");

    loop {
        settings.update();
        let process = Process::wait_attach("DELTARUNE.exe").await;
        let Ok(mut path) = process.get_module_path("DELTARUNE.exe") else {
            next_tick().await;
            continue;
        };

        process
            .until_closes(async {
                let attached = Instant::now();
                let (DELTARUNE, _) = process.wait_module_range("DELTARUNE.exe").await;
                let module_size = pe::read_size_of_image(&process, DELTARUNE).unwrap_or_default();
                timer::set_variable("Module Address",format!("{:X}",DELTARUNE.value()).as_str());


                let runner_md5 = &format!("{:X}", md5::compute(read(&path).unwrap_or_default()));
                timer::set_variable("Runner MD5", runner_md5);
                let version = match runner_md5.to_lowercase().as_str() {
                    "4d09627e1fa123d12ddf1a496c489f73" => GMS2_v2_2_0,
                    "dcfb86f7a80d9906bbbafa1b2c224848" => GMS2_2022_1,
                    "a9db8b7fb6333b5e267f574f46076b3f" => GMS2_2022_2,
                    "14af94e0435eb4cbe3bb5a03ab4218c4" => GM_LTS2022_0_3_99,
                    "7bf3cccc2e54481ced3a149e1a083684" => GM_LTS2022_0_3_104,
                    _ => {
                        timer::set_variable("Engine Version","Unrecognized");
                        timer::set_variable("Game Version","invalid or unrecognized");
                        asr::print_message("Unrecognized GameMaker runner, autosplitter cannot function with this");
                        loop {next_tick().await;}
                    },
                };
                timer::set_variable("Engine Version", match version {
                    GMS2_v2_2_0 => "GMS2 v2.2.0 (32-bit)",
                    GMS2_2022_1 => "GMS2 Monthly 2022.1.0.482 (32-bit)",
                    GMS2_2022_2 => "GMS2 Monthly 2022.2 (32-bit)",
                    GM_LTS2022_0_3_99 => "GameMaker LTS 2022.0.3.99 (64-bit)",
                    GM_LTS2022_0_3_104 => "GameMaker LTS 2022.0.3.104 (64-bit)",
                });


                path = path.replace("DELTARUNE.exe", "data.win");
                timer::set_variable("Path", path.as_str());
                let data_md5 = &format!("{:X}", md5::compute(read(path).unwrap_or_default()));
                timer::set_variable("data.win MD5", data_md5);
                
                let game_version = match data_md5.to_lowercase().as_str() {
                    "a88a2db3a68c714ca2b1ff57ac08a032" | //SP-EN vanilla
                    "047c11435b1c592ec731bff3b9c5b0cf" | //SP-EN 30tbps
                    "22008370824a37baef8948127963c769" | //SP-JP vanilla
                    "e05433fe679bc91e3809c1138e3a8ea1" => SP, //SP-JP 30tbps
                    "616c5751ac9fc584af250f1b04474afd" | //demo 1.09 vanilla Itch
                    "05689183497e58838e99b897f2e0e6ac" | //demo 1.09 30tbps Itch
                    "267a8abe468d824222810201f00003be" | //demo 1.09 vanilla Steam
                    "272a16964597ed6dc8d2393ed051d3ce" => D109, // demo 1.09 30tbps Steam
                    "5fbe01f2bc1c04f45d809ffd060ac386" | //demo 1.10 vanilla Itch
                    "a37c77a4310d2d6a6c2af18294aaae7a" | //demo 1.10 30tbps Itch
                    "cd77a63d7902990dbc704fe32b30700a" | //demo 1.10 vanilla Steam
                    "758c8862f22f778fdeafe25fbcd1f4ec" => D110, //demo 1.10 30tbps Steam
                    "ed4568bab864166bfd6322ceeb3fb544" | //demo 1.15 vanilla
                    "6bd6d1381c194c0f456b184cb48d132d" => D115, //demo 1.15 30tbps
                    "7ad299a8b33fa449e20edfe0fededdb2" | //demo 1.19 vanilla
                    "fd0857e6a3af3aa74e5e00f15aea5224" => D119, //demo 1.19 30tbps
                    "b5ef0eec9554c491777d6c4e93e0df76" | //v1.02 vanilla
                    "40a8185886a8a1a2be996bc57de3d916" => Ch4_v102, //v1.02 30tbps
                    "ddedbbd10ff129b49c64dbefaa763c6a" | //v244 vanilla
                    "4a9c69b42e442b673395b3253f292f17" | // v244 30 TBPS mod
                    "42b66b41b6cea12fb54219e9d31e5dc8" | // v244 Item tracker mod
                    "d0420c09a5debd6176ea24a1fe1ee3e3" => Ch5_v244, // OST% tracker mod
                    "908643b7593b000f5b6c61bb484d086a" | //v247 vanilla
                    "80a63475ef69529b612f9dca75af4cc5" | //v247 30tbps
                    "3217f3bfe82c3e4aa8ee2e9e3a4f4e14" | //v247 Item Tracker
                    "21cdd09eeadbcc77535ab2bb3412259a" => Ch5_v247, //v247 OST tracker
                    _ => Invalid,
                };
                timer::set_variable("Game Version", match game_version {
                    Invalid => "invalid or unrecognized",
                    SP => "SURVEY_PROGRAM",
                    D109 => "Demo v1.09",
                    D110 => "Demo v1.10",
                    D115 => "Demo v1.15",
                    D119 => "Demo v1.19",
                    Ch4_v102 => "Ch1-4 v1.02",
                    Ch5_v244 => "Ch1-5, Ch5 v0.0.244",
                    Ch5_v247 => "Ch1-5, Ch5 v0.0.247",
                });
                if !settings.allow_unsupported_version && matches!(game_version,Invalid) {
                    loop {
                        settings.update();
                        if settings.allow_unsupported_version { break; }
                        next_tick().await;
                    }
                }

                let ps = match version {
                    GMS2_v2_2_0 | GMS2_2022_1 | GMS2_2022_2 => ps32,
                    _ => ps64
                };

                /*let varJump = match ps {
                    ps64 => 0x10,
                    ps32 => 0xC, //seems oddly inaccurate? it's as if the jump betweens vars is LARGER on v1.10???? (NOTE: it might just be that there are empty slots for variables)
                    _ => unreachable!()
                };*/

                let psBytes = match ps {
                    ps64 => 0x8,
                    ps32 => 0x4,
                    _ => unreachable!()
                };

                /*let strNumOffset = match ps {
                    ps64 => -0x18,
                    ps32 => -0xC,
                    _ => unreachable!()
                };*/

                let mut chapter = -1;

                if ps == ps64
                { //working_directory only changes with change_game which starts a whole new process for the autosplitter to attach to, so we only need to read it once per process attached
                    let mut _dir : ArrayCString<256>;
                    loop {
                        _dir = process.read_pointer_path::<ArrayCString<256>>(DELTARUNE, ps, match version {
                            GMS2_v2_2_0 | GMS2_2022_1 | GMS2_2022_2 => unreachable!(), //we shouldn't be at this part of the code with those versions
                            GM_LTS2022_0_3_99 => &[0x8B2818,0],
                            GM_LTS2022_0_3_104 => &[0x8BA818,0],
                        }).unwrap_or_default();
                        if _dir != ArrayCString::<256>::default() {
                            break;
                        }
                        next_tick().await;
                    }
                    let dir = _dir.validate_utf8().unwrap_or("invalid UTF8");
                    timer::set_variable("dir", dir);
                    if dir.ends_with(r"chapter1_windows\") { chapter = 1 }
                    else if dir.ends_with(r"chapter2_windows\") { chapter = 2 }
                    else if dir.ends_with(r"chapter3_windows\") { chapter = 3 }
                    else if dir.ends_with(r"chapter4_windows\") { chapter = 4 }
                    else if dir.ends_with(r"chapter5_windows\") { chapter = 5 }
                    else { chapter = 0 }
                } else if matches!(version,GMS2_v2_2_0) {
                    chapter = 1;
                }
                timer::set_variable_int("Chapter",chapter);


                //rooms

                static array_sig64 : Signature<13> = Signature::new(&"74 0C 48 8B 05 ?? ?? ?? ?? 48 8B 04 D0");
                static array_sig32 : Signature<8> = Signature::new(&"8B 3D ?? ?? ?? ?? 2B EF");

                static id_sig64 : Signature<23> = Signature::new(&"48 ?? ?? ?? 3B 35 ?? ?? ?? ?? 41 ?? ?? ?? 49 ?? ?? E8 ?? ?? ?? ?? FF");
                static id_sig32 : Signature<16> = Signature::new(&"FF 35 ?? ?? ?? ?? E8 ?? ?? ?? ?? 83 C4 04 50 68");

                let mut room_array_addr = match ps {
                    ps64 => array_sig64.wait_scan_process_range(&process, (DELTARUNE, module_size as u64)).await.add(5),
                    _ => process.read_pointer(array_sig32.wait_scan_process_range(&process,(DELTARUNE, module_size as u64)).await.add(2), ps).unwrap_or_default()
                };
                room_array_addr = match ps {
                    ps64 => room_array_addr.add(process.read::<i32>(room_array_addr).unwrap_or_default() as u64 + 4),
                    _ => room_array_addr
                };
                timer::set_variable("Room Array Address",format!("{:X}",room_array_addr.value()).as_str());

                /*let room_id_addr = match ps {
                    PointerSize::Bit64 => {
                        let addr1 = id_sig64.wait_scan_process_range(&process,(DELTARUNE,module_size)).await.add(6);
                        return addr1.add(process.read::<i32>(addr1).unwrap_or_default() as u64 + 4);
                    },
                    _ => process.read_pointer(id_sig32.wait_scan_process_range(&process,(DELTARUNE,module_size)).await,ps).unwrap_or_default().add(2)
                };*/
                let mut room_id_addr = match ps {
                    ps64 => id_sig64.wait_scan_process_range(&process, (DELTARUNE, module_size as u64)).await.add(6),
                    _ => process.read_pointer(id_sig32.wait_scan_process_range(&process,(DELTARUNE, module_size as u64)).await.add(2), ps).unwrap_or_default()
                };
                room_id_addr = match ps {
                    ps64 => room_id_addr.add(process.read::<i32>(room_id_addr).unwrap_or_default() as u64 + 4),
                    _ => room_id_addr
                };
                timer::set_variable("Room ID Address",format!("{:X}",room_id_addr.value()).as_str());

                let mut room_watch = Watcher::<i32>::new();
                let mut room_name_watch = Watcher::<ArrayCString<64>>::new();

                //temporary unreachables for pointers I haven't found yet
                let stringsListOffset = match version {
                    GMS2_v2_2_0 => 0x3EAE58,
                    GMS2_2022_1 => 0x43EA88,
                    GMS2_2022_2 => 0x440AA8,
                    GM_LTS2022_0_3_99 => 0x5F4CF8,
                    GM_LTS2022_0_3_104 => 0x5FCD08,
                };

                //let stringsListTiming = Instant::now();
                let mut stringsList = HashMap::<u32,String>::new();
                retry(|| -> Option<bool> {
                    let Ok(sListPtr) = process.read_pointer(DELTARUNE.add(stringsListOffset),ps) else { return None; };
                    //the number of strings is always found at -0xC from the main array pointer regardless of 32-bit/64-bit
                    let Ok(strNum) = process.read::<u32>(DELTARUNE.add(stringsListOffset - 0xC)) else { return None; }; 
                    asr::print_message(format!("StringsList length: {}",strNum).as_str());
                    for i in 0..strNum {
                        let Ok(namePtr) = process.read_pointer(sListPtr.add(psBytes*i as u64), ps) else {
                            continue;
                        };
                        if namePtr.is_null() {  continue; }
                        let _name = process.read::<ArrayCString<64>>(namePtr).unwrap_or_default();
                        let name = _name.validate_utf8().unwrap_or_default();
                        if name != "" {
                            stringsList.insert(i + match version {GMS2_v2_2_0=>0,_=>100000}, name.to_string());

                            /*if matches!(name,"plot"|"mystring"|"flag"|"item"|"litem") {
                                asr::print_limited::<64>(&format_args!("{} found at StringID {}",name,i))
                            }
                            timer::set_variable_int("last real string index",i);*/
                        }
                    }
                    //in SP, the pointers to names of global variables are in a separate array from the ones for other objects' variables, but there is no overlap so we can just read both into one HashMap
                    if matches!(version,GMS2_v2_2_0) {
                        let Ok(sListPtr2) = process.read_pointer(DELTARUNE.add(stringsListOffset-0x10),ps) else { return None; };
                        //let strNum = process.read::<u32>(sListPtr.add_signed(strNumOffset)).unwrap_or_default(); //there doesn't seem to be a real length value anywhere around here
                        //asr::print_message(format!("StringsList length: {}",strNum).as_str());
                        for i in 0..strNum {
                            let Ok(namePtr) = process.read_pointer(sListPtr2.add(psBytes*i as u64), ps) else {
                                continue;
                            };
                            if namePtr.is_null() {  continue; }
                            let _name = process.read::<ArrayCString<64>>(namePtr).unwrap_or_default();
                            let name = _name.validate_utf8().unwrap_or_default();
                            if name != "" {
                                /*stringsList.insert(i, name.to_string());
                                if matches!(name,"plot"|"mystring"|"flag"|"item"|"litem") {
                                    asr::print_limited::<64>(&format_args!("{} found at StringID {}",name,i))
                                }
                                timer::set_variable_int("last real string index",i);*/
                            }
                        }
                    }
                    if (stringsList.len() as u32) < (strNum * 3 / 4) { return None; }
                    return Some(true);
                }).await;
                //asr::print_message(format!("StringsList read in {} seconds",stringsListTiming.elapsed().as_secs_f64()).as_str());
                asr::print_message(format!("Number of strings: {}",stringsList.len()).as_str());
                //asr::print_message(format!("plot's String index is {}",string_ids["plot"]).as_str());s

                //temporary unreachables for pointers I haven't found yet
                let objArrOffset = match version {
                    GMS2_v2_2_0 => 0x49A5C8,
                    GMS2_2022_1 => 0x4DCCEC,
                    GMS2_2022_2 => 0x4DE60C,
                    GM_LTS2022_0_3_99 => 0x69FA98,
                    GM_LTS2022_0_3_104 => 0x6A7A98,
                };

                let objPropOff = match ps {
                    ps64 => 0x18,
                    ps32 => 0xC,
                    _ => unreachable!()
                };

                let objNumOff = match ps {
                    ps64 => 0xC,
                    ps32 => 0x8,
                    _ => unreachable!()
                };

                let mut obj_addr_map = HashMap::<String,Address>::new();
                retry(|| -> Option<bool> {
                    //for testing we temporarily skip object reading in versions that don't have offsets found yet
                    let Ok(objArrBase) = process.read_pointer(DELTARUNE.add(objArrOffset),ps) else { return None; };
                    let Ok(objNum) = process.read::<u32>(objArrBase.add(objNumOff)) else {
                        return None;
                    };
                    if objNum == 0 { return None; }
                    asr::print_message(format!("Number of objects: {}",objNum).as_str());
                    let Ok(arr) = process.read_pointer(objArrBase,ps) else { return None; };
                    for i in 0..1024u64 { //might be higher in SP? Seems to go up to 2820. However there are only 342 objects in SP so I doubt it'd matter
                        let Ok(mut objAddr) = process.read_pointer(arr.add(psBytes as u64*i),ps) else { return None; };
                        for _layer in 1..=10 {
                            if objAddr.is_null() { break; }
                            let _name = process.read_pointer_path::<ArrayCString<64>>(objAddr,ps,&[objPropOff,match version { GMS2_v2_2_0 => 0x14, _ => 0 },0x0]).unwrap_or_default();
                            let name = _name.validate_utf8().unwrap_or_default();
                            if name != "" {
                                /*if matches!(name,"obj_writer"|"obj_moneydisplay"|"DEVICE_NAMER"|"obj_berdly_smoke") {
                                    asr::print_message(format!("{} found at index {} layer {}, address {}",name,i,_layer,objAddr).as_str());
                                }*/
                                obj_addr_map.insert(name.to_string(),objAddr);
                            }
                            objAddr = process.read_pointer(objAddr,ps).unwrap_or_default();
                        }
                    }
                    //if a significant amount of objects were missed
                    if (obj_addr_map.len() as u32) < (objNum * 3 / 4) { obj_addr_map.clear(); return None; }
                    return Some(true);
                }).await;
                asr::print_message(format!("objs successfully found: {}",obj_addr_map.len()).as_str());

                
                let mantleOutro_instCount = match chapter {
                    3 => Address::from(process.read_pointer_path::<u64>(get_obj(&obj_addr_map,"obj_shadow_mantle_enemy_outro"),ps,&[0x18,0x78]).unwrap_or_default()),
                    _ => Address::NULL
                };
                let crtEnd_instCount = match chapter {
                    5 => Address::from(process.read_pointer_path::<u64>(get_obj(&obj_addr_map,"obj_LW20W_end"),ps,&[0x18,0x78]).unwrap_or_default()),
                    _ => Address::NULL
                };
                /*let mut objs = String::from("");
                for k in obj_addr_map.keys() {
                    objs += k.as_str();
                    objs += ", ";
                }
                asr::print_message(&objs);*/


                
                /*let globalOffset : u64 = match version {
                    GMS2_v2_2_0 => 0x49C3E0, //0x48E5DC,
                    GMS2_2022_1 => 0x6FCF38,
                    GMS2_2022_2 => 0x6FE860,
                    GM_LTS2022_0_3_99 => 0x6A1CA8,
                    GM_LTS2022_0_3_104 => 0x6A9CA8,
                };*/

                let mut global = retry(|| global_setup(&process, DELTARUNE, version, ps)).await;
                asr::print_message(&format!("Found global, array starts at {}",global.finder.arrAddr));



                //simple constant-address watchers
                let mut _plot = Watcher::<f64>::new();
                let mut _choice = Watcher::<f64>::new();
                let mut _msc = Watcher::<f64>::new();
                let mut _fighting = Watcher::<f64>::new();
                let mut _chapter = Watcher::<f64>::new();
                let mut _darkzone = Watcher::<f64>::new();


                //watchers for object instance variables, strings, values in arrays, etc.
                //the same watcher may be used for different variables depending on the chapter and room
                let mut _namer = Watcher::<f64>::new();
                let mut _con = Watcher::<f64>::new();
                //let mut _posX = Watcher::<f32>::new();
                //let mut _posY = Watcher::<f32>::new();
                let mut _text_check = Watcher::<bool>::new();
                let mut _text_check2 = Watcher::<bool>::new();
                let mut _flag = Watcher::<f64>::new();
                let mut _instExist = Watcher::<bool>::new();
                //let mut _instBool = Watcher::<bool>::new();


                // sound stuff (pointer only varies by runner version)

                let mut snd_ptr = PathTrack::<ArrayCString<256>>::new(DELTARUNE, ps, match version {
                    GMS2_v2_2_0 => n,
                    GMS2_2022_1 => &[0x4E0794, 0x58, 0xC0,  0x40, 0x0],
                    GMS2_2022_2 => &[0x4E20B4, 0x58, 0xC0,  0x40, 0x0],
                    GM_LTS2022_0_3_99 => &[0x6A3818, 0x60, 0xD0, 0x58, 0x0],
                    GM_LTS2022_0_3_104 => &[0x6AB818, 0x60, 0xD0, 0x58, 0x0],
                });

                let mut mus_ptr = PathTrack::<ArrayCString<256>>::new(DELTARUNE, ps, match version {
                    GMS2_v2_2_0 => n,
                    GMS2_2022_1 => &[0x4DFF58, 0x0,  0x44,  0x0],
                    GMS2_2022_2 => &[0x4E1878, 0x0,  0x0,   0x0],
                    GM_LTS2022_0_3_99 => &[0x6A2F90, 0x0,  0x0,  0x0],
                    GM_LTS2022_0_3_104 => &[0x6AAF90, 0x0,  0x0,  0x0],
                });

                //const items_goal : usize = 160;

                //some helpful closures
                let chapter1ify = |name : &str| match version {
                    GMS2_2022_1 | GMS2_2022_2 => name.to_owned() + "_ch1",
                    _ => name.to_owned()
                };

                let objVar = |obj : &str, var : &str| get_obj_var(&process, version, &obj_addr_map, &stringsList, obj, var);

                let arrItem = | arr : Address, index : u64 | process.read::<f64>(arr.add(index*0x10)).unwrap_or_default();
                
                let arrCheck = | arr, val, indexStart, indexEnd | -> bool {
                    for i in indexStart..=indexEnd {
                        if arrItem(arr,i) == val {
                            return true;
                        }
                    }
                    false
                };

                let textCheck = |en,jp| check_text(&process,version,&stringsList,get_obj(&obj_addr_map,"obj_writer"),en,jp);


                let mut global_effectiveness_retry_timer: i32 = 5;


                asr::print_message(format!("ready for continuous logic after {} seconds",attached.elapsed().as_secs_f64()).as_str());
                // TODO: Load some initial information from the process.
                loop {
                    if !process.is_open() { break; }
                    settings.update();
                    if !settings.allow_unsupported_version && matches!(game_version,Invalid) {
                        next_tick().await;
                        continue;
                    }

                    if chapter != 0 && global.cache.is_empty() {
                        global_effectiveness_retry_timer -= 1;
                        if global_effectiveness_retry_timer <= 0 {
                            global_effectiveness_retry_timer = 5;
                            global = retry(|| global_setup(&process, DELTARUNE, version, ps)).await;
                        }
                    }


                    /*if chapter > 2 && flag0Ptr.is_null() {
                        flag0Ptr = get_array_element0(&process,ps,&global_ptr(&process,&stringsList,&globalFinder,&mut globalPtrs,"flag"));
                    }*/

                    if matches!(version,GMS2_2022_1|GMS2_2022_2) {
                        chapter = _chapter.update_infallible(global.num(&process,&stringsList,"chapter")).current as i32;
                    }
                    timer::set_variable_int("Chapter",chapter);

                    let room_id = room_watch.update(process.read::<i32>(room_id_addr).ok())
                        .unwrap_or(&Pair { old: 0i32, current: 0i32 });
                    timer::set_variable_int("Room ID", room_id.current);

                    let room_name_addr0 = process.read_pointer(room_array_addr,ps).unwrap_or_default().add(room_id.current as u64 * match ps { ps64 => 8, _ => 4});
                    let room_name_addr = process.read_pointer(room_name_addr0,ps).unwrap_or_default();
                    let room = room_name_watch.update_infallible(
                        process.read::<ArrayCString<64>>
                        (room_name_addr).unwrap_or_default());

                    let cur_room = room.current.validate_utf8().unwrap_or_default().trim_end_matches("_ch1");
                    let prev_room = room.old.validate_utf8().unwrap_or_default().trim_end_matches("_ch1");
                    //asr::timer::set_variable("Room Name Pointer Address",format!("{:X}",room_name_addr0.value()).as_str());
                    //asr::timer::set_variable("Room Name Address",format!("{:X}",room_name_addr.value()).as_str());
                    timer::set_variable("Room Name",cur_room);

                    timer::set_variable("text",get_obj_str::<128>(&process, version, &obj_addr_map, &stringsList, "obj_writer", "mystring").validate_utf8().unwrap_or_default()); //
                    timer::set_variable("writer addr",format!("{}",obj_addr_map.get(&"obj_writer".to_owned()).unwrap_or(&Address::NULL)).as_str());

                    //timer::set_variable_float("Plot",globalFinder.readNum::<f64>(&process, &stringsList, "plot"));


                    let state = timer::state();

                    if state == TimerState::NotRunning || state == TimerState::Ended {
                        //tempVar = 0;
                        if !splits.is_empty() { splits.clear(); }
                        if !item_tracker.is_empty() { item_tracker.clear(); }
                    }

                    match chapter {
                        //logic for autostart, autoreset, and continuing game time
                        0 => (),
                        1 => {
                            if prev_room != cur_room && cur_room == "PLACE_CONTACT" {
                                //tempVar = 0;
                                start(&settings.auto_start,&mut splits,&mut item_tracker);
                            }
                        }
                        5 if prev_room == "PLACE_CONTACT" => {
                            let namer_event = _namer.update_infallible(objVar("DEVICE_NAMER","EVENT")); //get_obj_var::<f64>(&process,ps,&obj_addr_map,&stringsList,"DEVICE_NAMER","EVENT")
                            timer::set_variable_float("Namer Event",namer_event.current);
                            if !matches!(settings.ch5_start_on_prev,Ch5StartOnPrev::Exclusively) {
                                if cur_room == "PLACE_MENU"
                                {
                                    if namer_event.current == 75.0 && namer_event.old == 74.0 {
                                        start(&settings.auto_start,&mut splits,&mut item_tracker);
                                    }
                                }
                            }
                            if !matches!(settings.ch5_start_on_prev,Ch5StartOnPrev::No) {
                                if prev_room == "PLACE_MENU" && cur_room == "room_krisroom" && namer_event.old != 75.0 {
                                    start(&settings.auto_start,&mut splits,&mut item_tracker);
                                }
                            }
                        }
                        _ if prev_room == "PLACE_MENU" => {
                            let namer_event = _namer.update_infallible(objVar("DEVICE_NAMER","EVENT")); //get_obj_var::<f64>(&process,ps,&obj_addr_map,&stringsList,"DEVICE_NAMER","EVENT")
                            timer::set_variable_float("Namer Event",namer_event.current);
                            if namer_event.current == 75.0 && namer_event.old != 75.0 {
                                start(&settings.auto_start,&mut splits,&mut item_tracker);
                            }
                        }
                        _ => ()
                    }
                    timer::set_variable("litem address",format!("{}",global.ptr(&process,&stringsList,"litem[0]")).as_str());


                    // if we're not in the middle of a run, no reason to do anything not related to autostart (note that IGT pauses don't affect whether the timer state counts as running or paused)
                    if timer::state() == TimerState::Running && chapter > 0 {
                        if settings.item_tracking && chapter > 0 && cur_room != "PLACE_MENU" {

                            let mut itemCheck = |itemType,inv,offset| item_check_slot(&process,&mut item_tracker,&item_map,chapter,itemType,global.ptr(&process,&stringsList,inv).add(offset));
                            /*match darkzone.current {
                                1.0 => {
                                    for i in 0..12 as u64 {
                                        let offset = i * 0x10;
                                        item_check_slot(&process,&mut item_tracker,&item_map,chapter,ItemType::Item,globPtr("item[0]").add(offset));
                                        item_check_slot(&process,&mut item_tracker,&item_map,chapter,KeyItem,globPtr("keyitem[0]").add(offset));
                                        item_check_slot(&process,&mut item_tracker,&item_map,chapter,Weapon,globPtr("weapon[0]").add(offset));
                                        item_check_slot(&process,&mut item_tracker,&item_map,chapter,Armor,globPtr("armor[0]").add(offset));
                                        if chapter > 1 {
                                            item_check_slot(&process,&mut item_tracker,&item_map,chapter,ItemType::Item,globPtr("pocketitem[0]").add(offset));
                                        }
                                    }
                                    if chapter > 1 {
                                        for i in 12..48 as u64 {
                                            let offset = i * 0x10;
                                            item_check_slot(&process,&mut item_tracker,&item_map,chapter,Weapon,globPtr("weapon[0]").add(offset));
                                            item_check_slot(&process,&mut item_tracker,&item_map,chapter,Armor,globPtr("armor[0]").add(offset));
                                            if (i < match chapter { 2|3 => 24, 4 => 36, 5 => 48, _ => unreachable!()}) {
                                                item_check_slot(&process,&mut item_tracker,&item_map,chapter,ItemType::Item,globPtr("pocketitem[0]").add(offset));
                                            }
                                        }
                                    }
                                }
                                0.0 => {
                                    for i in 0..12 as u64 {
                                            let offset = i * 0x10;
                                            item_check_slot(&process,&mut item_tracker,&item_map,chapter,ItemLW,globPtr("litem[0]").add(offset));
                                    }
                                }
                                _ => () //should always be 1 or 0
                            }*/
                            for offset in (0..arr_pos(12)).step_by(0x10) {
                                if offset < arr_pos(8) {
                                    itemCheck(ItemLW,"litem[0]",offset);
                                }
                                itemCheck(ItemType::Item,"item[0]",offset);
                                itemCheck(KeyItem,"keyitem[0]",offset);
                                itemCheck(Weapon,"weapon[0]",offset);
                                itemCheck(Armor,"armor[0]",offset);
                                if chapter > 1 {
                                    itemCheck(ItemType::Item,"pocketitem[0]",offset);
                                }
                            }
                            if chapter > 1 {
                                for offset in (arr_pos(12)..arr_pos(48)).step_by(0x10) {
                                    itemCheck(Weapon,"weapon[0]",offset);
                                    itemCheck(Armor,"armor[0]",offset);
                                    if offset < match chapter { 2|3 => arr_pos(24), 4 => arr_pos(36), 5 => arr_pos(48), _ => unreachable!()} {
                                        itemCheck(ItemType::Item,"pocketitem[0]",offset);
                                    }
                                }
                            }
                            let items_obtained = item_tracker.len();
                            timer::set_variable("Items",format!("{}/160",items_obtained).as_str())
                        }


                        let mut globVal = |name| global.num(&process,&stringsList,name);
                        let fighting = _fighting.update_infallible(globVal("fighting"));
                        timer::set_variable_float("fighting",fighting.current);
                        let plot = _plot.update_infallible(globVal("plot"));
                        timer::set_variable_float("Plot",plot.current);
                        timer::set_variable_float("Plot Alt",get_inst_var::<f64>(&process,version,&stringsList,process.read_pointer(DELTARUNE.add(0x49C3E0),ps).unwrap_or(Address::NULL),"plot"));
                        let choice = _choice.update_infallible(globVal("choice"));
                        timer::set_variable_float("Choice",choice.current);
                        let msc = _msc.update_infallible(globVal("msc"));
                        timer::set_variable_float("msc",msc.current);
                        let darkzone = _darkzone.update_infallible(globVal("darkzone"));
                        timer::set_variable_float("Dark World",darkzone.current);

                        //the next few vars are not detected for SP
                        let snd = snd_ptr.update_value(&process);
                        timer::set_variable("snd",snd.current.validate_utf8().unwrap_or_default());
                        let mus = mus_ptr.update_value(&process);
                        timer::set_variable("mus",mus.current.validate_utf8().unwrap_or_default());

                        //we don't really want to be constantly tracking text, we want to only check it in rooms with text splits
                        //let text = text_ptr1.update_value(&process);
                        //timer::set_variable("text",text.current.validate_utf8().unwrap_or_default());

                        match chapter {
                            // Chapter 1 logic
                            1 => {

                                let con = _con.update_infallible(match cur_room {
                                    "PLACE_CONTACT" => objVar(&chapter1ify("DEVICE_CONTACT",),"EVENT"),
                                    "room_castle_darkdoor" => objVar(&chapter1ify("obj_darkdoorevent"), "con"),
                                    "room_cc_joker" => objVar(&chapter1ify("obj_joker_body"), "dancelv"),
                                    _ => 0.0
                                });

                                let text_check = _text_check.update_infallible(match cur_room {
                                    "room_krisroom" => check_text(&process, version, &stringsList, get_obj(&obj_addr_map, &chapter1ify("obj_writer")),
                                                                  r"* (You decided to go to bed.)/%",
                                                                  r"＊ (ねむることにした)/%"),
                                    _ => false
                                });

                                // Chapter 1 room change splits
                                if room.current != room.old {
                                    split(&mut splits, &settings, match (prev_room,cur_room) {
                                        ("room_krisroom","room_dark1") => "ch1_bedskip",
                                        ("room_insidecloset","room_dark1") => "ch1_school",
                                        ("room_dark7","room_dark_chase1") => "ch1_cliffs",
                                        ("room_castle_darkdoor","room_field_start") => "ch1_castle_town_room",
                                        ("room_field4","room_field_checkers4") => "ch1_field",
                                        ("room_field_checkersboss","room_forest_savepoint1") => "ch1_board",
                                        ("room_forest_area3","room_forest_savepoint2") => "ch1_enter_bake_sale",
                                        ("room_forest_savepoint_relax","room_forest_maze1") => "ch1_enter_forest_maze",
                                        ("room_forest_fightsusie","room_forest_afterthrash2") => "ch1_susie_lancer_exit",
                                        ("room_forest_castlefront","room_cc_prison_cells") => "ch1_get_captured",
                                        ("room_cc_prison_cells","room_cc_prisonlancer") if plot.current == 156.0 => "ch1_escape_prison",
                                        ("room_cc_prison_to_elevator","room_cc_prisonelevator") => "ch1_enter_elevator",
                                        ("room_forest_fightsusie","room_field3") => "ch1_cf_warp",
                                        ("room_field3","room_forest_savepoint2") => "ch1_fb_warp",
                                        ("room_forest_savepoint2","room_forest_fightsusie") => "ch1_bc_warp",
                                        ("room_cc_prison_prejoker","room_cc_joker") => "ch1_enter_jevil",
                                        ("room_cc_joker","room_cc_prison_prejoker") => "ch1_exit_jevil",
                                        ("room_cc_6f","room_cc_throneroom") => "ch1_exit_kround2",
                                        ("room_cc_throneroom","room_cc_preroof") => "ch1_exit_throne_room",
                                        ("room_cc_preroof","room_cc_kingbattle") => "ch1_exit_preking",
                                        ("room_cc_kingbattle","room_cc_prefountain") => "ch1_post_king",
                                        ("room_cc_prefountain","room_cc_fountain") => "ch1_enter_fountain",
                                        ("room_cc_fountain","room_school_unusedroom") => "ch1_seal_fountain",
                                        _ => ""
                                    },false);
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                        "PLACE_CONTACT" if con.bytes_changed_from_to(&75.0,&76.0) => "ch1_contact",
                                        "room_castle_darkdoor" if con.bytes_changed_from_to(&7.0, &21.0) => "ch1_castle_town_door",
                                        "room_man" if msc.bytes_changed_to(&601.0) && choice.current == 0.0 => "ch1_egg",
                                        "room_cc_joker" if con.bytes_changed_to(&4.0) => "ch1_beat_jevil",
                                        "room_cc_kingbattle" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch1_king", //delay_split_frames("ch1_king",10).await
                                        "room_krisroom" if text_check.changed_to(&true) => "ch1_ending",
                                        "room_ed" if objVar("obj_credits","timer") >= 108.0 => "ch1_ending_ost",
                                        _ => ""
                                    },false);
                                }
                            }
                            // Chapter 2 logic
                            2 => {

                                let con = _con.update_infallible(match cur_room {
                                    "room_shop_ch2_spamton" => objVar( "obj_shop_ch2_spamton", "greybgtimer"),
                                    "room_dw_city_berdly" => objVar( "obj_spell_snowgrave", "timer"),
                                    _ => 0.0
                                });
                                timer::set_variable_float("LoadedDiskBG/Snowgrave",con.current);

                                let text_check = _text_check.update_infallible(match cur_room {
                                    "room_dw_cyber_queen_boxing" => textCheck(r"\\EH* C'mon^1, let's go after her!/%", r"\\EH＊ おまえら^1！&　 追っかけるぞ！/%"),
                                    "room_dw_city_big_2" => textCheck(r"* (You got the FreezeRing.)/%",r"＊ (凍てつく指輪を　手に入れた)/%"),
                                    "room_dw_city_moss" => textCheck(r"\S1* (You got the ThornRing.)/%",r"\S1＊ (いばらの指輪を　手に入れた)/%",),
                                    "room_dw_castle_west_cliff" => textCheck(r"* (You have too many \cYWEAPONs\cW to&||take \cYPuppetScarf\c0.)/%",
                                                                              r"＊ (\cYぶき\cWが多すぎて&　 \cYパペットマフラー\c0を&　 持てない)/%"),
                                    "room_torhouse" => textCheck(r"* (... Susie fell asleep.)/%",r"＊ (…スージィは　ねおちした)/%"),
                                    _ => false
                                });

                                let text_check2 = _text_check2.update_infallible(match cur_room {
                                    "room_torhouse" => textCheck(r"\E1* ... they're already&||asleep.../%",r"\E1＊ …ふたりとも　もう&　 ねむってしまったのね。/%"),
                                    _ => false
                                });


                                //Chapter 2 room-change splits
                                if cur_room != prev_room {
                                    split(&mut splits, &settings, match (prev_room,cur_room) {
                                        ("PLACE_MENU","room_krisroom") if settings.ac_pause_timer => "resume_igt",
                                        ("PLACE_MENU",_) if settings.ac_pause_timer && settings.ac_unpause_loadsave => "resume_igt",
                                        ("room_krisroom","room_dw_cyber_intro_1") => "ch2_bedskip",
                                        ("room_library","room_dw_cyber_intro_1") => "ch2_library",
                                        ("room_dw_cyber_queen_boxing","room_dw_cyber_musical_door") => "ch2_arcade_room",
                                        ("room_dw_cyber_musical_door","room_dw_cyber_musical_shop") => "ch2_dj_shop",
                                        ("room_dw_cyber_teacup_final","room_dw_cyber_rollercoaster") => "ch2_ragger2_room",
                                        ("room_dw_cyber_musical_door","room_dw_city_intro") => match plot.old < 60.0  {
                                          true => "ch2_cf_tz_skip",
                                          false => "ch2_cf_tz_warp"
                                        },
                                        ("room_dw_cyber_musical_door","room_dw_mansion_entrance") => match plot.old < 60.0 {
                                          true => "ch2_cf_m_skip",
                                          false => "ch2_cf_m_warp"
                                        },
                                        ("room_dw_city_intro","room_dw_cyber_musical_door") => "ch2_tz_cf_warp",
                                        ("room_dw_city_intro","room_dw_mansion_entrance") => "ch2_tz_m_warp",
                                        ("room_dw_mansion_entrance","room_dw_city_intro") => "ch2_m_tz_warp",
                                        ("room_dw_mansion_entrance","room_dw_cyber_musical_door") => "ch2_m_cf_warp",
                                        ("room_dw_city_mice2","room_dw_city_cheesemaze") => "ch2_maus_2",
                                        ("room_dw_city_berdly","room_dw_city_poppup") => "ch2_sideb_berdly2",
                                        ("room_dw_city_berdly","room_dw_city_traffic_4") => "ch2_berdly2_mr",
                                        ("room_dw_city_spamton_alley","room_dw_city_traffic_4") => "ch2_spamton_room",
                                        ("room_dw_city_mansion_front","room_dw_mansion_krisroom") => "ch2_cyber_city",
                                        ("room_dw_mansion_dining_a","room_dw_mansion_entrance") => "ch2_mansion_escape",
                                        ("room_dw_mansion_entrance","room_dw_mansion_fire_paintings") => "ch2_start_pandora",
                                        ("room_dw_mansion_tasquePaintings","room_dw_mansion_traffic") => "ch2_tasque_manager_room",
                                        ("room_dw_mansion_kitchen","room_dw_mansion_east_2f_transformed_new") => "ch2_mauswheel",
                                        ("room_dw_mansion_b_east","room_dw_mansion_b_east_a") => "ch2_exit_neo",
                                        ("room_dw_mansion_east_3f","room_dw_mansion_acid_tunnel") => "ch2_enter_acid",
                                        ("room_dw_mansion_acid_tunnel_loop_rouxls","room_dw_mansion_acid_tunnel_exit") => "ch2_exit_acid",
                                        ("room_dw_mansion_east_4f_d","room_dw_mansion_top") => "ch2_queen_room",
                                        ("room_dw_mansion_top","room_dw_mansion_top_post") => "ch2_giga_queen",
                                        ("room_dw_mansion_top_post","room_cc_fountain") | //Side A Fountain
                                        ("room_dw_mansion_prefountain","room_dw_mansion_fountain") => "ch2_enter_fountain", //Side B Fountain
                                        ("room_cc_fountain" | "room_dw_mansion_fountain","room_lw_computer_lab") => "ch2_seal_fountain",
                                        ("room_torhouse","room_ed") => "ch2_ending_ost",
                                        _ => ""

                                    },false)
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                          "room_dw_cyber_queen_boxing" if text_check.changed_to(&false) => "ch2_arcade_text",
                                          //"room_dw_cyber_queen_boxing" if plot.bytes_changed_to(&55.0) => delay_split_frames("ch2_arcade_text", 1).await,
                                          "room_dw_cyber_music_final" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch2_dj_battle",
                                          "room_dw_city_big_2" if text_check.changed_to(&true) => "ch2_freeze_ring",
                                          "room_dw_city_moss" if text_check.changed_to(&false) => "ch2_thorn_ring",
                                          "room_dw_cyber_musical_door" | "room_dw_city_man" if msc.old == 1173.0 && msc.current >= 1173.0 && choice.current <= 0.0 => "ch2_egg",
                                          "room_dw_castle_west_cliff" if text_check.changed_to(&true) => "ch2_thorny_ending",
                                          "room_torhouse" if text_check.changed_to(&true) => "ch2_ending_ac",
                                          "room_torhouse" if text_check2.changed_to(&true) => "ch2_ending_il",
                                          _ => ""
                                    },false);
                                }

                            }
                            // Chapter 3 logic
                            3 => {

                                let flag = _flag.update_infallible(match cur_room {
                                    "room_dw_ch3_man" => process.read::<f64>(global.ptr(&process,&stringsList,"flag[0]").add(arr_pos(930))).unwrap_or_default(),
                                    "room_dw_snow_zone" => process.read::<f64>(global.ptr(&process,&stringsList,"flag[0]").add(arr_pos(1047))).unwrap_or_default(),
                                    _ => 0.0
                                });
                                timer::set_variable_float("Flag",flag.current);
                                let mantle_outro = _instExist.update_infallible(match cur_room {
                                    "room_shadowmantle" => match process.read::<i32>(mantleOutro_instCount) { Ok(1) => true, _ => false },
                                    _ => false
                                });
                                timer::set_variable("Mantle Outro",mantle_outro.current.to_string().as_str());

                                if cur_room != prev_room {
                                    split(&mut splits,&settings,match (prev_room,cur_room) {
                                        ("PLACE_MENU","room_dw_couch_overworld_intro") if settings.ac_pause_timer => "resume_igt",
                                        ("PLACE_MENU",_) if settings.ac_pause_timer && settings.ac_unpause_loadsave => "resume_igt",
                                        ("room_dw_couch_overworld_intro" | "room_gameshowroom","room_board_gsa02_b0") => "ch3_enter_round1",
                                        ("room_board_1","room_dw_chef") => "ch3_enter_cooking",
                                        ("room_board_2","room_dw_rhythm") => "ch3_enter_rhythm",
                                        ("room_gameshowroom","room_dw_green_room") => match plot.current {
                                            110.0 | 120.0 => "ch3_end_round1",
                                            140.0 | 150.0 => "ch3_end_round2",
                                            _ => ""
                                        }
                                        ("room_gameshowroom","room_dw_backstage") => "ch3_escape_doom_board",
                                        ("room_dw_backstage","room_dw_teevie_intro") => "ch3_enter_tv_world",
                                        ("room_dw_b3bs_jail2","room_dw_teevie_cowboy_zone_02_intro") => "ch3_2nd_shootout_room",
                                        ("room_dw_teevie_stealth_d","room_dw_teevie_chef") => "ch3_enter_rouxls",
                                        ("room_dw_teevie_chef","room_dw_teevie_dust") => "ch3_exit_rouxls",
                                        ("room_dw_snow_zone_battle","room_dw_snow_zone") => "ch3_escape_doom_board",
                                        ("room_dw_snow_zone","room_gameover") => "ch3_knight_death",
                                        ("room_board_1_sword_trees","room_dw_console_room") => "ch3_ice_key",
                                        ("room_board_dungeon_2","room_dw_console_room") => "ch3_shelter_key",
                                        ("room_board_preshadowmantle","room_shadowmantle") => "ch3_enter_mantle",
                                        ("room_board_postshadowmantle","room_dw_console_room") => "ch3_exit_mantle",
                                        ("room_town_shelter","room_ed") => "ch3_ending_ost",
                                        _ => ""
                                    },false);
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                          "room_dw_snow_zone" if flag.bytes_changed_from(&0.0) => match flag.current {
                                              1.0 => "ch3_knight_win",
                                              2.0 => "ch3_knight_death",
                                              _ => "" //there shouldn't be any other possibility here?
                                          },
                                          "room_dw_snow_zone" if fighting.bytes_changed_from_to(&0.0,&1.0) => "ch3_enter_knight",
                                          "room_town_shelter" if mus.current.matches("") && mus.old.validate_utf8().unwrap_or_default().ends_with(r"mus\night_ambience.ogg")
                                          => "ch3_ending",
                                          "room_dw_man" if flag.bytes_changed_from_to(&0.0,&1.0) => "ch3_egg",
                                          "room_shadowmantle" if mantle_outro.changed_to(&true) => "ch3_end_mantle",
                                          _ => ""
                                    },false);
                                }
                            }
                            // Chapter 4 logic
                            4 => {

                                let con = _con.update_infallible(match cur_room {
                                    "room_dw_castle_tv_zone_battle" if fighting.current == 1.0 => objVar("obj_mike_attack_controller","action"),
                                    "room_torhouse" => objVar("obj_ch4_LWF03","upstairscon"),
                                    "room_dw_church_arena" => objVar("obj_hammer_of_justice_enemy","nohairsprite"),
                                    _ => 0.0
                                });
                                timer::set_variable_float("MikeAction/GersonDone/Ending",con.current);

                                let text_check = _text_check.update_infallible(match cur_room {
                                    "room_dw_churchb_man" => textCheck(r"* (An Egg was picked up from a&||nearby easel.)/%",r"＊ (近くのイーゼルから\n　 タマゴを　拾いあげた)/%"),
                                    "room_dw_churchc_prophecies" => textCheck(r"* (\cYPrincessRBN\cW was added to your&||\cYARMORs\cW.)/%",r"＊ (\cYプリティリボン\cWが&　 \cYぼうぐ\cWに　加わった)/%"),
                                    _ => false
                                });

                                if cur_room != prev_room {
                                    split(&mut splits,&settings,match (prev_room,cur_room) {
                                        ("PLACE_MENU","room_cc_fountain") if settings.ac_pause_timer => "resume_igt",
                                        ("PLACE_MENU",_) if settings.ac_pause_timer && settings.ac_unpause_loadsave => "resume_igt",
                                        ("room_schooldoor" | "room_dw_church_knightclimb_post","room_dw_castle_area_1") => "ch4_enter_castle_town",
                                        ("room_town_noellehouse","room_lw_noellehouse_main") => "ch4_enter_mansion",
                                        ("room_torhouse","room_dw_church_intro1") => "ch4_chairskip",
                                        ("room_town_church","room_dw_church_intro1") => "ch4_enter_sanctuary",
                                        ("room_dw_church_darkmaze","room_dw_church_gersonstudy") => "ch4_enter_study",
                                        ("room_dw_church_secretpiano","room_dw_church_gersonstudy") => "ch4_golden_piano",
                                        ("room_dw_church_gersonstudy","room_dw_church_arena") => "ch4_enter_hoj",
                                        ("room_dw_church_arena","room_dw_church_gersonstudy") => "ch4_exit_axe_room",
                                        ("room_dw_church_gersonstudy","room_dw_church_trueclimbadventure" | "room_dw_church_rightconnect") => "ch4_grand_piano",
                                        ("room_dw_church_holywatercooler","roohurch_entrance") => "ch4_second_sanctuary",
                                        ("room_dw_churchc_pretitan","room_dw_churchc_titanclimb1") => "ch4_start_titan_climb1",
                                        ("room_dw_churchc_titanclimb1","room_dw_churchc_titanclimb1_post") => "ch4_end_titan_climb1",
                                        ("room_dw_churchc_titanclimb1_post","room_dw_churchc_titanclimb2") => "ch4_start_titan_climb2",
                                        ("room_dw_churchc_titanclimb2","room_dw_churchc_titanclimb2_post") => "ch4_end_titan_climb2",
                                        ("room_dw_churchc_titanclimb2_post","room_dw_churchc_insidetitan") => "ch4_end_titan_fight",
                                        ("room_dw_churchc_insidetitan","room_dw_churchc_titandefeated") => "ch4_seal_titan",
                                        ("room_cc_fountain","room_lw_church_main") => "ch4_third_sanctuary",
                                        ("room_torhouse","room_krisroom_dark") => "ch4_ending",
                                        ("room_town_krisyard_dark","room_ed") => "ch4_ending_ost",
                                        _ => ""
                                    },false);
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                        "room_dw_castle_tv_zone_battle" if fighting.old == 0.0 && fighting.current == 1.0 => "ch4_start_mike",
                                        "room_dw_castle_tv_zone_battle" if fighting.current == 1.0 && con.bytes_changed_to(&18.0) => "ch4_beat_mike",
                                        "room_dw_church_jackenstein" if fighting.old == 1.0 && fighting.current == 0.0 => "ch4_jackenstein",
                                        "room_dw_church_arena" if fighting.current == 1.0 && con.bytes_changed_from(&0.0) => "ch4_hammer_of_justice",
                                        "room_dw_churchb_man" if text_check.changed_to(&false) => "ch4_egg",
                                        "room_dw_churchc_prophecies" if text_check.changed_to(&true) => "ch4_princess_ribbon",
                                        "room_dw_churchc_titanclimb2_post" if fighting.old == 0.0 && fighting.current == 1.0 => "ch4_start_titan_fight",
                                        "room_torhouse" if con.bytes_changed_from_to(&0.0,&1.0) => "ch4_ending_il",
                                        _ => ""
                                    },false);
                                }
                            }
                            // Chapter 5 logic
                            5 => {
                                let crt_start = _instExist.update_infallible(match cur_room {
                                    "room_beach" => match process.read::<i32>(crtEnd_instCount) { Ok(1) => true, _ => false },
                                    _ => false
                                });
                                timer::set_variable("CRT Start",crt_start.current.to_string().as_str());

                                let text_check = _text_check.update_infallible(match cur_room {
                                    "room_town_mid" => textCheck(r"* (You got the Bread.)/%",r"＊ (パンを　てにいれた)/%"),
                                    _ => false
                                });

                                if prev_room != cur_room {
                                    split(&mut splits,&settings,match (prev_room,cur_room) {
                                        ("PLACE_MENU","room_krisroom") if settings.ac_pause_timer => "resume_igt",
                                        ("PLACE_MENU",_) if settings.ac_pause_timer && settings.ac_unpause_loadsave => "resume_igt",
                                        ("room_schooldoor","room_dw_castle_area_1") => "ch5_enter_castle_town",
                                        ("room_krisroom","room_dw_garden_intro") => "ch5_bedskip",
                                        ("room_town_north","room_dw_garden_intro") => "ch5_enter_dw",
                                        ("room_dw_garden_firstdash","room_dw_garden_diner") => "ch5_enter_diner",
                                        ("room_dw_garden_diner","room_dw_garden_newdash") => "ch5_exit_diner",
                                        ("room_dw_garden_hardpressureplates","room_dw_garden_aquatransition") => "ch5_dark_garden",
                                        ("room_dw_garden_wateringcan_aqua","room_dw_garden_aqua") => "ch5_enter_aqua",
                                        ("room_dw_garden_aqua","room_dw_garden_aquadarkness") => "ch5_exit_aqua",
                                        ("room_dw_garden_aquashrine","room_dw_garden_aquahole" | "room_dw_garden_aquaplatforming") => "ch5_exit_feather",
                                        ("room_dw_garden_finalplatforming","room_dw_garden_cliffexit") => "ch5_enter_cliff1",
                                        ("room_dw_garden_cliffexit","room_dw_cliff_gardentransition_new") => "ch5_enter_cliff2",
                                        ("room_dw_cliff_seth_miniboss","room_dw_cliff_shop") => "ch5_enter_shop_room",
                                        ("room_dw_cliff_shop","room_dw_cliff_kawkawdash") => "ch5_exit_shop_room",
                                        ("room_dw_cliff_verticalwind_post","room_dw_cliff_sethaqua_battle") => "ch5_enter_seth_aqua",
                                        ("room_dw_cliff_sethaqua_battle","room_dw_fcastle_entrance") => "ch5_exit_seth_aqua",
                                        ("room_dw_fcastle_entrance","room_town_north") => "ch5_exit_dw",
                                        ("room_town_north","room_dw_fcastle_partyjail") => "ch5_reenter_dw",
                                        ("room_dw_fcastle_foyer","room_dw_fcastle_shinobeetle_encounter") => "ch5_enter_left",
                                        ("room_dw_fcastle_onsen","room_dw_fcastle_foyer") => "ch5_exit_left",
                                        ("room_dw_fcastle_foyer","room_dw_fcastle_cafe") => "ch5_enter_right",
                                        ("room_dw_fcastle_right_endingscene","room_dw_fcastle_foyer") => "ch5_exit_right",
                                        ("room_dw_fcastle_foyer","room_dw_fcastle_asgore") => "ch5_beanstalk",
                                        ("room_shop","room_dw_cliff_shop") if arrCheck(global.ptr(&process,&stringsList,"keyitem[0]"),
                                                                     32.0, 0, 11) => "ch5_pink_shop",
                                        ("room_dw_fcastle_top_pinkdoor","room_dw_fcastle_pinkroom") => "ch5_pink_door",
                                        ("room_dw_fcastle_pinkroom","room_dw_pink_encounter") => "ch5_pink_start",
                                        ("room_dw_pink_encounter","room_dw_fcastle_pinkroom") => "ch5_pink_exit",
                                        ("room_dw_fcastle_top_staircase_2","room_dw_fcastle_green_checkpoint") => "ch5_enter_ultimate_shop",
                                        ("room_dw_fcastle_green_checkpoint","room_dw_fcastle_top_ascent") => "ch5_exit_ultimate_shop",
                                        ("room_dw_fcastle_orange_gauntlet","room_dw_fcastle_final_save") => "ch5_enter_final_save",
                                        ("room_dw_fcastle_final_save","room_dw_fcastle_flowery") => "ch5_exit_final_save",
                                        ("room_dw_fcastle_flowery","room_dw_fcastle_flowerclimb") => "ch5_end_flowery",
                                        ("room_dw_fcastle_flowerclimb","room_dw_fcastle_flowerydash") => "ch5_end_final_climb",
                                        ("room_dw_fcastle_flowerydash","room_dw_post_flowery_battle") => "ch5_omega_flowery",
                                        ("room_dw_fcastle_top_fountain","room_dw_post_fountain_close") => "ch5_fountain1",
                                        ("room_cc_fountain","room_flowershop_2f") => "ch5_fountain2",
                                        (_,"room_schooldoor") if process.read::<f64>(global.ptr(&process,&stringsList,"flag[0]").add(arr_pos(1324))).unwrap_or_default() == 3.0
                                        => "ch5_ending_completion_data",
                                        ("room_schooldoor","room_ed") => "ch5_ending_src",
                                        _ => ""
                                    },false);
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                        "room_dw_garden_aqua" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch5_aqua_end",
                                        "room_man" if choice.bytes_changed_from_to(&-1.0,&0.0) => "ch5_egg",
                                        "room_dw_cliff_sethaqua_battle" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch5_beat_seth_aqua",
                                        "room_dw_pink_encounter" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch5_pink_end",
                                        "room_dw_fcastle_flowery" if fighting.bytes_changed_from_to(&0.0,&1.0) => "ch5_start_flowery",
                                        "room_flowershop_2f" if plot.bytes_changed_to(&560.0) => "ch5_seed_packets",
                                        "room_mid_town" if text_check.changed_to(&true) => "ch5_bread",
                                        "room_beach" if crt_start.changed_to(&true) => "ch5_sideb",
                                        _ => ""
                                    },false);
                                }
                            }
                            _ => {}
                        }

                    }

                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            }).await;
    }
}
