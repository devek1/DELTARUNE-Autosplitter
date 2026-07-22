use asr::{Process};

use self::Title::*;
use crate::{EngineVersion, helpers::*, title_tracker};

//Unique titles. The description counts as a part of the title that can make it unique, but the level number doesn't
#[derive(Eq,PartialEq,Hash,Clone)]
pub enum Title {
    //Kris Ch1
    Human,
    Leader,
    BedInspector,
    //Kris Ch2
    Tactician1,
    MossFinder,
    LeaderSideB,
    //Kris Ch3
    Tactician2,
    Director,
    MossMystery,
    Blazer,
    IceBlazer,
    Sheath,
    Enjoying,
    //Kris Ch4
    DarkHeroBlade, //shared with Susie, but description is the same
    MossMost,
    DarkBead,
    //Kris Ch5
    BlueRose,
    DarkRose,
    PinkRose,
    ShatteredRose,
    LastMoss,
    Walkerstar,
    LV1, //empty title used in egg room

    //Susie Ch1
    MeanGirl,
    DarkKnight,
    //Susie Ch2
    HealingMaster,
    MossEnjoyer,
    //Susie Ch3
    DarkActorS,
    //Susie Ch4
    AxeOfJustice,
    //Susie Ch5
    ViolentViolet,
    DarkVineS,

    //Ralsei Ch1
    LonelyPrince,
    PricklyPrince,
    FluffyPrince,
    //Ralsei Ch2
    DarkPrince1, //"has friends now"
    HugPrince,
    PosePrince,
    RudePrince,
    BlankPrince,
    //Ralsei Ch3
    DarkPrince2, //"has friends, but..."
    DarkActorR,
    Horse,
    ExHorse,
    DarkPrince3, //"has friends"
    //Ralsei Ch4
    DarkHeroRecords,
    StoolBoy,
    //Ralsei Ch5
    Artemisia,
    DarkVineR,

    //Noelle Ch2
    Snowcaster,
    Frostmancer,
    IceTrancer,
    MossNeutral,

    //Flowery Ch5
    Roommate,
}

pub fn titles_check(proc : &Process, version : EngineVersion, global : &GlobalReader, room : &str) {
    let chapter = global.num(proc,"chapter");
    let plot = global.num(proc,"plot");
    let flag = |index| global.arrNum(proc,"flag[0]",index);
    let sword_progress = flag(1055);
    for charslot in 0..=2 {
        let title = match global.arrNum(proc,"char[0]",charslot) {
            1.0 => match chapter { //Kris
                1.0 => match () {
                    _ if global.arrNum(proc,"flag[0]",252) == 1.0 => BedInspector,
                    _ if plot >= 30.0 => Leader,
                    _ => Human
                }
                2.0 => match () {
                    _ if flag(915) > 0.0 && flag(916) == 0.0 => LeaderSideB,
                    _ if flag(920) == 1.0 => MossFinder,
                    _ if flag(252) == 1.0 => BedInspector,
                    _ if plot >= 60.0 => Tactician1,
                    _ => Leader
                }
                3.0 => match () {
                    _ if flag(930) == 1.0 => Enjoying, //the game technically checks for the Egg key item but these checks should always align within Ch3
                    _ if sword_progress == 6.0 => Sheath,
                    _ if sword_progress == 3.0 => IceBlazer,
                    _ if sword_progress == 1.0 => Blazer,
                    _ if plot >= 250.0 => Director,
                    _ => Tactician2
                }
                4.0 => match () {
                    _ if flag(915) >= 7.0 && flag(916) == 0.0 => DarkBead,
                    _ if flag(106) == 1.0 && flag(922) == 1.0 && flag(1078) == 1.0 && flag(1592) == 1.0 => MossMost,
                    _ => DarkHeroBlade
                }
                5.0 => match () {
                    _ if !get_obj_inst(proc, version, "obj_room_man").is_null() => LV1,
                    _ if proc.read::<i64>(global.ptr(proc,"flag[0]").add(arr_pos(1851))).unwrap_or_default() & 1 == 1 => Walkerstar,
                    _ if plot >= 398.0 && flag(106) == 1.0 && flag(922) == 1.0 && flag(1078) == 1.0 && flag(1592) == 1.0 => LastMoss,
                    _ if flag(1846) >= 2.0 => PinkRose,
                    _ if plot >= 440.0 => DarkRose,
                    _ if flag(1743) == 1.0 => ShatteredRose,
                    _ => BlueRose
                }
                _ => {continue; }
            }
            2.0 => match chapter { //Susie
                1.0 => match () {
                    _ if plot >= 154.0 => DarkKnight,
                    _ => MeanGirl
                }
                2.0 => match () {
                    _ if flag(922) == 1.0 => MossEnjoyer,
                    _ if plot >= 95.0 => HealingMaster,
                    _ => DarkKnight
                }
                3.0 => match () {
                    _ if plot >= 250.0 => DarkActorS,
                    _ => DarkKnight,
                }
                4.0 => match () {
                    _ if flag(852) > 0.0 => AxeOfJustice,
                    _ => DarkHeroBlade
                }
                5.0 => match () {
                    _ if plot >= 440.0 => DarkVineS,
                    _ => ViolentViolet
                }
                _ => {continue;}
            }
            3.0 => match chapter { //Ralsei (and briefly Flowery)
                1.0 => match global.arrNum(proc,"charweapon[0]",3) {
                    10.0 => FluffyPrince,
                    9.0 => PricklyPrince,
                    _ => LonelyPrince
                }
                2.0 => match flag(325) {
                    1.0 => HugPrince,
                    2.0 => PosePrince,
                    3.0 => RudePrince,
                    4.0 => BlankPrince,
                    _ => DarkPrince1
                }
                3.0 => match () {
                    _ if plot >= 250.0 => DarkActorR,
                    _ => DarkPrince2
                }
                4.0 => match () {
                    _ if plot >= 145.0 && room == "room_dw_church_bookcase" => StoolBoy,
                    _ => DarkHeroRecords
                }
                5.0 => match () {
                    _ if get_all_instances(proc,version,get_obj("obj_caterpillar_generic")).iter().any(|inst| get_inst_str::<32>(proc,version,*inst,"name").matches("flowery")) => Roommate,
                    _ if plot >= 440.0 => DarkVineR,
                    _ => Artemisia
                }
                _ => {continue;}

            }
            4.0 => match chapter { //Noelle
                2.0 => match () {
                    _ if flag(921) == 1.0 && (flag(916) != 0.0  || flag(915) <= 0.0) => MossNeutral,
                    _ if global.arrNum(proc, "charweapon[0]", 4) == 13.0 => IceTrancer,
                    _ if flag(925) > 0.0 => Frostmancer,
                    _ => Snowcaster
                }
                _ => {continue;}
            }
            _ => {continue;}
        };
        title_tracker.insert(title);
    }
}