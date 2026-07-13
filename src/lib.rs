#![allow(nonstandard_style)]

use std::fs::{read};
use asr::{
    future::next_tick, PointerSize, Process, deep_pointer::DeepPointer,
    watcher::{Watcher,Pair}, Address, string::ArrayCString, signature::Signature, timer, timer::TimerState, time_util::Instant,
    settings::{Map,Gui,gui::Title}, file_format::pe
};
use std::collections::{HashMap, HashSet};
use core::time::Duration;
use asr::future::retry;
use crate::Textboxes::{*};
use crate::Ver::{*};

asr::async_main!(stable);

enum Ver {
    SP,
    D109,
    D110,
    D115,
    D119,
    Ch4_v102, //for Ch3
    Ch5_v244, //for All Items
    Ch5_v247
}

#[derive(Gui)]
enum PauseTiming {
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
enum AutoStart {
    /// Start the timer, resetting if it was already running
    #[default]
    AutoReset,
    /// Start the timer, unpause IGT if already running
    AutoStartAndUnpause,
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
    chapter_pause_timing : PauseTiming,
    ///Also unpause from loading a savefile
    ac_unpause_loadsave: bool,


    ///Chapter 1: The Beginning
    ch1_title : Title,
    ///CONTACT
    ch1_contact : bool,
    ///Enter Dark World (Bed Skip)
    ch1_bedskip : bool,
    ///Enter Dark World (from school)
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
    ///Enter Cyber World (Bed Skip)
    ch2_bedskip : bool,
    ///Enter Cyber World (from Librarby)
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
    ///Ending (Full Game)
    ch2_ending_ac : bool,
    ///Ending (IL)
    ch2_ending_il : bool,
    ///Ending (Demo/IL OST%)
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
    ///Thorny Done (Open PuppetScarf Chest in Castle Town with no space)
    ch2_thorny_ending : bool,


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
    ///Enter Dark Sanctuary (Chair Skip)
    ch4_chairskip : bool,
    ///Enter Castle Town
    ch4_enter_castle_town : bool,
    ///Start Mike Fight
    ch4_start_mike : bool,
    ///End Mike fight
    ch4_beat_mike : bool,
    ///Enter Noelle's House
    ch4_enter_mansion : bool,
    ///Enter Dark Sanctuary (True Reset)
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
    ///Ending (Full Game)
    ch4_ending : bool,
    ///Ending (IL)
    ch4_ending_il : bool,
    ///Ending (OST%)
    ch4_ending_ost : bool,


    ///All Bosses Splits
    #[heading_level = 2]
    ch4_ab_title : Title,
    ///Solve the Golden Piano puzzle
    ch4_golden_piano : bool,
    ///Enter Hammer of Justice battle room
    ch4_enter_hoj: bool,
    ///End Hammer of Justice battle
    ch4_hammer_of_justice : bool,
    ///Exit Hammer of Justice battle room
    ch4_exit_axe_room : bool,



    ///Chapter 5: Festival Day
    ch5_title : Title,
    ///Start/reset timer on loading Ch4 completion data?
    ch5_start_on_prev : Ch5StartOnPrev,
    ///Bed Skip
    ch5_bedskip : bool,
    ///Enter Castle Town
    ch5_enter_castle_town : bool,
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
    ///Susie's Idea
    ch5_end_flowery : bool,
    ///End final climb
    ch5_end_final_climb : bool,
    ///Omega Flowery Clash
    ch5_omega_flowery : bool,
    ///Seal Fountain 1
    ch5_fountain1 : bool,
    ///Seal Fountain 2
    ch5_fountain2 : bool,
    ///Ending (SRC rules)
    ch5_ending_src : bool,
    ///Ending (completion data timing) [NOT IMPLEMENTED YET]
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

/*fn former_match<T: PartialEq>(state : Pair<T>, val : T) -> bool {
    state.current == val && state.old != val
}

fn new_match<T: PartialEq>(state : Pair<T>, val : T) -> bool {
    state.old == val && state.current != val
}*/

/*fn room_match(cur_room : ArrayCString<64>, check_room : &str) -> bool {
    cur_room.validate_utf8().unwrap_or_default().strip_suffix("_ch1").unwrap_or_default() == check_room
}

fn room_check(room : &Pair<ArrayCString<64>>, dest : &str) -> bool {
    room_match(room.current,dest) && !room_match(room.old,dest)
}

fn room_check_both(room : &Pair<ArrayCString<64>>, orig : &str, dest : &str) -> bool {
    room_match(room.current,dest) && room_match(room.old,orig)
}*/

struct VarFinder {
    numAddr : Address,
    arrAddr : Address,
    ps : PointerSize
}

impl VarFinder {
    fn try_new(process: &Process, ps: PointerSize, instAddr: Address) -> Option<VarFinder> {
        let Ok(midAddr) = process.read_pointer_path::<u64>(instAddr, ps, &[0x0, 0x48]) else {
            return None;
        };
        Some(VarFinder {
            numAddr: Address::new(midAddr + 0x8),
            arrAddr: process.read_pointer(Address::new(midAddr + 0x10), ps).unwrap(),
            ps
        })
    }
    fn new(process: &Process, ps: PointerSize, instAddr: Address) -> VarFinder {
        let midAddr = process.read_pointer_path::<u64>(instAddr, ps, &[0x0, 0x48]).unwrap();
        VarFinder {
            numAddr: process.read_pointer(Address::new(midAddr + 0x8), ps).unwrap(),
            arrAddr: process.read_pointer(Address::new(midAddr + 0x10), ps).unwrap(),
            ps
        }
    }

