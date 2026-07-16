#![allow(nonstandard_style)]

use std::fs::{read};
use asr::{future::next_tick, settings::Gui, PointerSize, Process, deep_pointer::DeepPointer,
    watcher::{Watcher,Pair}, Address, string::{ArrayCString}, signature::Signature, timer};
use asr::file_format::pe;
use asr::settings::gui::Title;
use std::collections::{HashSet};
use asr::timer::TimerState;

asr::async_main!(stable);

#[derive(Gui)]
enum ChapterEndTimings {
    /// Individual chapter run rules
    SingleChapter,
    /// All Chapters rules
    #[default]
    AllChapters,
    /// OST% timing (Chapters 1, 3, 4, and 5 end on credits music start)
    OST,
    /// Demo/per-chapter OST% timing (EVERY chapter ends on credits song start)
    OSTLateCh2
}
#[derive(Gui)]
pub enum AutoStart {
    /// Start the timer, resetting if it was already running
    #[default]
    AutoReset,
    /// Start the timer, if it was not already running
    AutoStart,
    /// Do nothing
    Off
}

#[derive(Gui)]
enum Ch5StartOnPrev {
    ///No
    #[default]
    No,
    ///Yes
    Yes,
    ///Exclusively
    Exclusively
}

#[derive(Gui)]
struct Settings {

    ///General Settings
    gen_title : Title,
    ///On creating a new file:
    auto_start : AutoStart,
    ///Pause the timer between chapters
    #[default = true]
    ac_pause_timer: bool,
    ///Timing of the pauses
    chapter_pause_timing : ChapterEndTimings,


    ///Chapter 1: The Beginning
    ch1_title : Title,
    ///CONTACT
    ch1_contact : bool,
    ///Enter Dark World (True Reset)
    ch1_school : bool,
    ///Enter Chase 1 room
    ch1_cliffs : bool,
    ///Castle Town (Door Close)
    ch1_castle_town_door : bool,
    ///Castle Town (Room Change)
    ch1_castle_town_room : bool,
    ///Field of Hopes and Dreams
    ch1_field : bool,
    ///Great Board
    ch1_board : bool,
    ///Enter Bake Sale
    ch1_enter_bake_sale : bool,
    ///Obtain Egg
    ch1_egg : bool,
    ///Enter Forest Maze
    ch1_enter_forest_maze : bool,
    ///Exit Forest (Susie & Lancer battle room)
    ch1_susie_lancer_exit : bool,
    ///Enter Prison Cell (captured)
    ch1_get_captured : bool,
    ///Escape Prison
    ch1_escape_prison : bool,
    ///Enter Elevator
    ch1_enter_elevator : bool,
    ///Exit K. Round 2 battle room
    ch1_exit_kround2 : bool,
    ///Exit Throne Room
    ch1_exit_throne_room : bool,
    ///Exit Pre-King battle room
    ch1_exit_preking : bool,
    ///End King battle
    ch1_king : bool,
    ///Exit King battle room
    ch1_post_king : bool,
    ///Enter Fountain
    ch1_enter_fountain : bool,
    ///Seal Fountain
    ch1_seal_fountain : bool,
    ///Ending
    ch1_ending : bool,
    ///(OST%) Ending
    ch1_ending_ost : bool,
    ///Ch1 All Bosses Splits
    #[heading_level = 2]
    ch1_ab_splits : Title,
    ///Warp from Castle to Field
    ch1_cf_warp : bool,
    ///Warp from Field to Bake Sale
    ch1_fb_warp : bool,
    ///Warp from Bake Sale to Castle
    ch1_bc_warp : bool,
    ///Enter Jevil
    ch1_enter_jevil : bool,
    ///Defeat Jevil
    ///
    ///This autosplit does not work if you remove THE WORLD REVOLVING from the game files (mus\joker.ogg) in non-SURVEY_PROGRAM versions.
    ch1_beat_jevil : bool,
    ///Exit Jevil battle room
    ch1_exit_jevil : bool,


