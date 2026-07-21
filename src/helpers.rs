use std::collections::{HashMap, HashSet};
use asr::deep_pointer::DeepPointer;
use asr::settings::Map;
use asr::watcher::{Pair, Watcher};
use asr::{Address, PointerSize, Process, string::ArrayCString, timer};
use crate::item_tracking::Item;
use crate::{AC_Pauses, IL_Pauses, OST_LateCh2_Pauses, OST_Pauses, ps32};
use crate::{EngineVersion::{self,*}, settings::{*}, ps64};

pub async fn delay_split_frames(name : &str, frames : u32) -> &str {
    for _ in 0..(2*frames) {
        asr::future::next_tick().await;
    }
    name
}

pub fn verPs(version : EngineVersion) -> PointerSize {
    match version { 
        GMS2_v2_2_0 | GMS2_2022_1 | GMS2_2022_2 => ps32,
        _ => ps64
    }
}

pub struct VarFinder {
    pub numAddr : Address,
    pub arrAddr : Address,
    version : EngineVersion,
    ps : PointerSize,
    jmp : usize,
}

impl VarFinder {
    pub fn try_new(process: &Process, version : EngineVersion, instAddr: Address) -> Result<Self,asr::Error> {
        let ps= verPs(version);
        let midAddr = process.read_pointer(instAddr.add(match version { GMS2_v2_2_0=>0x60 ,GMS2_2022_1|GMS2_2022_2 => 0x24, _ => 0x48}), ps)?;
        Ok(VarFinder {
            numAddr: midAddr.add(0x8),
            arrAddr: process.read_pointer(midAddr.add(0x10), ps)?.add(match version {GMS2_v2_2_0=>0x4,_=>0x0}),
            version,
            ps,
            jmp: match ps {ps64=>0x10,_=>0xC}
        })
    }
    pub fn try_new_demoGlobal(process: &Process, version : EngineVersion, globalAddr: Address) -> Result<Self,asr::Error> {
        let arrAddr = process.read_pointer(globalAddr.add(0x30), ps32)?;
        Ok(VarFinder {
            numAddr: globalAddr.add(0x28),
            arrAddr: arrAddr,
            version,
            ps : ps32,
            jmp: 0xC
        })
    }