    //Find a pointer to a specific variable, this is used to find and store permanent pointers for global variables
    fn findVarPtr(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> Address {
        for i in 0..process.read::<u32>(self.numAddr).unwrap_or_default() {
            let offset = i * 0x10;
            let stringID = process.read::<u32>(self.arrAddr + offset + 0x8).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            if stringsList.get(&(stringID - 100000)).unwrap_or(&String::from("")) == name {
                process.read_pointer(self.arrAddr + offset, self.ps).unwrap_or_default();
            }
        }
        Address::default()
    }

    //Populate a HashMap with pointers for variables from provided list
    fn populatePtrMap(&self, process: &Process, stringsList: &HashMap<u32, String>, pointerMap : &mut HashMap<String,Address>, names: &[&str]) {
        for i in 0..process.read::<u32>(self.numAddr).unwrap_or_default() {
            let offset = i * 0x10;
            let stringID = process.read::<u32>(self.arrAddr + offset + 0x8).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            let Some(name) = stringsList.get(&(stringID - 100000)) else {
                continue;
            };
            if names.contains(&name.as_str())  {
                pointerMap.insert(name.clone(),process.read_pointer(self.arrAddr + offset, self.ps).unwrap_or_default());
            }
        }
    }

    //Immediately read first value of a variable, this is used to read variable values from instances of objects without needing
    fn readVar<T: bytemuck::Pod + Default>(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> T {
        for i in 0..process.read::<u32>(self.numAddr).unwrap_or_default() {
            let offset = i * 0x10;
            let stringID = process.read::<u32>(self.arrAddr + offset + 0x8).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            if stringsList.get(&(stringID - 100000)).unwrap_or(&String::from("")) == name {
                //timer::set_variable("Address read from",process.read_pointer(self.arrAddr.add(offset as u64),self.ps).unwrap_or_default().);
                return process.read_pointer_path::<T>(self.arrAddr, self.ps, &[offset as u64, 0x0]).unwrap();
            }
        }
        T::default()
    }

    fn readStr<const len : usize>(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> String {
        for i in 0..process.read::<u32>(self.numAddr).unwrap_or_default() {
            let offset = i * 0x10;
            let stringID = process.read::<u32>(self.arrAddr + offset + 0x8).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            if stringsList.get(&(stringID - 100000)).unwrap_or(&String::from("")) == name {
                let res = process.read_pointer_path::<ArrayCString<len>>(self.arrAddr, self.ps, &[offset as u64, 0x0]).unwrap_or_default();
                return res.validate_utf8().unwrap_or_default().to_string();
                //return process.read_pointer_path::<ArrayCString<len>>(self.arrAddr, self.ps, &[offset as u64, 0x0]).unwrap_or_default().validate_utf8().unwrap_or_default().to_string();
            }
        }
        String::default()
    }
}

fn get_first_instance(process : &Process, ps : PointerSize, obj : Address) -> Option<Address> {
    let Ok(obj_prop) = process.read_pointer(obj.add(0x18),ps) else {
        return None;
    };
    let instCount = process.read::<u32>(obj_prop.add(0x78)).unwrap_or_default();
    if instCount == 0 { return None; }
    let mut node = process.read_pointer(obj_prop.add(0x68),ps).unwrap_or_default();
    process.read_pointer(node.add(0x10),ps).ok()
}

fn get_all_instances(process : &Process, ps : PointerSize, obj : Address) -> Vec<Address> {
    let mut vec = Vec::<Address>::new();
    let Ok(obj_prop) = process.read_pointer(obj.add(0x18),ps) else {
        return vec;
    };
    let instCount = process.read::<u32>(obj_prop.add(0x78)).unwrap_or_default();
    if instCount == 0 { return vec; }
    let mut node = process.read_pointer(obj_prop.add(0x68),ps).unwrap_or_default();
    for i in 0..instCount {
        vec.push(process.read_pointer(node.add(0x10),ps).unwrap_or_default());
        if i<instCount-1 { node = process.read_pointer(node,ps).unwrap_or_default(); }
    }
    vec
}

#[derive(PartialEq,Eq,Hash)]
enum Textboxes {
    Ch1GoToSleep,
    Bagels,
    FreezeRing,
    ThornRing,
    Thorny,
    SusieFellAsleep,
    Ch2TorielLast,
    Ch4Egg,
    //PrincessRBN,
}

fn update_text_open(process : &Process, ps : PointerSize, stringsList: &HashMap<u32, String>, writer : &Address, textWatchers : &mut HashMap<Textboxes,Watcher<bool>>, text : &HashMap<Textboxes,String>) {
    let instVec = get_all_instances(process, ps, *writer);
    timer::set_variable_int("number of obj_writer instances",instVec.len());
    if instVec.len() == 0 { return; }
    timer::set_variable("obj_writer address",format!("{:X}",instVec[0].value()).as_str());
    let strVec = instVec.iter().map(|x| {
        let finder = VarFinder::new(process,ps,*x);
        let varPtr = finder.findVarPtr(process,stringsList,"mystring");
        return process.read_pointer_path::<ArrayCString<128>>(varPtr,ps,&[0x0,0x0,0x0,0x0]).unwrap_or_default().validate_utf8().unwrap_or_default().to_string();
    }).collect::<Vec<String>>();
    timer::set_variable("last read string",strVec[0].as_str());
    for (key,watcher) in textWatchers {
        watcher.update_infallible(strVec.contains(&text[key]));
    }
}

fn text_match(txt : ArrayCString<128>, en : &str, jp : &str) -> bool {
    txt.matches(en) || txt.matches(jp)
}

fn text_open_check(txt : &Pair<ArrayCString<128>>, en : &str, jp : &str) -> bool {
    //text_match(txt.current,en,jp) && !text_match(txt.old,en,jp)
    txt.bytes_changed() && text_match(txt.current,en,jp)
}

fn text_close_check(txt : &Pair<ArrayCString<128>>, en : &str, jp : &str) -> bool {
    //text_match(txt.old,en,jp) && !text_match(txt.current,en,jp)
    txt.bytes_changed() && text_match(txt.old,en,jp)
}

fn text_open_check_multipointer(txts : &Vec<&Pair<ArrayCString<128>>>, en : &str, jp : &str) -> bool {
    for txt in txts {
        if text_open_check(txt,en,jp) {
            return true;
        }
    }
    false
}
fn text_close_check_multipointer(txts : &Vec<&Pair<ArrayCString<128>>>, en : &str, jp : &str) -> bool {
    for txt in txts {
        if text_close_check(txt,en,jp) {
            return true;
        }
    }
    false
}

fn read_setting(key : &str) -> bool {
    match Map::load().get(key) {
        Some(x) => x.get_bool().unwrap_or(false),
        None => false
    }
}

fn start(auto_start : &AutoStart, splits : &mut HashSet<String>) {
    match auto_start {
        AutoStart::AutoReset => {
            splits.clear();
            timer::reset();
            timer::start();
        }
        AutoStart::AutoStartAndUnpause => {
            timer::start();
            timer::resume_game_time();
        }
        AutoStart::AutoStart => {
            //splits.clear();
            timer::start();
        }
        AutoStart::Off => ()
    }
}

const IL_Pauses : [&str;5] = ["ch1_ending","ch2_ending_il","ch3_ending","ch4_ending_il","ch5_ending_src"];
const AC_Pauses : [&str;5] = ["ch1_ending","ch2_ending_ac","ch3_ending","ch4_ending_ac","ch5_ending_src"]; //Ch5 ending in this set will change after Ch6 release
const OST_Pauses : [&str;5] = ["ch1_ending_ost","ch2_ending_ac","ch3_ending_ost","ch4_ending_ost","ch5_ending_src"];
const OST_LateCh2_Pauses : [&str;5] = ["ch1_ending_ost","ch2_ending_ost","ch3_ending","ch4_ending_ost","ch5_ending_src"];

//also does some handling for IGT pausing and unpausing to simplify code elsewhere
fn split(splits : &mut HashSet<String>, settings : &Settings, name : &str, already_checked : bool) {
    if settings.ac_pause_timer {
        if name == "resume_igt" {
            timer::resume_game_time();
            return;
        }
        //NOTE: Ch5 ending pauses are currently meant for TRACABARTPEEG and assume the category will follow the main rules regarding 5A end timing. After Ch6 releases, the AllChapters mode pause timing for Ch5 will presumably change to completion data
        if match &settings.chapter_pause_timing {
            PauseTiming::SingleChapter => IL_Pauses,
            PauseTiming::AllChapters => AC_Pauses,
            PauseTiming::OST => OST_Pauses,
            PauseTiming::OSTLateCh2 => OST_LateCh2_Pauses
        }.contains(&name) {
            timer::pause_game_time();
        }
    }
    if !already_checked && (name == "" || splits.contains(name) || !read_setting(name) ) {
        return;
    }
    splits.insert(name.to_string());
    asr::print_message(format!("Split triggered: {}",name).as_str());
    timer::split();
}

struct VarTrack<T: Clone + bytemuck::Pod> {
    pointer : Option<DeepPointer<16>>,
    watcher : Watcher<T>,
}
impl<T: Clone + bytemuck::Pod> VarTrack<T> {
    fn disabled() -> VarTrack<T> {
        VarTrack {
            watcher: Watcher::<T>::new(),
            pointer: None,
        }
    }
    fn new(module_base : Address, pointer_size: PointerSize, offsets : &[u64]) -> VarTrack<T> {
        VarTrack {
            watcher: Watcher::<T>::new(),
            pointer: match offsets {
                &[0] => None,
                x => Some(DeepPointer::new(module_base, pointer_size, x)),
            }
        }
    }
    fn update_value(&mut self, process: &Process) -> &Pair<T> {
        if self.pointer.is_none() {
            return self.watcher.update_infallible(T::zeroed())
        }
        let value = self.pointer.unwrap().deref(&process).unwrap_or_else(|_e| T::zeroed());
        self.watcher.update_infallible(value)
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

                let (DELTARUNE, _) = process.wait_module_range("DELTARUNE.exe").await;
                let module_size = pe::read_size_of_image(&process, DELTARUNE).unwrap_or_default();
                timer::set_variable("Module Address",format!("{:X}",DELTARUNE.value()).as_str());


                path = path.replace("DELTARUNE.exe", "data.win");
                timer::set_variable("Path", path.as_str());
                let md5 = &format!("{:X}", md5::compute(read(path).unwrap_or_default()));
                timer::set_variable("MD5", md5);
                let version = match md5.to_uppercase().as_str() {
                    "A88A2DB3A68C714CA2B1FF57AC08A032" | //SP-EN vanilla
                    "047C11435B1C592EC731BFF3B9C5B0CF" | //SP-EN 30tbps
                    "22008370824A37BAEF8948127963C769" | //SP-JP vanilla
                    "E05433FE679BC91E3809C1138E3A8EA1" => SP, //SP-JP 30tbps
                    "616C5751AC9FC584AF250F1B04474AFD" | //demo 1.09 vanilla Itch
                    "05689183497E58838E99B897F2E0E6AC" | //demo 1.09 30tbps Itch
                    "267A8ABE468D824222810201F00003BE" | //demo 1.09 vanilla Steam
                    "272A16964597ED6DC8D2393ED051D3CE" => D109, // demo 1.09 30tbps Steam
                    "5FBE01F2BC1C04F45D809FFD060AC386" | //demo 1.10 vanilla Itch
                    "A37C77A4310D2D6A6C2AF18294AAAE7A" | //demo 1.10 30tbps Itch
                    "CD77A63D7902990DBC704FE32B30700A" | //demo 1.10 vanilla Steam
                    "758C8862F22F778FDEAFE25FBCD1F4EC" => D110, //demo 1.10 30tbps Steam
                    "ED4568BAB864166BFD6322CEEB3FB544" | //demo 1.15 vanilla
                    "6BD6D1381C194C0F456B184CB48D132D" => D115, //demo 1.15 30tbps
                    "7AD299A8B33FA449E20EDFE0FEDEDDB2" | //demo 1.19 vanilla
                    "FD0857E6A3AF3AA74E5E00F15AEA5224" => D119, //demo 1.19 30tbps
                    "B5EF0EEC9554C491777D6C4E93E0DF76" | //v1.02 vanilla
                    "40A8185886A8A1A2BE996BC57DE3D916" => Ch4_v102, //v1.02 30tbps
                    "DDEDBBD10FF129B49C64DBEFAA763C6A" | //v244 vanilla
                    "4A9C69B42E442B673395B3253F292F17" | // v244 30 TBPS mod
                    "42B66B41B6CEA12FB54219E9D31E5DC8" | // v244 Item tracker mod
                    "D0420C09A5DEBD6176EA24A1FE1EE3E3" => Ch5_v244, // OST% tracker mod
                    "908643B7593B000F5B6C61BB484D086A" | //v247 vanilla
                    "80A63475EF69529B612F9DCA75AF4CC5" | //v247 30tbps
                    "3217F3BFE82C3E4AA8EE2E9E3A4F4E14" | //v247 Item Tracker
                    "21CDD09EEADBCC77535AB2BB3412259A" => Ch5_v247, //v247 OST tracker
                    _ => {
                        timer::set_variable("version","Invalid");
                        loop {next_tick().await;}
                    },
                };
                timer::set_variable("version", match version {
                    SP => "SURVEY_PROGRAM",
                    D109 => "Demo v1.09",
                    D110 => "Demo v1.10",
                    D115 => "Demo v1.15",
                    D119 => "Demo v1.19",
                    Ch4_v102 => "Ch1-4 v1.02",
                    Ch5_v244 => "Ch1-5, Ch5 v0.0.244",
                    Ch5_v247 => "Ch1-5, Ch5 v0.0.247",
                });

                let ps = match version {
                    SP | D109 | D110 | D115 => ps32,
                    _ => ps64
                };

                let mut chapter = 0;

                if ps == ps64
                { //the directory only changes with change_game which starts a whole new process for the autosplitter to attach to, so we only need to read it once per process attached
                    let mut _dir : ArrayCString<256>;
                    loop {
                        _dir = process.read_pointer_path::<ArrayCString<256>>(DELTARUNE, ps, match version {
                            SP|D109|D110|D115 => n,
                            Ch5_v244 | Ch5_v247 => &[0x8BA818,0],
                            Ch4_v102 => &[0x8B2818,0],
                            D119 => &[0x8D06E0,0],
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

                let mut stringsList = HashMap::<u32,String>::new();
                {
                    let sListPtr = process.read_pointer(DELTARUNE.add(0x5FCD08),ps).unwrap();
                    let strNum = process.read::<u32>(sListPtr.add_signed(-0x18)).unwrap();
                    asr::print_message(format!("StringsList length: {}",strNum).as_str());
                    for i in 0..strNum {
                        //let entryAddr = process.read_pointer(sListPtr.add(8*i as u64), ps).unwrap();
                        let namePtr = process.read_pointer(sListPtr.add(8*i as u64), ps).unwrap();
                        let _name = process.read::<ArrayCString<64>>(namePtr).unwrap_or_default();
                        let name = _name.validate_utf8().unwrap_or_default();
                        if name != "" {
                            stringsList.insert(i, name.to_string());
                            if name == "plot" {
                                asr::print_limited::<64>(&format_args!("plot found at StringID {}",i))
                            }
                        }
                    }
                }
                asr::print_message(format!("Number of actual strings: {}",stringsList.len()).as_str());
                //asr::print_message(format!("plot's String index is {}",string_ids["plot"]).as_str());

                let mut obj_addr_map = HashMap::<String,Address>::new();
                loop {
                    let objArrBase = process.read_pointer(DELTARUNE.add(0x6A7A98),ps).unwrap();
                    let Ok(objNum) = process.read::<u32>(objArrBase.add(0xC)) else {
                        continue;
                    };
                    asr::print_message(format!("Number of objects: {}",objNum).as_str());
                    let arr = process.read_pointer(objArrBase,ps).unwrap();
                    for i in 0..objNum {
                        let objAddr = process.read_pointer(arr.add(i as u64 * 0x10),ps).unwrap();
                        let _name = process.read_pointer_path::<ArrayCString<64>>(objAddr,ps,&[0x18,0x0,0x0]).unwrap_or_default();
                        let name = _name.validate_utf8().unwrap_or_default();
                        if name != "" {
                            if name == "obj_writer" {
                                asr::print_message(format!("obj_writer found at {}",objAddr).as_str());
                            }
                            obj_addr_map.insert(name.to_string(),objAddr);
                        }
                    }
                    break;
                }
                let mut objs = String::from("");
                for k in obj_addr_map.keys() {
                    objs += k.as_str();
                    objs += ", ";
                }
                asr::print_message(&objs);



                let globalOffset : u64 = match version {
                    SP => 0x48E5DC,
                    D109 | D110 => 0x6FCF38,
                    D115 => 0x6FE860,
                    D119 | Ch4_v102 => 0x6A1CA8,
                    Ch5_v244 | Ch5_v247 => 0x6A9CA8,
                };

                let globalFinder = retry(|| VarFinder::try_new(&process,ps,DELTARUNE.add(globalOffset))).await;
                asr::print_message("Found global");

                // sound stuff (pointer only varies by runner version)

                let mut snd_ptr = VarTrack::<ArrayCString<256>>::new(DELTARUNE,ps,match version {
                    SP => n,
                    D109|D110 => &[0x4E0794, 0x58, 0xC0,  0x40, 0x0],
                    D115 => &[0x4E20B4, 0x58, 0xC0,  0x40, 0x0],
                    D119 | Ch4_v102 => &[0x6A3818, 0x60, 0xD0, 0x58, 0x0],
                    Ch5_v244 | Ch5_v247 => &[0x6AB818, 0x60, 0xD0, 0x58, 0x0],
                });

                let mut mus_ptr = VarTrack::<ArrayCString<256>>::new(DELTARUNE,ps,match version {
                    SP => n,
                    D109|D110 => &[0x4DFF58, 0x0,  0x44,  0x0],
                    D115 => &[0x4E1878, 0x0,  0x0,   0x0],
                    D119 | Ch4_v102 => &[0x6A2F90, 0x0,  0x0,  0x0],
                    Ch5_v244 | Ch5_v247 => &[0x6AAF90, 0x0,  0x0,  0x0],
                });

                let text_en = HashMap::from([
                    (Ch1GoToSleep,r"* (You decided to go to bed.)/%".to_string()),
                    (Bagels,r"* (You were crushed under the&||weight of 400 bagels and&||defeated instantly...)/%".to_string()),
                    (FreezeRing,r"* (You got the FreezeRing.)/%".to_string()),
                    (ThornRing,r"\S1* (You got the ThornRing.)/%".to_string()),
                    (Thorny,r"* (You have too many \cYWEAPONs\cW to&||take \cYPuppetScarf\c0.)/%".to_string()),
                    (SusieFellAsleep,r"* (... Susie fell asleep.)/%".to_string()),
                    (Ch2TorielLast,r"\E1* ... they're already&||asleep.../%".to_string()),
                    (Ch4Egg,r"* (An Egg was picked up from a&||nearby easel.)/%".to_string())
                ]);

                let mut ch2_texts = HashMap::from([
                    (Bagels,Watcher::<bool>::new())
                ]);


                //DEVICE_NAMER.EVENT

                let mut namer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => n,
                    D109 => &[0x6EF220, 0xD4, 0x5C,  0x20, 0x24,  0x10, 0x9C,  0x0],
                    D110 => &[0x6EF220, 0xD4, 0x5C,  0x20, 0x24,  0x10, 0x2F4, 0x0],
                    D115 => &[0x6F0B48, 0xD4, 0x5C,  0x20, 0x24, 0x10, 0xFC,  0x0],
                    D119 => &[0x8B2790, 0x178, 0x70,  0x38, 0x48,  0x10, 0x3B0, 0x0],
                    Ch4_v102 => match chapter {
                        2 | 3 => &[0x8B2790, 0x178, 0x70, 0x38, 0x48, 0x10, 0x60, 0x0],
                        4 => &[0x8B2790, 0x178, 0x70, 0x38, 0x48, 0x10, 0x280, 0x0],
                        _ => n
                    }
                    Ch5_v244 => match chapter {
                        2 => &[0x8BA790, 0x178, 0x70,  0x38,   0x48,  0x10,  0x90,  0x0],
                        3 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x120, 0x0],
                        4 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48,  0x10,  0x40,  0x0],
                        5 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x170, 0x0],
                        _ => n
                    }
                    Ch5_v247 => match chapter {
                        2 => &[0x8BA790, 0x178, 0x70,  0x38,   0x48,  0x10,  0x90,  0x0],
                        3 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x120, 0x0],
                        4 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48,  0x10,  0x40,  0x0],
                        5 => &[0x8BA790, 0x178, 0x70,   0x38,   0x48, 0x10, 0x220, 0x0],
                        _ => n
                    }
                });

                //Global variables
                //(note: for global.flag[N] values, the last offset is the only difference between different flags' locations, and is equal to 16x the flag's index number - which you can get either by directly multiplying by 16 and putting it in as a decimal number, or by converting to hex then adding a trailing zero.)

                let mut old_chapter_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    D109 | D110 => &[0x6FCF38, 0x30, 0x24D8, 0x0],
                    D115 => &[0x6FE860, 0x30, 0x2F34, 0x80],
                    _ => n
                });