    ///Chapter 2: A Cyber's World
    ch2_title : Title,
    ///Enter Dark World (True Reset)
    ch2_library : bool,
    ///End Punch-Out minigame (textbox close)
    ///
    ///This autosplit does not work if you remove A CYBER'S WORLD? from the game files (mus\cyber.ogg) - which is also illegal.
    ch2_arcade_text : bool,
    ///End Punch-Out minigame (room exit)
    ch2_arcade_room : bool,
    ///End Sweet Cap'n Cakes battle
    ch2_dj_battle : bool,
    ///Enter Sweet Cap'n Cakes' shop room
    ch2_dj_shop : bool,
    ///Exit Ragger2 room
    ch2_ragger2_room : bool,
    ///Exit Cyber Field
    ch2_cyber_field : bool,
    ///Warp from Cyber Field to Trash Zone (normally)
    ch2_cf_tz_warp : bool,
    ///Warp from Cyber Field to Trash Zone (with Door Overflow)
    ch2_cf_tz_skip : bool,
    ///Warp from Cyber Field to Mansion (normally)
    ch2_cf_m_warp : bool,
    ///Warp from Cyber Field to Mansion (with Door Overflow)
    ch2_cf_m_skip : bool,
    ///Warp from Trash Zone to Cyber Field
    ch2_tz_cf_warp : bool,
    ///Warp from Trash Zone to Mansion
    ch2_tz_m_warp : bool,
    ///Warp from Mansion to Cyber Field
    ch2_m_cf_warp : bool,
    ///Warp from Mansion to Trash Zone
    ch2_m_tz_warp : bool,
    ///Obtain Egg (both sources)
    ch2_egg : bool,
    ///Exit Mouse 2 Puzzle room
    ch2_maus_2 : bool,
    ///Exit Berdly 2 battle room (Side A)
    ch2_berdly2_mr : bool,
    ///Exit Spamton battle room
    ch2_spamton_room : bool,
    ///Exit Cyber City (captured by Queen)
    ch2_cyber_city : bool,
    ///Enter Mansion Entrance save point room
    ch2_mansion_escape : bool,
    ///Exit Mansion Entrance save point room
    ch2_start_pandora : bool,
    ///Exit Tasque Manager battle room
    ch2_tasque_manager_room : bool,
    ///Exit Mauswheel battle room
    ch2_mauswheel : bool,
    ///Enter Acid Tunnel
    ch2_enter_acid : bool,
    ///Exit Acid Tunnel
    ch2_exit_acid : bool,
    ///Exit Queen battle room
    ch2_queen_room : bool,
    ///End Giga Queen battle
    ch2_giga_queen : bool,
    ///Enter Fountain
    ch2_enter_fountain : bool,
    ///Seal Fountain
    ch2_seal_fountain : bool,
    ///Open PuppetScarf Chest (Castle Town)
    ch2_puppet_scarf_late : bool,
    ///Ending (individual chapter)
    ch2_ending_il : bool,
    ///Ending (All Chapters)
    ch2_ending_ac : bool,
    ///Ending (Demo OST%)
    ch2_ending_ost : bool,
    ///All Bosses Splits
    #[heading_level = 2]
    ch2_ab_title : Title,
    ///Obtain Loaded Disk
    ch2_load_disk : bool,
    ///Insert Loaded Disk
    ch2_insert_disk : bool,
    ///Defeat basement NEO
    ///
    ///This autosplit does not work if you remove BIG SHOT from the game files (mus\spamton_neo_mix_ex_wip.ogg).
    ch2_defeat_neo_ab : bool,
    ///Exit basement NEO room
    ch2_exit_neo : bool,
    ///Side B Splits
    #[heading_level = 2]
    ch2_sideb_title : Title,
    ///Obtain FreezeRing
    ch2_freeze_ring : bool,
    ///Obtain ThornRing (original source)
    ch2_thorn_ring : bool,
    ///SnowGrave
    ch2_snowgrave : bool,
    ///Exit Berdly 2 battle room (Side B)
    ch2_sideb_berdly2 : bool,
    ///End fountain Spamton NEO battle
    ch2_sideb_neo : bool,


    ///Chapter 3: Late Night
    ch3_title : Title,
    ///Enter Board 1 (True Reset)
    ch3_enter_round1 : bool,
    ///Enter Cooking Show
    ch3_enter_cooking : bool,
    ///Enter Green Room (post-Board 1)
    ch3_greenroom1 : bool,
    ///Enter Lightners Live
    ch3_enter_rhythm : bool,
    ///Enter Green Room (post-Board 2)
    ch3_greenroom2 : bool,
    ///Enter TV World Backstage (post-Doom Board)
    ch3_escape_doom_board : bool,
    ///Enter TV World
    ch3_enter_tv_world : bool,
    ///Enter 2nd shootout room
    ch3_2nd_shootout_room : bool,
    ///Enter Rouxls battle room
    ch3_enter_rouxls : bool,
    ///Exit Rouxls battle room
    ch3_exit_rouxls : bool,
    ///Obtain Egg
    ch3_egg : bool,
    ///Start Tenna battle
    ch3_enter_tenna : bool,
    ///Defeat Tenna
    ch3_beat_tenna : bool,
    ///Start Knight battle
    ch3_enter_knight : bool,
    ///End Knight battle (loss)
    ch3_knight_death : bool,
    ///End Knight battle (win)
    ch3_knight_win : bool,
    ///Ending
    ///
    ///This autosplit does not work if you remove Crickets from the game files (mus\night_ambience.ogg).
    ch3_ending : bool,
    ///Ending (OST%)
    ch3_ending_ost : bool,

    ///All Bosses Splits
    #[heading_level = 2]
    ch3_ab_title : Title,
    ///Obtain Ice Key (room exit)
    ch3_ice_key : bool,
    ///Obtain Shelter Key (room exit)
    ch3_shelter_key : bool,
    ///Enter Shadow Mantle fight
    ch3_enter_mantle : bool,
    ///Defeat Shadow Mantle Enemy
    ch3_end_mantle : bool,
    ///Obtain Shadow Mantle (room exit)
    ch3_exit_mantle : bool,


    ///Chapter 4: Prophecy
    ch4_title : Title,
    ///Enter Castle Town
    ch4_enter_castle_town : bool,
    ///Start Mike Fight
    ch4_start_mike : bool,
    ///End Mike fight
    ch4_beat_mike : bool,
    ///Enter Noelle's House
    ch4_enter_mansion : bool,
    ///Enter Dark Sanctuary (Couch Skip or True Reset)
    ch4_enter_sanctuary : bool,
    ///Enter Gerson's Study
    ch4_enter_study : bool,
    ///End Jackenstein Fight
    ch4_jackenstein : bool,
    ///Exit Grand Piano Room
    ch4_grand_piano : bool,
    ///Exit Miss Mizzle fight room
    ch4_miss_mizzle : bool,
    ///Seal First Sanctuary
    ch4_first_sanctuary : bool,
    ///Fall Below Study
    ch4_fall_down : bool,
    ///Obtain Egg
    ch4_egg : bool,
    ///End Sound of Justice fight
    ch4_sound_of_justice : bool,
    ///Seal Second Sanctuary
    ch4_second_sanctuary : bool,
    ///Obtain PrincessRBN
    ch4_princess_ribbon : bool,
    ///Start First Titan Climb
    ch4_start_titan_climb1 : bool,
    ///End First Titan Climb
    ch4_end_titan_climb1 : bool,
    ///Start Second Titan Climb
    ch4_start_titan_climb2 : bool,
    ///End Second Titan Climb
    ch4_end_titan_climb2 : bool,
    ///Start Titan Fight
    ch4_start_titan_fight : bool,
    ///End Titan Fight
    ch4_end_titan_fight : bool,
    ///Seal the Titan
    ch4_seal_titan : bool,
    ///Seal Third Sanctuary
    ch4_third_sanctuary : bool,
    ///Ending
    ch4_ending : bool,
    ///Ending (OST%)
    ch4_ending_ost : bool,