    //Find a pointer to a specific variable, this can be used to find initial pointers for variables of complex types
    pub fn getVarPtr(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> Address {
        for offset in (0..(process.read::<u32>(self.numAddr).unwrap_or_default() as u64)*self.jmp as u64).step_by(self.jmp) {
            let stringID = process.read::<u32>(self.arrAddr + offset + match self.version {GMS2_v2_2_0=>-0x4,GMS2_2022_1|GMS2_2022_2=>0x4,_=>0x8}).unwrap_or_default();
            /*if !matches!(self.version,GMS2_v2_2_0) && stringID < 100000 {
                continue;
            }*/
            if stringsList.get(&stringID).unwrap_or(&"".to_owned()) == name.trim_end_matches("[0]") {
                if name.ends_with("[0]") {
                    match self.version { //there's no direct support for reading a pointer from a pointer path, so we need to specifically read different types (64-bit address vs 32-bit address) depending on 64-bit vs 32-bit
                        GMS2_v2_2_0 => { return Address::from(process.read_pointer_path::<u32>(self.arrAddr, self.ps,&[offset,0x0,0x24,0xC]).unwrap_or_default()); }
                        GMS2_2022_1|GMS2_2022_2 => { return Address::from(process.read_pointer_path::<u32>(self.arrAddr, self.ps,&[offset,0x0,0x64]).unwrap_or_default()); }
                        _ => { return Address::from(process.read_pointer_path::<u64>(self.arrAddr, self.ps,&[offset,0x0,0x90]).unwrap_or_default()); }
                    }
                }
                return process.read_pointer(self.arrAddr + offset, self.ps).unwrap_or_default();
            }
        }
        Address::NULL
    }

    //Immediately read simple value from a variable, this is used to read numeric variable values from instances of objects without needing
    //not strictly limited to numbers, but most other types require further pointers
    pub fn readNum<T: bytemuck::Pod + Default>(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> T {
        let ptr = self.getVarPtr(process, stringsList, name);
        match ptr {
            Address::NULL => T::default(),
            _ => process.read::<T>(ptr).unwrap_or_default()
        }
    }

    //Strings take additional layers of pointers
    pub fn readStr<const len : usize>(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> ArrayCString<len> {
        let ptr = self.getVarPtr(process, stringsList, name);
        match ptr {
            Address::NULL => ArrayCString::<len>::default(),
            _ => process.read_pointer_path::<ArrayCString<len>>(ptr,self.ps,&[0x0,0x0,0x0]).unwrap_or_default()
        }
    }
}

pub struct LongTermVarReader {
    pub finder : VarFinder,
    pub(crate) cache : HashMap<String,Address>
}

impl LongTermVarReader {
    pub fn try_new(process: &Process, version : EngineVersion, instAddr: Address) -> Result<Self,asr::Error> {
        Ok(LongTermVarReader {
            finder : VarFinder::try_new(process,version,instAddr)?,
            cache : HashMap::<String,Address>::new()
        })
    }
    pub fn try_new_demo_global(process: &Process, version: EngineVersion, globalAddr: Address) -> Result<Self,asr::Error> {
        Ok(LongTermVarReader {
            finder : VarFinder::try_new_demoGlobal(process,version,globalAddr)?,
            cache : HashMap::<String,Address>::new()
        })
    }

    pub fn ptr(&mut self, process : &Process, stringsList : &HashMap<u32,String>, name : &str) -> Address {
        if self.cache.contains_key(name) {
            {
                #![cfg(debug_assertions)]
                timer::set_variable(&format!("{} from",name), "cache");
            }
            return *self.cache.get(name).unwrap_or(&Address::NULL); //technically safety should already be ensured but we want to absolutely avoid any risk of panics
        }
        let foundPtr = self.finder.getVarPtr(process, stringsList, name);
        if !foundPtr.is_null() { self.cache.insert(name.to_owned(),foundPtr); }
        {
            #![cfg(debug_assertions)]
            timer::set_variable(&format!("{} from",name), "process");
        }
        foundPtr
    }

    pub fn num(&mut self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> f64 {
        let ptr = self.ptr(process, stringsList, name);
        match ptr {
            Address::NULL => 0.0,
            _ => process.read::<f64>(ptr).unwrap_or_default()
        }
    }

    //Strings take additional layers of pointers
    pub fn str<const len : usize>(&mut self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> ArrayCString<len> {
        let ptr = self.ptr(process, stringsList, name);
        match ptr {
            Address::NULL => ArrayCString::<len>::default(),
            _ => process.read_pointer_path::<ArrayCString<len>>(ptr,self.finder.ps,&[0x0,0x0,0x0]).unwrap_or_default()
        }
    }
}

fn get_first_instance(process : &Process, version : EngineVersion , obj : Address) -> Address {
    let ps = verPs(version);
    let Ok(obj_prop) = process.read_pointer(obj.add(match ps {ps64=>0x18,_=>0xC}),ps) else {
        return Address::NULL;
    };
    //NOTE: 
    let instCount = process.read::<u32>(obj_prop.add(match version {GMS2_v2_2_0=>0xCC,GMS2_2022_1|GMS2_2022_2=>0x40,_=>0x78})).unwrap_or_default();
    {
        #![cfg(debug_assertions)]
        timer::set_variable("instance count",&format!("{}",instCount));
    }
    if instCount == 0 { return Address::NULL; }
    let node = process.read_pointer(obj_prop.add(match version {GMS2_v2_2_0=>0xC4,GMS2_2022_1|GMS2_2022_2=>0x38,_=>0x68}),ps).unwrap_or_default();
    {
        #![cfg(debug_assertions)]
        timer::set_variable("last found first node",&format!("{}",node));
    }
    process.read_pointer(node.add(match ps {ps64=>0x10,_=>0x8}),ps).unwrap_or(Address::NULL)
}

fn get_all_instances(process : &Process, version : EngineVersion, obj : Address) -> Vec<Address> {
    let ps = verPs(version);
    let mut vec = Vec::<Address>::new();
    let Ok(obj_prop) = process.read_pointer(obj.add(match version {GMS2_2022_1|GMS2_2022_2=>0xC, _=>0x18}),ps) else {
        return vec;
    };
    let instCount = process.read::<u32>(obj_prop.add(match version {GMS2_v2_2_0=>0xCC,GMS2_2022_1|GMS2_2022_2=>0x40,_=>0x78})).unwrap_or_default();
    {
        #![cfg(debug_assertions)]
        timer::set_variable("instance count",&format!("{}",instCount));
    }
    if instCount == 0 { return vec; }
    let mut node = process.read_pointer(obj_prop.add(match version {GMS2_v2_2_0=>0xC4,GMS2_2022_1|GMS2_2022_2=>0x38,_=>0x68}),ps).unwrap_or_default();
    for i in 0..instCount {
        vec.push(process.read_pointer(node.add(match version {GMS2_v2_2_0|GMS2_2022_1|GMS2_2022_2=>0x8,_=>0x10}),ps).unwrap_or_default());
        if i<instCount-1 { node = process.read_pointer(node,ps).unwrap_or_default(); }
    }
    vec
}

pub fn get_obj(map : &HashMap<String,Address>, name : &str) -> Address {
    *map.get(&String::from(name)).unwrap_or(&Address::NULL)
}

pub fn get_inst_var<T: bytemuck::Pod + Default>(process : &Process, version : EngineVersion, stringsList : &HashMap<u32,String>, inst : Address, name : &str) -> T {
    match inst {
        Address::NULL => T::default(),
        inst => match VarFinder::try_new(&process,version,inst) {
            Ok(finder) => finder.readNum::<T>(&process, stringsList, name),
            Err(_) => T::default()
        }
    }
}

pub fn get_obj_inst(process : &Process, version : EngineVersion, objMap : &HashMap<String,Address>, _obj : &str) -> Address {
    match get_obj(objMap,_obj) {
        Address::NULL => Address::NULL,
        obj => match get_first_instance(&process,version,obj) {
            Address::NULL => Address::NULL,
            inst => inst
        }
    }
}

pub fn get_obj_var<T: bytemuck::Pod + Default>(process : &Process, version : EngineVersion, objMap : &HashMap<String,Address>, stringsList : &HashMap<u32,String>, _obj : &str, name : &str) -> T {
    get_inst_var::<T>(&process,version,stringsList,get_obj_inst(process,version,objMap,_obj),name)
}

pub fn get_obj_str<const len : usize>(process : &Process, version : EngineVersion, objMap : &HashMap<String,Address>, stringsList : &HashMap<u32,String>, _obj : &str, name : &str) -> ArrayCString<len> {
    let inst = get_obj_inst(process,version,objMap,_obj);
    let Ok(finder) = VarFinder::try_new(process, version, inst) else {
        return ArrayCString::<len>::default();
    };
    let ptr = finder.getVarPtr(process, stringsList, name);
    process.read_pointer_path(ptr, verPs(version), &[0x0,0x0,0x0]).unwrap_or_default()
}

pub fn check_text(process : &Process, version : EngineVersion, stringsList: &HashMap<u32, String>, writer : Address, en : &str, jp : &str) -> bool {
    let instVec = get_all_instances(process, version, writer);
    if instVec.len() == 0 { return false; }
    for instance in instVec {
        let Ok(finder) = VarFinder::try_new(&process,version,instance) else { continue; };
        let txt = finder.readStr::<128>(&process,&stringsList,"mystring");
        if txt.matches(en) || txt.matches(jp) {
            return true;
        }
    }
    false
}

//values in an array are located at pointer offsets 0x0,0x90,(8*index) from the array's own pointer. This function takes the address array's pointer and gets the address to index 0's pointer, which can then be used to more performantly find any element in the array
/*fn get_array_element0(proc : &Process, ps : PointerSize, base : &Address) -> Address {
    match base {
        &Address::NULL => Address::NULL,
        _ => Address::from(proc.read_pointer_path::<u64>(*base,ps,&[0x0,0x90]).unwrap_or_default())
    }
}*/

pub fn read_setting(key : &str) -> bool {
    match Map::load().get(key) {
        Some(x) => x.get_bool().unwrap_or(false),
        None => false
    }
}

pub const fn arr_pos(i : u64) -> u64 { i * 0x10 }

pub fn start(auto_start : &AutoStart, splits : &mut HashSet<String>, item_tracker : &mut HashSet<Item>) {
    match auto_start {
        AutoStart::AutoReset => {
            splits.clear();
            item_tracker.clear();
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

//also does some handling for IGT pausing and unpausing to simplify code elsewhere
pub fn split(splits : &mut HashSet<String>, settings : &Settings, name : &str, already_checked : bool) {
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
    asr::print_message(&format!("Split triggered: {}",name));
    timer::split();
}

pub struct PathTrack<T: Clone + bytemuck::Pod> {
    pointer : Option<DeepPointer<16>>,
    watcher : Watcher<T>,
}
impl<T: Clone + bytemuck::Pod> PathTrack<T> {
    pub fn new(module_base : Address, pointer_size: PointerSize, offsets : &[u64]) -> PathTrack<T> {
        PathTrack {
            watcher: Watcher::<T>::new(),
            pointer: match offsets {
                &[0] => None,
                x => Some(DeepPointer::new(module_base, pointer_size, x)),
            }
        }
    }
    pub fn update_value(&mut self, process: &Process) -> &Pair<T> {
        if self.pointer.is_none() {
            return self.watcher.update_infallible(T::zeroed())
        }
        let value = self.pointer.unwrap_or_default().deref(&process).unwrap_or_else(|_e| T::zeroed());
        self.watcher.update_infallible(value)
    }
}

pub fn global_setup(process : &Process, DELTARUNE : Address, version : EngineVersion, ps : PointerSize) -> Result<LongTermVarReader,asr::Error> {
    let globalAddr = process.read_pointer(DELTARUNE.add(match version {
                GMS2_v2_2_0 => 0x49C3E0, //0x48E5DC,
                GMS2_2022_1 => 0x6FCF38,
                GMS2_2022_2 => 0x6FE860,
                GM_LTS2022_0_3_99 => 0x6A1CA8,
                GM_LTS2022_0_3_104 => 0x6A9CA8,
            }),ps)?;
    match version {
        //GMS2_v2_2_0 => LongTermVarReader::try_new_SP_global(&process,globalAddr),
        GMS2_2022_1|GMS2_2022_2 => LongTermVarReader::try_new_demo_global(&process,version,globalAddr),
        _ => LongTermVarReader::try_new(&process,version,globalAddr),
    }
}