                let mut filechoice_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    SP => &[0x48E5DC, 0x27C, 0x488, 0x4D0],
                    _ => n
                });

                //DOESN'T WORK FOR CHAPTER 1?
                let mut fighting_ptr = VarTrack::<f64>::new(DELTARUNE,ps, match version {
                    SP => n,
                    D109 | D110 => &[0x6FCF38, 0x30, 0x4F8,  0x0],
                    D115 => &[0x6FE860, 0x30, 0xA758, 0x0],
                    D119 => match chapter {
                        1 => n, //&[0x6A1CA8, 0x48, 0x10, 0x32F0, 0x710], (doesn't work points to some tiny value)
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0xBB0],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        1 => n, //&[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x720], (other LTS fight_ch1 pointers don't work so I assume this one doesn't either
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x100,  0x0],
                        3 => &[0x6A1CA8, 0x48,  0x10,   0x1190, 0x370],
                        4 => &[0x6A1CA8, 0x48,  0x10,   0x72B0, 0x370],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        1 => n, //&[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x740], (points to Dark Dollars instead)
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x100,  0x0],
                        3 => &[0x6A9CA8, 0x48,  0x10,   0x1190, 0x370],
                        4 => &[0x6A9CA8, 0x48,  0x10,   0x72B0, 0x370],
                        5 => &[0x6A9CA8, 0x48,  0x10,   0x820,  0x70],
                        _ => n
                    }
                });

                let mut plot_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    D109|D110|D115|D119 => n,
                    SP => &[0x48E5DC, 0x27C, 0x488, 0x500],
                    Ch4_v102 => match chapter {
                        3 => &[0x6A1CA8, 0x48,  0x10,   0x1000, 0x250],
                        4 => &[0x6A1CA8, 0x48,  0x10,   0x2F40, 0x30],
                        _ => n
                    },
                    Ch5_v244 | Ch5_v247 => match chapter {
                        3 => &[0x6A9CA8, 0x48,  0x10,   0x1000, 0x250],
                        4 => &[0x6A9CA8, 0x48,  0x10,   0x2F70, 0x30],
                        _ => n
                    },
                });

                let mut choicer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => &[0x48E5DC, 0x27C, 0x28,  0x40],
                    D109 | D110 => &[0x6FCF38, 0x30, 0x18C0, 0x0],
                    D115 => &[0x6FE860, 0x30, 0xBA0,  0xC0],
                    D119 => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0x0],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0x0],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x10],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x7870, 0x0],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x10],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x7870, 0x0],
                        5 => &[0x6A9CA8, 0x48,  0x10,   0x150,  0x20],
                        _ => n
                    }
                });

                let mut msc_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => &[0x48E5DC, 0x27C, 0x28,  0x140],
                    D109 | D110 => &[0x6FCF38, 0x30, 0x354C, 0x0],
                    D115 => &[0x6FE860, 0x30, 0x17AC, 0x0],
                    D119 => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10, 0x32F0, 0xF0],
                        2 => &[0x6A1CA8, 0x48, 0x10, 0x7790, 0x130],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        1 => &[0x6A1CA8, 0x48, 0x10,  0x1E40, 0x100],
                        2 => &[0x6A1CA8, 0x48,  0x10,  0x7310, 0x0],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        1 => &[0x6A9CA8, 0x48, 0x10,  0x1E40, 0x100],
                        2 => &[0x6A9CA8, 0x48,  0x10,  0x7310, 0x0],
                        _ => n
                    }
                });

                //globals with chapter-specific relevance (e.g. flags)

                let mut knight_result_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x6A1CA8, 0x48,  0x10,   0x6A70, 0x0,  0x90, 0x4170],
                        Ch5_v244 | Ch5_v247 => &[0x6A9CA8, 0x48,  0x10,   0x6A70, 0x0,  0x90, 0x4170],
                    }
                    _ => n
                });

                let mut pink_coins_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    5 => match version {
                        SP|D109|D110|D115|D119|Ch4_v102 => n,
                        Ch5_v244 => &[0x6A9CA8, 0x48,  0x10,   0x6BB0, 0x0,  0x90, 0x5200],
                        Ch5_v247 => &[0x6A9CA8, 0x48,  0x10,   0x6BA0, 0x0,  0x90, 0x5200],
                    }
                    _ => n
                });




                //recurring objects across chapters

                let mut text_ptr1 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match version {
                    SP => n,
                    D109 | D110 => &[0x6FCE4C, 0x8,  0x144, 0x24, 0x10, 0x5A0, 0x0, 0x0, 0x0],
                    D115 => &[0x6FE774, 0x8,  0x144, 0x24, 0x10, 0x0, 0x0, 0x0, 0x0],
                    D119 => match chapter {
                        1 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0xF0, 0x0, 0x0, 0x0],
                        2 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x5F0, 0x0, 0x0, 0x0],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        1 => &[0x8C2008, 0x10, 0x1A0, 0x48,   0x10,  0x390, 0x0, 0x0, 0x0],
                        2 => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x6F0, 0x0,   0x0,  0x0],
                        4 => &[0x8C2008, 0x10,  0x1A0,  0x48,   0x10,  0x300, 0x0,   0x0, 0x0],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        1 => &[0x8CE220, 0x10, 0x1A0, 0x48,   0x10,  0x390, 0x0, 0x0, 0x0],
                        2 => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x6F0, 0x0,   0x0,  0x0],
                        4 => &[0x8CE220, 0x10,  0x1A0,  0x48,   0x10,  0x310, 0x0,   0x0,  0x0],
                        _ => n
                    }
                });

                let mut text_ptr2 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        SP|D109|D110|D115 => n,
                        D119 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x6D0, 0x0, 0x0, 0x0],
                        Ch4_v102 => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x700, 0x0,   0x0,  0x0],
                        Ch5_v244 | Ch5_v247 => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x700, 0x0,   0x0,  0x0],
                    }
                    _ => n
                });
                let mut text_ptr3 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        SP|D109|D110|D115 => n,
                        D119 => &[0x8C2008, 0x10, 0x1A0, 0x48, 0x10, 0x6F0, 0x0, 0x0, 0x0],
                        Ch4_v102 => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x710, 0x0,   0x0,  0x0],
                        Ch5_v244 | Ch5_v247 => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x710, 0x0,   0x0,  0x0],
                    }
                    _ => n
                });
                let mut text_ptr4 = VarTrack::<ArrayCString<128>>::new(DELTARUNE,ps,match chapter {
                    2 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x8C2008, 0x10,  0x1A0, 0x48,   0x10,  0x7E0, 0x0,   0x0,  0x0],
                        Ch5_v244 | Ch5_v247 => &[0x8CE220, 0x10,  0x1A0, 0x48,   0x10,  0x7E0, 0x0,   0x0,  0x0],
                    }
                    _ => n
                });


                let mut susie_sprite_ptr = VarTrack::<i32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x69FA98, 0x0,   0x1008, 0x50,   0x158, 0x10,  0xBC],
                        Ch5_v244 | Ch5_v247 => &[0x6A7A98, 0x0,   0x1018, 0x50,   0x158, 0x10,  0xBC],
                    }
                    _ => n
                });

                let mut player_x_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x69FA98, 0x0,   0x198,  0x0,    0x50,  0x158, 0x10,  0xE8],
                        Ch5_v244 | Ch5_v247 => &[0x6A7A98, 0x0,   0x1A8,  0x0,    0x50,  0x158, 0x10,  0xE8],
                    }
                    _ => n
                });

                let mut player_y_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x69FA98, 0x0,   0x198,  0x0,    0x50,  0x158, 0x10,  0xEC],
                        Ch5_v244 | Ch5_v247 => &[0x6A7A98, 0x0,   0x1A8,  0x0,    0x50,  0x158, 0x10,  0xEC],
                    }
                    _ => n
                });



                //Ch1 objects

                let mut great_door_con_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => &[0x48BDEC, 0xC,  0x60, 0x10, 0x10,  0x0],
                    D109 | D110 => &[0x6EF220, 0x84, 0x24,  0x10, 0x18,  0x0],
                    D115 => &[0x6F0B48, 0x84, 0x24,  0x10, 0x18, 0x0],
                    D119 => match chapter {
                        1 => &[0x8B2790, 0xE0,  0x48,  0x10, 0x0,   0x0],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        1 => &[0x8B2790, 0xE0, 0x48,  0x10,   0x30,  0x0],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        1 => &[0x8BA790, 0xE0, 0x48,  0x10,   0x30,  0x0],
                        _ => n
                    }
                });

                let mut king_pos_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match version {
                    SP => &[0x6AEB80, 0x4, 0x178, 0x80, 0xC8, 0x8, 0xB4],
                    D109 | D110 => &[0x6F1394, 0x4, 0x140, 0x68, 0x3C, 0x8, 0xB0],
                    D115 => &[0x6F2CBC, 0x4, 0x140, 0x68, 0x3C, 0x8, 0xB0],
                    D119 => match chapter {
                        1 => &[0x69FA98, 0x0, 0x530, 0x50, 0x158, 0x10, 0xE8],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        1 => &[0x69FA98, 0x0,  0x560, 0x50,   0x158, 0x10,  0xE8],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        1 => &[0x6A7A98, 0x0,  0x560, 0x50,   0x158, 0x10,  0xE8],
                        _ => n
                    }
                });



                //SP-specific object checks

                let mut jevil_dance_ptr1 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => &[0x48BDEC, 0x78, 0x60, 0x10, 0x10,  0x0],
                    _ => n
                });
                let mut jevil_dance_ptr2 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => &[0x48BDEC, 0x7C, 0x60, 0x10, 0x10,  0x0],
                    _ => n
                });
                let mut final_textbox_ptr1 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => &[0x48BDEC, 0x98, 0x60, 0x10, 0x274, 0x0],
                    _ => n
                });
                let mut final_textbox_ptr2 = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => &[0x48BDEC, 0x9C, 0x60, 0x10, 0x274, 0x0],
                    _ => n
                });



                //Ch2 objects

                let mut loaded_disk_bg_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => n,
                    D109 => &[0x6EF220, 0x84, 0x24,  0x10, 0x3D8, 0x0],
                    D110 => &[0x6EF220, 0x84, 0x24,  0x10, 0x87C, 0x0],
                    D115 => &[0x6F0B48, 0x84, 0x24,  0x10, 0x0,  0x0],
                    D119 => match chapter {
                        2 => &[0x8B2790, 0xE0,  0x48,  0x10, 0x3C0, 0x0],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        2 => &[0x8B2790, 0xE0,  0x48,  0x10,   0xC70, 0x0],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        2 => &[0x8BA790, 0xE0,  0x48,  0x10,   0xCA0, 0x0],
                        _ => n
                    }
                });

                let mut snowgrave_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match version {
                    SP => n,
                    D109 | D110 => &[0x6EF220, 0xF4, 0x27C, 0x6C, 0x5C,  0x20, 0x144, 0x24, 0x10, 0xC0, 0x0],
                    D115 => &[0x6F0B48, 0xF4, 0x27C, 0x6C, 0x5C, 0x20, 0x144, 0x24, 0x10, 0x120, 0x0],
                    D119 => match chapter {
                        2 => &[0x8B2790, 0x1A0, 0x3B0, 0x88, 0x70,  0x38, 0x1A0, 0x48, 0x10, 0x3D0, 0x0],
                        _ => n
                    }
                    Ch4_v102 => match chapter {
                        2 => &[0x8B2790, 0x1A0, 0x3B0, 0x88,   0x70,  0x38,  0x1A0, 0x48, 0x10, 0xA0, 0x0],
                        _ => n
                    }
                    Ch5_v244 | Ch5_v247 => match chapter {
                        2 => &[0x8BA790, 0x1A0, 0x3B0, 0x88,   0x70,  0x38,  0x1A0, 0x48, 0x10, 0x80, 0x0],
                        _ => n
                    }
                });





                //Ch3 objects

                let mut egg_timer_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x8B2790, 0x1E8, 0x530,  0x38,   0x48, 0x10, 0x290, 0x0],
                        Ch5_v244 | Ch5_v247 => &[0x8BA790, 0x1E8, 0x40,   0x38,   0x48, 0x10, 0x330, 0x0],
                    }
                    _ => n
                });
                let mut mantle_outro_ptr = VarTrack::<f32>::new(DELTARUNE,ps,match chapter {
                    3 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x69FA98, 0x0,   0x19B0, 0x18,   0x50, 0x10, 0xD0],
                        Ch5_v244 | Ch5_v247 => &[0x6A7A98, 0x0,   0x19B0, 0x18,   0x50, 0x10, 0xD0],
                    }
                    _ => n
                });



                //Ch4 objects

                let mut mike_action_ptr = VarTrack::<f64>::new(DELTARUNE,ps,match chapter {
                    4 => match version {
                        SP|D109|D110|D115|D119 => n,
                        Ch4_v102 => &[0x8B2790, 0x1A0, 0x2F0,  0x90,   0x78,  0x38,  0x198, 0x48, 0x10, 0x140, 0x0],
                        Ch5_v244 | Ch5_v247 => &[],
                    }
                    _ => n
                });


                //Ch5 objects

                let mut crt_start_ptr = VarTrack::<i32>::new(DELTARUNE,ps,match chapter {
                    5 => match version {
                        SP|D109|D110|D115|D119|Ch4_v102 => n,
                        Ch5_v244 | Ch5_v247 => &[0x6A7A98, 0x0,   0x1910, 0x8,    0x18, 0x68, 0x10,  0xE4],
                    }
                    _ => n
                });

                let mut tempVar = 0;
                let mut SPEndingTriggered = false;
                let mut ost_end_active = false;
                let mut ost_end_started = Instant::now();

                let mut bagel_door = false;







                // TODO: Load some initial information from the process.
                loop {
                    settings.update();

                    if matches!(version,D109|D110|D115) {
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

                    let cur_room = room.current.validate_utf8().unwrap_or_default().trim_end_matches("_ch1");
                    let prev_room = room.old.validate_utf8().unwrap_or_default().trim_end_matches("_ch1");
                    //asr::timer::set_variable("Room Name Pointer Address",format!("{:X}",room_name_addr0.value()).as_str());
                    //asr::timer::set_variable("Room Name Address",format!("{:X}",room_name_addr.value()).as_str());
                    timer::set_variable("Room Name",cur_room);

                    timer::set_variable_float("Plot (global read)",globalFinder.readVar::<f64>(&process,&stringsList,"plot"));

                    timer::set_variable_float("Namer Event (dynamic)",match obj_addr_map.get("DEVICE_NAMER") {
                        Some(obj) => {
                            match get_first_instance(&process,ps,*obj) {
                                Some(inst) => match VarFinder::try_new(&process,ps,inst) {
                                    Some(finder) => finder.readVar::<f64>(&process,&stringsList,"event"),
                                    None => 0.0
                                }
                                None => 0.0,
                            }
                        }
                        None => 0.0,
                    });

                    update_text_open(&process,ps,&stringsList,obj_addr_map.get("obj_writer").unwrap(),&mut ch2_texts,&text_en);
                    if ch2_texts[&Bagels].pair.is_some() {
                        timer::set_variable("Bagels textbox open",ch2_texts[&Bagels].pair.unwrap().current.to_string().as_str());
                    }

                    let state = timer::state();

                    if state == TimerState::NotRunning || state == TimerState::Ended {
                        tempVar = 0;
                        SPEndingTriggered = false;
                        ost_end_active = false;
                        if !splits.is_empty() { splits.clear(); }
                    }

                    match chapter {
                        //logic for autostart, autoreset, and continuing game time
                        0 => (),
                        1 => {
                            if room.changed() && cur_room == "PLACE_CONTACT" {
                                SPEndingTriggered = false;
                                tempVar = 0;
                                ost_end_active = false;
                                start(&settings.auto_start,&mut splits);
                            }
                        }
                        5 => {
                            let namer_event = namer_ptr.update_value(&process);
                            timer::set_variable_float("Namer Event",namer_event.current);
                            if !matches!(settings.ch5_start_on_prev,Ch5StartOnPrev::Exclusively) {
                                if cur_room == "PLACE_MENU"
                                {
                                    if namer_event.current == 75.0 && namer_event.old == 74.0 {
                                        tempVar = 0;
                                        ost_end_active = false;
                                        start(&settings.auto_start,&mut splits);
                                    }
                                }
                            }
                            if !matches!(settings.ch5_start_on_prev,Ch5StartOnPrev::No) {
                                if prev_room == "PLACE_MENU" && cur_room == "room_krisroom" && namer_event.old != 75.0 {
                                    tempVar = 0;
                                    ost_end_active = false;
                                    start(&settings.auto_start,&mut splits);
                                }
                            }
                        }
                        _ => {
                            if cur_room == "PLACE_MENU"
                            {
                                let namer_event = namer_ptr.update_value(&process);
                                timer::set_variable_float("Namer Event",namer_event.current);
                                if namer_event.current == 75.0 && namer_event.old != 75.0 {
                                    tempVar = 0;
                                    ost_end_active = false;
                                    start(&settings.auto_start,&mut splits);
                                }
                            }
                        }
                    }


                    // if we're not in the middle of a run, no reason to do anything not related to autostart (note that IGT pauses don't affect whether the timer state counts as running or paused)
                    if timer::state() == TimerState::Running {
                        let fighting = fighting_ptr.update_value(&process);
                        timer::set_variable_float("fighting",fighting.current); //not tracked for Chapter 1 (except in 32-bit 1+2 Demo versions)
                        //the next few vars are not in SP
                        let text = text_ptr1.update_value(&process);
                        timer::set_variable("text",text.current.validate_utf8().unwrap_or_default());
                        let snd = snd_ptr.update_value(&process);
                        timer::set_variable("snd",snd.current.validate_utf8().unwrap_or_default());
                        let mus = mus_ptr.update_value(&process);
                        timer::set_variable("mus",mus.current.validate_utf8().unwrap_or_default());

                        match chapter {
                            // Chapter 1 logic
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

                                // OST% ending (delayed split after room transition)
                                if ost_end_active && ost_end_started.elapsed() >= Duration::from_millis(3600) {
                                    ost_end_active = false;
                                    split(&mut splits, &settings, "ch1_ending_ost", false);
                                }

                                // Chapter 1 room change splits
                                if room.current != room.old {
                                    split(&mut splits, &settings, match (prev_room,cur_room) {
                                        ("PLACE_CONTACT","room_krisroom") => "ch1_contact",
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
                                        ("room_cc_prison_cells","room_cc_prison_cells") => match tempVar { //only split the 2nd time this room transition happens
                                            1 => {
                                                tempVar = 0;
                                                "ch1_escape_prison"
                                            },
                                            _ => {
                                                tempVar = 1;
                                                ""
                                            }
                                        },
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
                                        ("room_krisroom","room_ed") => { //setup for OST% ending split, which is delayed
                                            ost_end_active = true;
                                            ost_end_started = Instant::now();
                                            ""
                                        }
                                        _ => ""
                                    },false);
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                        "room_castle_darkdoor" if great_door_con.bytes_changed_from_to(&7.0,&21.0) => "ch1_castle_town_door",
                                        "room_man" if msc.bytes_changed_to(&601.0) && choice.current == 0.0 => "ch1_egg",
                                        "room_cc_joker" if match version {
                                            SP => jevil_dance1.current == 4.0 || jevil_dance2.current == 4.0,
                                            _ => mus.old.validate_utf8().unwrap_or_default().ends_with(r"mus\joker.ogg") && mus.current.matches(""),
                                        } => "ch1_beat_jevil",
                                        "room_cc_kingbattle" if king_pos.old == 680.0 && king_pos.current >= 1020.0 && king_pos.current <= 1030.0 => "ch1_king",

                                        "room_krisroom" if match version {
                                            SP => !SPEndingTriggered && plot.current == 251.0 && (final_textbox1.bytes_changed_from(&2.0) || final_textbox2.bytes_changed_from(&2.0) || filechoice.current > 2.0),
                                          _ => text_close_check(&text,r"* (You decided to go to bed.)/%",r"＊ (ねむることにした)/%")
                                        } => "ch1_ending",

                                        _ => ""
                                    },false);
                                }
                            }
                            // Chapter 2 logic
                            2 => {
                                let choice = choicer_ptr.update_value(&process);
                                timer::set_variable_float("Choice",choice.current);
                                let msc = msc_ptr.update_value(&process);
                                timer::set_variable_float("msc",msc.current);

                                let loaded_disk_bg = loaded_disk_bg_ptr.update_value(&process);
                                timer::set_variable_float("LoadedDisk BG",loaded_disk_bg.current);
                                let snowgrave = snowgrave_ptr.update_value(&process);
                                timer::set_variable_float("SnowGrave",snowgrave.current);

                                //variables only in versions with change_game
                                let text_all = match version {
                                    SP => unreachable!(),
                                    D109 | D110 | D115 => vec![text],
                                    D119 => vec![text,text_ptr2.update_value(&process),text_ptr3.update_value(&process)],
                                    Ch4_v102 | Ch5_v244 | Ch5_v247 => vec![text,text_ptr2.update_value(&process),text_ptr3.update_value(&process),text_ptr4.update_value(&process)],
                                };
                                for i in 1..text_all.len() {
                                    timer::set_variable(format!("Text ({})",i+1).as_str(),text_all[i].current.validate_utf8().unwrap_or_default());
                                }
                                /*let text2 = text_ptr2.update_value(&process);
                                timer::set_variable("Text (2)",text2.current.validate_utf8().unwrap_or_default());
                                let text3 = text_ptr3.update_value(&process);
                                timer::set_variable("Text (3)",text3.current.validate_utf8().unwrap_or_default());
                                let text4 = text_ptr4.update_value(&process);
                                timer::set_variable("Text (4)",text4.current.validate_utf8().unwrap_or_default());*/


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
                                        ("room_dw_cyber_musical_door","room_dw_city_intro") => match bagel_door {
                                          true => "ch2_cf_tz_skip",
                                          false => "ch2_cf_tz_warp"
                                        },
                                        ("room_dw_cyber_musical_door","room_dw_mansion_entrance") => match bagel_door {
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
                                        ("room_cc_fountain" | "room_dw_mansion_fountain","room_lw_computer_lab") => "ch2_giga_queen",
                                        ("room_torhouse","room_ed") => "ch2_ending_ost",
                                        _ => ""

                                    },false);
                                    if bagel_door { bagel_door = false; }
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                          "room_dw_cyber_queen_boxing" if msc.current == 1015.0 && mus.bytes_changed() && mus.current.validate_utf8().unwrap_or_default().ends_with(r"mus\cyber.ogg")
                                          => "ch2_arcade_text",

                                          "room_dw_cyber_music_final" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch2_dj_battle",
                                          "room_dw_city_big_2" if text_open_check_multipointer(&text_all,r"* (You got the FreezeRing.)/%", r"＊ (凍てつく指輪を　手に入れた)/%") => "ch2_freeze_ring",
                                          "room_dw_city_moss" if text_close_check_multipointer(&text_all,r"\S1* (You got the ThornRing.)/%", r"\S1＊ (いばらの指輪を　手に入れた)/%") => "ch2_thorn_ring",
                                          "room_dw_cyber_musical_door" | "room_dw_city_man" if msc.old == 1173.0 && msc.current >= 1173.0 && choice.current <= 0.0 => "ch2_egg",

                                          "room_dw_castle_west_cliff" if text_open_check_multipointer(&text_all,r"* (You have too many \cYWEAPONs\cW to&||take \cYPuppetScarf\c0.)/%", r"＊ (\cYぶき\cWが多すぎて&　 \cYパペットマフラー\c0を&　 持てない)/%")
                                          => "ch2_thorny_ending",

                                          "room_torhouse" if text_open_check_multipointer(&text_all,r"* (... Susie fell asleep.)/%", r"＊ (…スージィは　ねおちした)/%") => "ch2_ending_ac",
                                          "room_torhouse" if text_open_check_multipointer(&text_all,r"\E1* ... they're already&||asleep.../%", r"\E1＊ …ふたりとも　もう&　 ねむってしまったのね。/%") => "ch2_ending_il",

                                          "room_dw_cyber_musical_door" if text_close_check_multipointer(&text_all,r"* (You were crushed under the&||weight of 400 bagels and&||defeated instantly...)/%", r"＊ (ベーグル400コの　重みに耐えきれ^1ず&　たちまち　力つきた…)/%")
                                          => { bagel_door = true; "" }

                                          _ => ""
                                    },false);
                                }

                            }
                            // Chapter 3 logic
                            3 => {
                                let plot = plot_ptr.update_value(&process);
                                timer::set_variable_float("Plot",plot.current);

                                let knight_result = knight_result_ptr.update_value(&process);
                                timer::set_variable_float("Knight Result",knight_result.current);
                                let egg_timer = egg_timer_ptr.update_value(&process);
                                timer::set_variable_float("Egg Timer",egg_timer.current);
                                let mantle_outro = mantle_outro_ptr.update_value(&process);
                                timer::set_variable_float("Mantle Outro",mantle_outro.current);

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
                                          "room_dw_snow_zone" if knight_result.bytes_changed_from(&0.0) => match knight_result.current {
                                              1.0 => "ch3_knight_win",
                                              2.0 => "ch3_knight_death",
                                              _ => ""
                                          },
                                          "room_dw_snow_zone" if fighting.bytes_changed_from_to(&0.0,&1.0) => "ch3_enter_knight",

                                          "room_town_shelter" if mus.current.matches("") && mus.old.validate_utf8().unwrap_or_default().ends_with(r"mus\night_ambience.ogg")
                                          => "ch3_ending",

                                          "room_dw_man" if egg_timer.old <= 1.0 && egg_timer.current > 1.0 => "ch3_egg",
                                          "room_shadowmantle" if mantle_outro.bytes_changed_to(&0.125) => "ch3_end_mantle",
                                          _ => ""
                                    },false);
                                }
                            }
                            // Chapter 4 logic
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
                                        ("room_dw_church_holywatercooler","room_dw_church_intro_gerson") => "ch4_miss_mizzle",
                                        ("room_dw_church_fountain","room_lw_church_entrance") => "ch4_first_sanctuary",
                                        ("room_dw_churchb_darkclimb_scene","room_dw_churchb_darkclimb") => "ch4_fall_down",
                                        ("room_dw_churchb_nongerson","room_dw_churchb_nongerson_post") => "ch4_sound_of_justice",
                                        ("room_dw_churchb_fountain","room_lw_church_entrance") => "ch4_second_sanctuary",
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
                                        "room_dw_castle_tv_zone_battle" if fighting.current == 1.0 && mike_action.bytes_changed_to(&18.0) => "ch4_beat_mike",
                                        "room_dw_church_jackenstein" if fighting.old == 1.0 && fighting.current == 0.0 => "ch4_jackenstein",
                                        "room_dw_church_arena" if fighting.current == 1.0 && susie_sprite.changed() => match (susie_sprite.old,susie_sprite.current) {
                                            (3128|3129|3130,_) => { tempVar = 1; ""}
                                            (1535,1553) | (1536,1554) if tempVar == 1 => "ch4_hammer_of_justice",
                                            _ => ""
                                        }
                                        "room_dw_churchb_man" if text_close_check(&text,r"* (An Egg was picked up from a&||nearby easel.)/%", "＊ (近くのイーゼルから\n　 タマゴを　拾いあげた)/%")
                                        => "ch4_egg",
                                        "room_dw_churchc_prophecies" if text_open_check(&text,r"* (\cYPrincessRBN\cW was added to your&||\cYARMORs\cW.)/%", r"＊ (\cYプリティリボン\cWが&　 \cYぼうぐ\cWに　加わった)/%")
                                        => "ch4_princess_ribbon",
                                        "room_dw_churchc_titanclimb2_post" if fighting.old == 0.0 && fighting.current == 1.0 => "ch4_start_titan_fight",
                                        "room_torhouse" if plot.current == 310.0 && (player_x.bytes_changed() || player_y.bytes_changed()) && player_x.current < 160.0 && player_y.current < 80.0
                                        => "ch4_ending_il",
                                        _ => ""
                                    },false);
                                }
                            }
                            // Chapter 5 logic
                            5 => {
                                let choice = choicer_ptr.update_value(&process);
                                timer::set_variable_float("Choice",choice.current);
                                let pink_coins = pink_coins_ptr.update_value(&process);
                                timer::set_variable_float("Pink Coins",pink_coins.current);
                                let crt_start = crt_start_ptr.update_value(&process);
                                timer::set_variable_int("CRT Start",crt_start.current);

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
                                        ("room_shop","room_dw_cliff_shop") if tempVar == 1 => "ch5_pink_shop",
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
                                        ("room_cc_fountain","room_flowershop_2f") => { tempVar = 1; "ch5_fountain2" }, //Completion data can only happen after this and in the same session
                                        (_,"PLACE_MENU") => {tempVar = 0;""} //just in case, make sure to disable the completion data split's eligibility if you somehow go back to the menu and load a different save (which shouldn't really be possible on PC since you're in the Light World)
                                        (_,"room_schooldoor") if tempVar == 1 => "ch5_ending_completion_data",
                                        ("room_schooldoor","room_ed") => "ch5_ending_src",
                                        _ => ""
                                    },false);
                                } else {
                                    split(&mut splits,&settings,match cur_room {
                                        "room_dw_garden_aqua" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch5_aqua_end",
                                        "room_man" if choice.bytes_changed_from_to(&-1.0,&0.0) => "ch5_egg",
                                        "room_dw_cliff_sethaqua_battle" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch5_beat_seth_aqua",
                                        "room_shop" if settings.ch5_pink_shop && pink_coins.decreased() => { tempVar = 1; "" },
                                        "room_dw_pink_encounter" if fighting.bytes_changed_from_to(&1.0,&0.0) => "ch5_pink_end",
                                        "room_dw_fcastle_flowery" if fighting.bytes_changed_from_to(&0.0,&1.0) => "ch5_start_flowery",
                                        "room_beach" if crt_start.changed_to(&16777215) => "ch5_sideb",
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