    ///All Bosses Splits
    #[heading_level = 2]
    ch4_ab_title : Title,
    ///Solve the Golden Piano puzzle
    ch4_golden_piano : bool,
    ///Enter Hammer of Justice battle room
    ch4_start_gerson_fight : bool,
    ///End Hammer of Justice battle
    ch4_hammer_of_justice : bool,
    ///Exit Hammer of Justice battle room
    ch4_exit_axe_room : bool,



    ///Chapter 5: Festival Day
    ch5_title : Title,
    ///Start/reset timer on loading Ch4 completion data?
    ch5_start_on_prev : Ch5StartOnPrev,
    ///Enter Castle Town
    ch5_enter_ct : bool,
    ///Enter Flower King Dark World (True Reset)
    ch5_enter_dw : bool,
    ///Enter Ideal Diner
    ch5_enter_diner : bool,
    ///Exit Ideal Diner
    ch5_exit_diner : bool,
    ///Enter dark garden room
    ch5_dark_garden : bool,
    ///Enter Aqua battle room
    ch5_enter_aqua : bool,
    ///End Aqua battle
    ch5_aqua_end : bool,
    ///Exit Aqua battle room
    ch5_exit_aqua : bool,
    ///Exit Petal Feather room
    ch5_exit_feather : bool,
    ///Enter first Cliffs save point room
    ch5_enter_cliff1 : bool,
    ///Exit first Cliffs save point room
    ch5_enter_cliff2 : bool,
    ///Enter Pink's shop room
    ch5_enter_shop_room : bool,
    ///Exit Pink's shop room
    ch5_exit_shop_room : bool,
    ///Obtain Egg
    ch5_egg : bool,
    ///Enter Seth & Aqua battle room
    ch5_enter_seth_aqua : bool,
    ///End Seth & Aqua battle
    ch5_beat_seth_aqua : bool,
    ///Exit Seth & Aqua battle room
    ch5_exit_seth_aqua : bool,
    ///Leave Dark World
    ch5_exit_dw : bool,
    ///Reenter Dark World
    ch5_reenter_dw : bool,
    ///Enter the left side from the foyer
    ch5_enter_left : bool,
    ///Enter the foyer from the left side
    ch5_exit_left : bool,
    ///Enter the right side from the foyer
    ch5_enter_right : bool,
    ///Enter the foyer from the right side
    ch5_exit_right : bool,
    ///Climb the foyer beanstalk
    ch5_beanstalk : bool,
    ///Enter the ultimate shop room
    ch5_enter_ultimate_shop : bool,
    ///Exit the ultimate shop room
    ch5_exit_ultimate_shop : bool,
    ///Enter the final save point room
    ch5_enter_final_save : bool,
    ///Enter Flowery battle room
    ch5_exit_final_save : bool,
    ///Start Flowery battle
    ch5_start_flowery : bool,
    ///End mid-battle climb
    ch5_end_final_climb : bool,
    ///End Flowery battle
    ch5_omega_flowery : bool,
    ///Seal Fountain 1
    ch5_fountain1 : bool,
    ///Seal Fountain 2
    ch5_fountain2 : bool,
    ///Ending (SRC rules)
    ch5_ending_src : bool,
    ///Ending (completion data timing)
    ch5_ending_completion_data : bool,
    ///Complete Side B
    ch5_sideb : bool,

    ///All Bosses Splits
    #[heading_level = 2]
    ch5_ab_title : Title,
    ///Exit Pink's shop after buying the key
    ch5_pink_shop : bool,
    ///Enter the pink door
    ch5_pink_door : bool,
    ///Enter Pink battle room
    ch5_pink_start : bool,
    ///End Pink battle
    ch5_pink_end : bool,
    ///Exit Pink battle room
    ch5_pink_exit : bool,
}

const n : &[u64] = &[0];

const ps64: PointerSize = PointerSize::Bit64;
const ps32: PointerSize = PointerSize::Bit32;

pub fn room_match(cur_room : ArrayCString<64>, check_room : &str) -> bool {
    cur_room.validate_utf8().unwrap_or_default().strip_suffix("_ch1").unwrap_or_default() == check_room
}

pub fn room_check(room : Pair<ArrayCString<64>>, dest : &str) -> bool {
    room_match(room.current,dest) && !room_match(room.old,dest)
}

pub fn room_check_both(room : Pair<ArrayCString<64>>, orig : &str, dest : &str) -> bool {
    room_match(room.current,dest) && room_match(room.old,orig)
}


pub fn text_match(txt : ArrayCString<128>, en : &str, jp : &str) -> bool {
    txt.matches(en) || txt.matches(jp)
}

pub fn text_open_check(txt : Pair<ArrayCString<128>>, en : &str, jp : &str) -> bool {
    text_match(txt.current,en,jp) && !text_match(txt.old,en,jp)
}

