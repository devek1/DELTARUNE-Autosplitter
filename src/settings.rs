use asr::{settings::{Gui,gui::Title}};

#[derive(Gui)]
pub enum PauseTiming {
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
    /// Start the timer, unpause IGT if already running
    AutoStartAndUnpause,
    /// Start the timer, if it was not already running
    AutoStart,
    /// Do nothing
    Off
}

#[derive(Gui)]
pub enum Ch5StartOnPrev {
    ///No
    #[default]
    No,
    ///Yes
    Yes,
    ///Exclusively
    Exclusively
}

#[derive(Gui)]
pub struct Settings {

    ///General Settings
    gen_title : Title,
    ///On creating a new file:
    pub(crate) auto_start : AutoStart,
    ///Pause the timer between chapters
    #[default = true]
    pub(crate) ac_pause_timer : bool,
    ///Timing of the pauses
    pub(crate) chapter_pause_timing : PauseTiming,
    ///Also unpause from loading a savefile
    pub(crate) ac_unpause_loadsave : bool,
    ///Item Tracker
    pub(crate) item_tracking : bool,
    ///Function with unrecognized data.win files (for brand-new patches, and mod speedruns)
    pub(crate) allow_unsupported_version : bool,


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
    pub(crate) ch5_start_on_prev : Ch5StartOnPrev,
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
    ///Obtain Seed Packets
    ch5_seed_packets : bool,
    ///Ending (SRC rules)
    ch5_ending_src : bool,
    ///Ending (completion data timing) [NOTE: no category uses this yet]
    ch5_ending_completion_data : bool,
    ///Obtain Bread
    ch5_bread : bool,
    ///Complete Side B - speedrun timing
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