pub fn text_close_check(txt : Pair<ArrayCString<128>>, en : &str, jp : &str) -> bool {
    text_match(txt.old,en,jp) && !text_match(txt.current,en,jp)
}

pub fn start(auto_start : &AutoStart, splits : &mut HashSet<String>, tempVar : &mut u64) {
    match auto_start {
        AutoStart::AutoReset => {
            tempVar = 0;
            splits.clear();
            timer::reset();
            timer::start();
        }
        AutoStart::AutoStart => {
            //tempVar = 0;
            //splits.clear();
            timer::start();
        }
        AutoStart::Off => {}
    }
}

pub fn split(splits : &mut HashSet<String>, name : &str) {
    splits.insert(name.to_string());
    asr::print_message(format!("Split triggered: {}",name).as_str());
    timer::split();
}


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
        *self.watcher.unwrap().update_infallible(value.unwrap())
    }
}

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();
    let mut splits = HashSet::<String>::new();

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
                timer::set_variable("Module Address",format!("{:X}",DELTARUNE.value()).as_str());
                //timer::set_variable_int("Module Address",DELTARUNE.value());


                path = path.replace("DELTARUNE.exe", "data.win");
                timer::set_variable("Path", path.as_str());
                let md5 = &format!("{:X?}", md5::compute(read(path).unwrap_or_default()));
                timer::set_variable("MD5", md5);
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
                    "272A16964597ED6DC8D2393ED051D3CE" => "Demo v1.09", // demo 1.09 30tbps Steam
                    "A88A2DB3A68C714CA2B1FF57AC08A032" | //SP-EN vanilla
                    "047C11435B1C592EC731BFF3B9C5B0CF" | //SP-EN 30tbps
                    "22008370824A37BAEF8948127963C769" | //SP-JP vanilla
                    "E05433FE679BC91E3809C1138E3A8EA1" => "SURVEY_PROGRAM", //SP-JP 30tbps
                    _ => "invalid",
                };
                timer::set_variable("version", version);

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
                            _ => n,
                        }).unwrap_or_default();
                        if _dir != ArrayCString::<256>::default() {
                            break;
                        }
                        next_tick().await;
                    }
                    let dir = _dir.validate_utf8().unwrap_or("invalid UTF8");
                    timer::set_variable("dir", dir);
                    if dir.ends_with("chapter1_windows\\") { chapter = 1 }
                    else if dir.ends_with("chapter2_windows\\") { chapter = 2 }
                    else if dir.ends_with("chapter3_windows\\") { chapter = 3 }
                    else if dir.ends_with("chapter4_windows\\") { chapter = 4 }
                    else if dir.ends_with("chapter5_windows\\") { chapter = 5 }
                }
                timer::set_variable_int("Chapter",chapter);


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
                timer::set_variable("Room Array Address",format!("{:X}",room_array_addr.value()).as_str());

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
                room_id_addr = match ps {
                    ps64 => room_id_addr.add(process.read::<i32>(room_id_addr).unwrap_or_default() as u64 + 4),
                    _ => room_id_addr
                };
                timer::set_variable("Room ID Address",format!("{:X}",room_id_addr.value()).as_str());

                let mut room_watch = Watcher::<i32>::new();
                let mut room_name_watch = Watcher::<ArrayCString<64>>::new();

                // sound stuff (pointer only varies by runner version)

                let mut snd_ptr = VarTrack::<ArrayCString<256>>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x4E0794, 0x58, 0xC0,  0x40, 0x0],
                    "Demo v1.15" => &[0x4E20B4, 0x58, 0xC0,  0x40, 0x0],
                    "Demo v1.19" | "CH1-4 v1.02" => &[0x6A3818, 0x60, 0xD0, 0x58, 0x0],
                    "CH1-5 v244" => &[0x6AB818, 0x60, 0xD0, 0x58, 0x0],
                    _ => n
                });

                let mut mus_ptr = VarTrack::<ArrayCString<256>>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x4DFF58, 0x0,  0x44,  0x0],
                    "Demo v1.15" => &[0x4E1878, 0x0,  0x0,   0x0],
                    "Demo v1.19" | "CH1-4 v1.02" => &[0x6A2F90, 0x0,  0x0,  0x0],
                    "CH1-5 v244" => &[0x6AAF90, 0x0,  0x0,  0x0],
                    _ => n
                });


                //DEVICE_NAMER.EVENT

                let mut namer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" => &[0x6EF220, 0xD4, 0x5C,  0x20, 0x24,  0x10, 0x9C,  0x0],
                    "Demo v1.10" => &[0x6EF220, 0xD4, 0x5C,  0x20, 0x24,  0x10, 0x2F4, 0x0],
                    "Demo v1.15" => &[0x6F0B48, 0xD4, 0x5C,  0x20, 0x24, 0x10, 0xFC,  0x0],
                    "Demo v1.19" => &[0x8B2790, 0x178, 0x70,  0x38, 0x48,  0x10, 0x3B0, 0x0],
                    "CH1-4 v1.02" => match chapter {
                        2 | 3 => &[0x8B2790, 0x178, 0x70, 0x38, 0x48, 0x10, 0x60, 0x0],
                        4 => &[0x8B2790, 0x178, 0x70, 0x38, 0x48, 0x10, 0x280, 0x0],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        2 => &[0x8BA790, 0x178, 0x70,  0x38,   0x48,  0x10,  0x90,  0x0],
                        3 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x120, 0x0],
                        4 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48,  0x10,  0x40,  0x0],
                        5 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x170, 0x0],
                        _ => n
                    }
                    _ => n
                });

                //Global variables
                //(note: for global.flag[N] values, the last offset is the only difference between different flags' locations, and is equal to 16x the flag's index number - which you can get either by directly multiplying by 16 and putting it in as a decimal number, or by converting to hex then adding a trailing zero.)

                let mut old_chapter_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x24D8, 0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0x2F34, 0x80],
                    _ => n
                });

                let mut filechoice_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x488, 0x4D0],
                    _ => n
                });

                let mut fighting_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    "SURVEY_PROGRAM" => n,
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x4F8,  0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0xA758, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0x710],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0xBB0],
                        _ => n
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x720],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x100,  0x0],
                        3 => &[0x6A1CA8, 0x48,  0x10,   0x1190, 0x370],
                        4 => &[0x6A1CA8, 0x48,  0x10,   0x72B0, 0x370],
                        _ => n
                    }
                    "CH1-5 v1.02" => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x740],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x100,  0x0],
                        3 => &[0x6A9CA8, 0x48,  0x10,   0x1190, 0x370],
                        4 => &[0x6A9CA8, 0x48,  0x10,   0x72B0, 0x370],
                        5 => &[0x6A9CA8, 0x48,  0x10,   0x820,  0x70],
                        _ => n
                    }
                    _ => n,
                });

                let mut plot_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x488, 0x500],
                    "CH1-4 v1.02" => match chapter {
                        3 => &[0x6A1CA8, 0x48,  0x10,   0x1000, 0x250],
                        4 => &[0x6A1CA8, 0x48,  0x10,   0x2F40, 0x30],
                        _ => n
                    },
                    "CH1-5 v244" => match chapter {
                        3 => &[0x6A9CA8, 0x48,  0x10,   0x1000, 0x250],
                        4 => &[0x6A9CA8, 0x48,  0x10,   0x2F70, 0x30],
                        _ => n
                    },
                    _ => n
                });

                let mut choicer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x28,  0x40],
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x18C0, 0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0xBA0,  0xC0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0x0],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0x0],
                        _ => n
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x10],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x7870, 0x0],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x10],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x7870, 0x0],
                        5 => &[0x6A9CA8, 0x48,  0x10,   0x150,  0x20],
                        _ => n
                    }
                    _ => n,
                });

                let mut msc_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48E5DC, 0x27C, 0x28,  0x140],
                    "Demo v1.09" | "Demo v1.10" => &[0x6FCF38, 0x30, 0x354C, 0x0],
                    "Demo v1.15" => &[0x6FE860, 0x30, 0x17AC, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0xF0],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0x130],
                        _ => n
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x100],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x7310, 0x0],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x100],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x7310, 0x0],
                        _ => n
                    }
                    _ => n
                });

                //globals with chapter-specific relevance (e.g. flags)

                let mut knight_result_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        "CH1-4 v1.02" => &[0x6A1CA8, 0x48,  0x10,   0x6A70, 0x0,  0x90, 0x4170],
                        "CH1-5 v244" => &[0x6A9CA8, 0x48,  0x10,   0x6A70, 0x0,  0x90, 0x4170],
                        _ => n
                    }
                    _ => n
                });

                let mut pink_coins_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    5 => match version {
                        "CH1-5 v244" => &[0x6A9CA8, 0x48,  0x10,   0x6BB0, 0x0,  0x90, 0x5200],
                        _ => n
                    }
                    _ => n
                });




                //recurring objects across chapters

                let mut text_ptr1 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" |
                    "Demo v1.10" => &[0x6FCE4C, 0x8,  0x144, 0x24, 0x10, 0x5A0, 0x0, 0x0, 0x0],
                    "Demo v1.15" => &[0x6FE774, 0x8,  0x144, 0x24, 0x10, 0x0, 0x0, 0x0, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0xF0, 0x0, 0x0, 0x0],
                        2 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x5F0, 0x0, 0x0, 0x0],
                        _ => n
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x8C2008, 0x10, 0x1A0, 0x48,   0x10,  0x390, 0x0, 0x0, 0x0],
                        2 => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x6F0, 0x0,   0x0,  0x0],
                        4 => &[0x8C2008, 0x10,  0x1A0,  0x48,   0x10,  0x300, 0x0,   0x0, 0x0],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x8CE220, 0x10, 0x1A0, 0x48,   0x10,  0x390, 0x0, 0x0, 0x0],
                        2 => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x6F0, 0x0,   0x0,  0x0],
                        4 => &[0x8CE220, 0x10,  0x1A0,  0x48,   0x10,  0x310, 0x0,   0x0,  0x0],
                        _ => n
                    }
                    _ => n
                });

                let mut text_ptr2 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        "Demo v1.19" => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x6D0, 0x0, 0x0, 0x0],
                        "CH1-4 v1.02" => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x700, 0x0,   0x0,  0x0],
                        "CH1-5 v244" => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x700, 0x0,   0x0,  0x0],
                        _ => n
                    }
                    _ => n
                });
                let mut text_ptr3 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        "Demo v1.19" => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x6F0, 0x0, 0x0, 0x0],
                        "CH1-4 v1.02" => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x710, 0x0,   0x0,  0x0],
                        "CH1-5 v244" => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x710, 0x0,   0x0,  0x0],
                        _ => n
                    }
                    _ => n
                });
                let mut text_ptr4 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        "CH1-4 v1.02" => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x7E0, 0x0,   0x0,  0x0],
                        "CH1-5 v244" => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x7E0, 0x0,   0x0,  0x0],
                        _ => n
                    }
                    _ => n
                });


                let mut susie_sprite_ptr = VarTrack::<i32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x1008, 0x50,   0x158, 0x10,  0xBC],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1018, 0x50,   0x158, 0x10,  0xBC],
                        _ => n
                    }
                    _ => n
                });

                let mut player_x_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x198,  0x0,    0x50,  0x158, 0x10,  0xE8],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1A8,  0x0,    0x50,  0x158, 0x10,  0xE8],
                        _ => n
                    }
                    _ => n
                });

                let mut player_y_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x198,  0x0,    0x50,  0x158, 0x10,  0xEC],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1A8,  0x0,    0x50,  0x158, 0x10,  0xEC],
                        _ => n
                    }
                    _ => n
                });



                //Ch1 objects

                let mut great_door_con_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0xC,  0x60, 0x10, 0x10,  0x0],
                    "Demo v1.09" | "Demo v1.10" => &[0x6EF220, 0x84, 0x24,  0x10, 0x18,  0x0],
                    "Demo v1.15" => &[0x6F0B48, 0x84, 0x24,  0x10, 0x18, 0x0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x8B2790, 0xE0,  0x48,  0x10, 0x0,   0x0],
                        _ => n
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x8B2790, 0xE0, 0x48,  0x10,   0x30,  0x0],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x8BA790, 0xE0, 0x48,  0x10,   0x30,  0x0],
                        _ => n
                    }
                    _ => n
                });

                let mut king_pos_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x6AEB80, 0x4, 0x178, 0x80, 0xC8, 0x8, 0xB4],
                    "Demo v1.09" | "Demo v1.10" => &[0x6F1394, 0x4, 0x140, 0x68, 0x3C, 0x8, 0xB0],
                    "Demo v1.15" => &[0x6F2CBC, 0x4, 0x140, 0x68, 0x3C, 0x8, 0xB0],
                    "Demo v1.19" => match chapter {
                        1 => &[0x69FA98, 0x0, 0x530, 0x50, 0x158, 0x10, 0xE8],
                        _ => n
                    }
                    "CH1-4 v1.02" => match chapter {
                        1 => &[0x69FA98, 0x0,  0x560, 0x50,   0x158, 0x10,  0xE8],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        1 => &[0x6A7A98, 0x0,  0x560, 0x50,   0x158, 0x10,  0xE8],
                        _ => n
                    }
                    _ => n
                });



                //SP-specific object checks

                let mut jevil_dance_ptr1 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x78, 0x60, 0x10, 0x10,  0x0],
                    _ => n
                });
                let mut jevil_dance_ptr2 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x7C, 0x60, 0x10, 0x10,  0x0],
                    _ => n
                });
                let mut final_textbox_ptr1 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x98, 0x60, 0x10, 0x274, 0x0],
                    _ => n
                });
                let mut final_textbox_ptr2 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "SURVEY_PROGRAM" => &[0x48BDEC, 0x9C, 0x60, 0x10, 0x274, 0x0],
                    _ => n
                });



                //Ch2 objects

                let mut loaded_disk_bg_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" => &[0x6EF220, 0x84, 0x24,  0x10, 0x3D8, 0x0],
                    "Demo v1.10" => &[0x6EF220, 0x84, 0x24,  0x10, 0x87C, 0x0],
                    "Demo v1.15" => &[0x6F0B48, 0x84, 0x24,  0x10, 0x0,  0x0],
                    "Demo v1.19" => match chapter {
                        2 => &[0x8B2790, 0xE0,  0x48,  0x10, 0x3C0, 0x0],
                        _ => n
                    }
                    "Ch1-4 v1.02" => match chapter {
                        2 => &[0x8B2790, 0xE0,  0x48,  0x10,   0xC70, 0x0],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        2 => &[0x8BA790, 0xE0,  0x48,  0x10,   0xCA0, 0x0],
                        _ => n
                    }
                    _ => n
                });

                let mut snowgrave_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    "Demo v1.09" | "Demo v1.10" => &[0x6EF220, 0xF4, 0x27C, 0x6C, 0x5C,  0x20, 0x144, 0x24, 0x10, 0xC0, 0x0],
                    "Demo v1.15" => &[0x6F0B48, 0xF4, 0x27C, 0x6C, 0x5C, 0x20, 0x144, 0x24, 0x10, 0x120, 0x0],
                    "Demo v1.19" => match chapter {
                        2 => &[0x8B2790, 0x1A0, 0x3B0, 0x88, 0x70,  0x38, 0x1A0, 0x48, 0x10, 0x3D0, 0x0],
                        _ => n
                    }
                    "CH1-4 v1.02" => match chapter {
                        2 => &[0x8B2790, 0x1A0, 0x3B0, 0x88,   0x70,  0x38,  0x1A0, 0x48, 0x10, 0xA0, 0x0],
                        _ => n
                    }
                    "CH1-5 v244" => match chapter {
                        2 => &[0x8BA790, 0x1A0, 0x3B0, 0x88,   0x70,  0x38,  0x1A0, 0x48, 0x10, 0x80, 0x0],
                        _ => n
                    }
                    _ => n
                });





                //Ch3 objects

                let mut egg_timer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        "CH1-4 v1.02" => &[0x8B2790, 0x1E8, 0x530,  0x38,   0x48, 0x10, 0x290, 0x0],
                        "CH1-5 v244" => &[0x8BA790, 0x1E8, 0x40,   0x38,   0x48, 0x10, 0x330, 0x0],
                        _ => n
                    }
                    _ => n
                });
                let mut mantle_outro_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        "CH1-4 v1.02" => &[0x69FA98, 0x0,   0x19B0, 0x18,   0x50, 0x10, 0xD0],
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x19B0, 0x18,   0x50, 0x10, 0xD0],
                        _ => n
                    }
                    _ => n
                });



                //Ch4 objects

                let mut mike_action_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        "CH1-4 v1.02" => &[0x8B2790, 0x1A0, 0x2F0,  0x90,   0x78,  0x38,  0x198, 0x48, 0x10, 0x140, 0x0],
                        "CH1-5 v244" => &[],
                        _ => n
                    }
                    _ => n
                });


                //Ch5 objects

                let mut crt_start_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    5 => match version {
                        "CH1-5 v244" => &[0x6A7A98, 0x0,   0x1910, 0x8,    0x18, 0x68, 0x10,  0xE4],
                        _ => n
                    }
                    _ => n
                });

                let mut tempVar = 0;







                // TODO: Load some initial information from the process.
                loop {
                    settings.update();

                    if ps == ps32 {
                        chapter = old_chapter_ptr.update_value(&process).current as i32;
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
                    //asr::timer::set_variable("Room Name Pointer Address",format!("{:X}",room_name_addr0.value()).as_str());
                    //asr::timer::set_variable("Room Name Address",format!("{:X}",room_name_addr.value()).as_str());
                    timer::set_variable("Room Name", room.current.validate_utf8().unwrap_or_default());

                    let state = timer::state();

                    if (state == TimerState::NotRunning || state == TimerState::Ended) && !splits.is_empty() {
                        tempVar = 0;
                        splits.clear();
                    }

                    match chapter {
                        //logic for autostart, autoreset, and continuing game time
                        0 => {}
                        1 => {
                            if room_check(*room, "PLACE_CONTACT") {
                                start(&settings.auto_start,&mut splits,&mut tempVar);
                            }
                        }
                        5 => {
                            let namer_event = namer_ptr.update_value(&process);
                            timer::set_variable_float("Namer Event",namer_event.current);
                            if !matches!(settings.ch5_start_on_prev,Ch5StartOnPrev::Exclusively) {
                                if room_match(room.current, "PLACE_MENU")
                                {
                                    if namer_event.current == 75.0 && namer_event.old == 74.0 {
                                        start(&settings.auto_start,&mut splits,&mut tempVar);
                                    }
                                }
                            }
                            if !matches!(settings.ch5_start_on_prev,Ch5StartOnPrev::No) {
                                if room_check_both(*room,"PLACE_MENU","room_krisroom") && namer_event.old != 75.0 {
                                    start(&settings.auto_start,&mut splits,&mut tempVar);
                                }
                            }
                        }
                        _ => {
                            if room_match(room.current, "PLACE_MENU")
                            {
                                let namer_event = namer_ptr.update_value(&process);
                                timer::set_variable_float("Namer Event",namer_event.current);
                                if namer_event.current == 75.0 && namer_event.old == 74.0 {
                                    start(&settings.auto_start,&mut splits,&mut tempVar);
                                }
                            }
                        }
                    }


                    // if we're not in the middle of a run, no reason to do anything past the autostart check
                    if timer::state() == TimerState::Running {
                        let fighting = fighting_ptr.update_value(&process);
                        timer::set_variable_float("Fighting",fighting.current);
                        //the next few vars are not in SP but whatever
                        let text = text_ptr1.update_value(&process);
                        timer::set_variable("Text",text.current.validate_utf8().unwrap_or_default());
                        let snd = snd_ptr.update_value(&process);
                        timer::set_variable("snd",snd.current.validate_utf8().unwrap_or_default());
                        let mus = mus_ptr.update_value(&process);
                        timer::set_variable("mus",mus.current.validate_utf8().unwrap_or_default());

                        match chapter {
                            1 => {
                                let choice = choicer_ptr.update_value(&process);
                                timer::set_variable_float("Choice",choice.current);
                                let msc = msc_ptr.update_value(&process);
                                timer::set_variable_float("msc",msc.current);

                                let great_door_con = great_door_con_ptr.update_value(&process);
                                timer::set_variable_float("doorCon",great_door_con.current);
                                let king_pos = king_pos_ptr.update_value(&process);
                                timer::set_variable_float("kingPos",king_pos.current);

                                //SP-only vars
                                let plot = plot_ptr.update_value(&process);
                                timer::set_variable_float("Plot",plot.current);
                                let filechoice = filechoice_ptr.update_value(&process);
                                timer::set_variable_float("fileChoice",filechoice.current);
                                let jevil_dance1 = jevil_dance_ptr1.update_value(&process);
                                timer::set_variable_float("Jevil Dance",jevil_dance1.current);
                                let jevil_dance2 = jevil_dance_ptr2.update_value(&process);
                                timer::set_variable_float("Jevil Dance (2)",jevil_dance2.current);
                                let final_textbox1 = final_textbox_ptr1.update_value(&process);
                                timer::set_variable_float("Final Textbox",final_textbox1.current);
                                let final_textbox2 = final_textbox_ptr2.update_value(&process);
                                timer::set_variable_float("Final Textbox (2)",final_textbox2.current);

                                //Ch1 splits and AC-pause go here
                                if settings.ch1_contact && !splits.contains("ch1_contact") && room_check_both(*room,"PLACE_CONTACT","room_krisroom") {
                                    split(&mut splits,"ch1_contact");
                                }
                                if settings.ch1_school && !splits.contains("ch1_school") && room_check_both(*room,"room_insidecloset","room_dark1") {
                                    split(&mut splits,"ch1_school");
                                }
                                if settings.ch1_cliffs && !splits.contains("ch1_cliffs") && room_check_both(*room,"room_dark7","room_dark_chase1") {
                                    split(&mut splits,"ch1_cliffs");
                                }
                                if settings.ch1_castle_town_door && !splits.contains("ch1_castle_town_door") && room_match(*room,"room_castle_darkdoor") && great_door_con.old == 7.0 && great_door_con.current == 21.0 {
                                    split(&mut splits,"ch1_castle_town_door");
                                }
                                if settings.ch1_castle_town_room && !splits.contains("ch1_castle_town_room") && room_check_both(*room,"room_castle_darkdoor","room_field_start") {
                                    split(&mut splits,"ch1_castle_town_room");
                                }
                                if settings.ch1_field && !splits.contains("ch1_field") && room_check_both(*room,"room_field4","room_field_checkers4") {
                                    split(&mut splits,"ch1_field");
                                }
                                if settings.ch1_board && !splits.contains("ch1_board") && room_check_both(*room,"room_field_checkersboss","room_forest_savepoint1") {
                                    split(&mut splits,"ch1_board");
                                }
                                if settings.ch1_enter_bake_sale && !splits.contains("ch1_enter_bake_sale") && room_check_both(*room,"room_forest_area3","room_forest_savepoint2") {
                                    split(&mut splits,"ch1_enter_bake_sale");
                                }
                                if settings.ch1_egg && !splits.contains("ch1_egg") && room_match(*room,"room_man") && msc.old != 601 && msc.current == 601 && choice.current == 0 {
                                    split(&mut splits,"ch1_egg");
                                }
                                if settings.ch1_enter_forest_maze && !splits.contains("ch1_enter_forest_maze") && room_check_both(*room,"room_forest_savepoint_relax","room_forest_maze1") {
                                    split(&mut splits,"ch1_enter_forest_maze");
                                }
                                if settings.ch1_susie_lancer_exit && !splits.contains("ch1_susie_lancer_exit") && room_check_both(*room,"room_forest_fightsusie","room_forest_afterthrash2") {
                                    split(&mut splits,"ch1_susie_lancer_exit");
                                }
                                if settings.ch1_get_captured && !splits.contains("ch1_get_captured") && room_check_both(*room,"room_forest_castlefront","room_cc_prison_cells") {
                                    split(&mut splits,"ch1_get_captured");
                                }
                                if settings.ch1_escape_prison && !splits.contains("ch1_escape_prison") && room_check_both(*room,"room_cc_prison_cells","room_cc_prisonlancer") {
                                    if tempVar == 1 {
                                        split(&mut splits,"ch1_escape_prison");
                                    } else tempVar = 1;
                                }
                            }
                            2 => {
                                let choice = choicer_ptr.update_value(&process);
                                timer::set_variable_float("Choice",choice.current);
                                let msc = msc_ptr.update_value(&process);
                                timer::set_variable_float("msc",msc.current);

                                let loaded_disk_bg = loaded_disk_bg_ptr.update_value(&process);
                                timer::set_variable_float("LoadedDisk BG",loaded_disk_bg.current);
                                let snowgrave = snowgrave_ptr.update_value(&process);
                                timer::set_variable_float("SnowGrave",snowgrave.current);

                                //change_game-only variables
                                let text2 = text_ptr2.update_value(&process);
                                timer::set_variable("Text (2)",text2.current.validate_utf8().unwrap_or_default());
                                let text3 = text_ptr3.update_value(&process);
                                timer::set_variable("Text (3)",text3.current.validate_utf8().unwrap_or_default());
                                let text4 = text_ptr4.update_value(&process);
                                timer::set_variable("Text (4)",text4.current.validate_utf8().unwrap_or_default());

                                //Ch2 splits and AC-pause go here


                            }
                            3 => {
                                let plot = plot_ptr.update_value(&process);
                                timer::set_variable_float("Plot",plot.current);

                                let knight_result = knight_result_ptr.update_value(&process);
                                timer::set_variable_float("Knight Result",knight_result.current);
                                let egg_timer = egg_timer_ptr.update_value(&process);
                                timer::set_variable_float("Egg Timer",egg_timer.current);
                                let mantle_outro = mantle_outro_ptr.update_value(&process);
                                timer::set_variable_float("Mantle Outro",mantle_outro.current);

                                //Ch3 splits and AC-pause go here


                            }
                            4 => {
                                let plot = plot_ptr.update_value(&process);
                                timer::set_variable_float("Plot",plot.current);

                                let mike_action = mike_action_ptr.update_value(&process);
                                timer::set_variable_float("Mike Action",mike_action.current);
                                let susie_sprite = susie_sprite_ptr.update_value(&process);
                                timer::set_variable_int("Susie Sprite",susie_sprite.current);
                                let player_x = player_x_ptr.update_value(&process);
                                timer::set_variable_float("Player X",player_x.current);
                                let player_y = player_y_ptr.update_value(&process);
                                timer::set_variable_float("Player Y",player_y.current);

                                //Ch4 splits and AC-pause go here

                            }
                            5 => {
                                let plot = plot_ptr.update_value(&process);
                                timer::set_variable_float("Plot",plot.current);
                                let choice = choicer_ptr.update_value(&process);
                                timer::set_variable_float("Choice",choice.current);
                                let pink_coins = pink_coins_ptr.update_value(&process);
                                timer::set_variable_float("Pink Coins",pink_coins.current);
                                let crt_start = crt_start_ptr.update_value(&process);
                                timer::set_variable_float("CRT Start",crt_start.current);

                                //Ch5 splits and AC-pause go here
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
