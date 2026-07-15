#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

use asr::deep_pointer::DeepPointer;
use asr::settings::Map;
use asr::watcher::{Pair, Watcher};
use asr::{Address, PointerSize, Process, string::ArrayCString, timer};
use crate::item_tracking::Item;
use crate::{AC_Pauses, IL_Pauses, OST_LateCh2_Pauses, OST_Pauses};
use crate::{EngineVersion::{self,*}, settings::{*}, ps64};



pub struct VarFinder {
    pub numAddr : Address,
    pub arrAddr : Address,
    ps : PointerSize
}

impl VarFinder {
    pub fn try_new(process: &Process, ps: PointerSize, instAddr: Address) -> Option<VarFinder> {
        let Ok(midAddr) = process.read_pointer(instAddr.add(match ps { ps64 => 0x48, _ => 0x24}), ps) else {
            return None;
        };
        Some(VarFinder {
            numAddr: midAddr.add(0x8),
            arrAddr: process.read_pointer(midAddr.add(0x10), ps).unwrap(),
            ps
        })
    }
    pub fn try_new_alt32(process: &Process, ps: PointerSize, globalAddr: Address) -> Option<VarFinder> {
        let Ok(arrAddr) = process.read_pointer(globalAddr.add(0x30), ps) else {
            return None;
        };
        Some(VarFinder {
            numAddr: globalAddr.add(0x28),
            arrAddr: arrAddr,
            ps
        })
    }
    pub fn new(process: &Process, ps: PointerSize, instAddr: Address) -> VarFinder {
        let midAddr = process.read_pointer(instAddr.add(match ps { ps64 => 0x48, _ => 0x24}), ps).unwrap();
        VarFinder {
            numAddr: midAddr.add(0x8),
            arrAddr: process.read_pointer(midAddr.add(0x10), ps).unwrap(),
            ps
        }
    }

    //Find a pointer to a specific variable, this can be used to find initial pointers for variables of complex types
    pub fn findVarPtr(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> Address {
        for i in 0..(process.read::<u32>(self.numAddr).unwrap_or_default() as u64) {
            let offset = match self.ps {ps64=>0x10,_=>0xC} * i;
            let stringID = process.read::<u32>(self.arrAddr + offset + match self.ps {ps64=>0x8,_=>0x4}).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            if stringsList.get(&(stringID - 100000)).unwrap_or(&String::from("")) == name {
                return process.read_pointer(self.arrAddr + offset, self.ps).unwrap_or_default();
            }
        }
        Address::NULL
    }

    //Populate a HashMap with pointers for variables from provided list
    pub fn populatePtrMap(&self, process: &Process, stringsList: &HashMap<u32, String>, pointerMap : &mut HashMap<&'static str, Address>, names: &[&'static str]) {
        for i in 0..(process.read::<u32>(self.numAddr).unwrap_or_default() as u64) {
            let offset = match self.ps {ps64=>0x10,_=>0xC} * i;
            let stringID = process.read::<u32>(self.arrAddr + offset + match self.ps {ps64=>0x8,_=>0x4}).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            let Some(string) = stringsList.get(&(stringID - 100000)) else {
                continue;
            };
            for name in names {
                if string.as_str() == name.trim_end_matches("[0]") {
                    if name.ends_with("[0]") {
                        pointerMap.entry(name).or_insert_with(|| Address::from(process.read_pointer_path::<u64>(self.arrAddr, self.ps,&[offset,0x0,0x90]).unwrap_or_default()));
                    } else {
                        pointerMap.entry(name).or_insert_with(|| process.read_pointer(self.arrAddr + offset, self.ps).unwrap_or_default());
                    }
                    asr::print_message(format!("global.{} found at {}",name,pointerMap.get(name).unwrap_or(&Address::NULL)).as_str());
                }
            }
            /*if names.contains(&name.as_str())  {
                pointerMap.insert(name.clone().as_str(),process.read_pointer(self.arrAddr + offset, self.ps).unwrap_or_default());
            }*/
        }
    }

    //Immediately read simple value from a variable, this is used to read numeric variable values from instances of objects without needing
    //not strictly limited to numbers, but most other types require further pointers
    pub fn readNum<T: bytemuck::Pod + Default>(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> T {
        for i in 0..(process.read::<u32>(self.numAddr).unwrap_or_default() as u64) {
            let offset = match self.ps {ps64=>0x10,_=>0xC} * i;
            let stringID = process.read::<u32>(self.arrAddr + offset + match self.ps {ps64=>0x8,_=>0x4}).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            if stringsList.get(&(stringID - 100000)).unwrap_or(&String::from("")) == name {
                //timer::set_variable("Address read from",process.read_pointer(self.arrAddr.add(offset as u64),self.ps).unwrap_or_default().);
                return process.read_pointer_path::<T>(self.arrAddr, self.ps, &[offset,0x0]).unwrap();
            }
        }
        T::default()
    }

    //Strings take additional layers of pointers
    pub fn readStr<const len : usize>(&self, process: &Process, stringsList: &HashMap<u32, String>, name: &str) -> ArrayCString<len> {
        for i in 0..(process.read::<u32>(self.numAddr).unwrap_or_default() as u64) {
            let offset = match self.ps {ps64=>0x10,_=>0xC} * i;
            let stringID = process.read::<u32>(self.arrAddr + offset + match self.ps {ps64=>0x8,_=>0x4}).unwrap_or_default();
            if stringID < 100000 {
                continue;
            }
            if stringsList.get(&(stringID - 100000)).unwrap_or(&String::from("")) == name {
                let res = process.read_pointer_path::<ArrayCString<len>>(self.arrAddr, self.ps, &[offset,0x0,0x0,0x0]).unwrap_or_default();
                return res;//.validate_utf8().unwrap_or_default().to_string();
                //return process.read_pointer_path::<ArrayCString<len>>(self.arrAddr, self.ps, &[offset as u64, 0x0]).unwrap_or_default().validate_utf8().unwrap_or_default().to_string();
            }
        }
        ArrayCString::<len>::default()
    }
}

fn get_first_instance(process : &Process, ps : PointerSize, obj : Address) -> Address {
    let Ok(obj_prop) = process.read_pointer(obj.add(match ps {ps64=>0x18,_=>0xC}),ps) else {
        return Address::NULL;
    };
    let instCount = process.read::<u32>(obj_prop.add(0x78)).unwrap_or_default();
    if instCount == 0 { return Address::NULL; }
    let node = process.read_pointer(obj_prop.add(0x68),ps).unwrap_or_default();
    timer::set_variable("last found first node",format!("{}",node).as_str());
    process.read_pointer(node.add(0x10),ps).unwrap_or(Address::NULL)
}

fn get_all_instances(process : &Process, ps : PointerSize, obj : Address) -> Vec<Address> {
    let mut vec = Vec::<Address>::new();
    let Ok(obj_prop) = process.read_pointer(obj.add(match ps {ps64=>0x18,_=>0xC}),ps) else {
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

pub fn get_obj(map : &HashMap<String,Address>, name : &str) -> Address {
    *map.get(&String::from(name)).unwrap_or(&Address::NULL)
}

pub fn get_inst_var<T: bytemuck::Pod + Default>(process : &Process, ps : PointerSize, stringsList : &HashMap<u32,String>, inst : Address, name : &str) -> T {
    match inst {
        Address::NULL => T::default(),
        inst => match VarFinder::try_new(&process,ps,inst) {
            Some(finder) => finder.readNum::<T>(&process, stringsList, name),
            None => T::default()
        }
    }
}

pub fn get_obj_inst(process : &Process, ps : PointerSize, objMap : &HashMap<String,Address>, _obj : &str) -> Address {
    match get_obj(objMap,_obj) {
        Address::NULL => Address::NULL,
        obj => match get_first_instance(&process,ps,obj) {
            Address::NULL => Address::NULL,
            inst => inst
        }
    }
}

pub fn get_obj_var<T: bytemuck::Pod + Default>(process : &Process, ps : PointerSize, objMap : &HashMap<String,Address>, stringsList : &HashMap<u32,String>, _obj : &str, name : &str) -> T {
    get_inst_var(&process,ps,stringsList,get_obj_inst(process,ps,objMap,_obj),name)
}

pub fn chapter1ify(version : &EngineVersion, objName : &str) -> String {
    match version {
        LTS2022_1 | LTS2022_2 => objName.to_owned() + "_ch1",
        _ => objName.to_owned()
    }
}

pub fn check_text(process : &Process, ps : PointerSize, stringsList: &HashMap<u32, String>, writer : Address, en : &str, jp : &str) -> bool {
    let instVec = get_all_instances(process, ps, writer);
    if instVec.len() == 0 { return false; }
    for instance in instVec {
        let txt = VarFinder::new(&process,ps,instance).readStr::<128>(&process,&stringsList,"mystring");
        if txt.matches(en) || txt.matches(jp) {
            return true;
        }
    }
    false
}

//if a global was previously missed when populating the global pointers map, try searching for it again along with any other missing ones. Otherwise simply read from the pointer map
pub fn global_ptr(globalPtrs : &HashMap<&'static str,Address>, name : &str ) -> Address {
    *globalPtrs.get(name).unwrap_or(&Address::NULL)
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

pub const fn arr_index(i : u64) -> u64 { i * 0x10 }

pub fn check_val_in_arr(process : &Process, ps : PointerSize, addr : Address, val : f64, start_index : u64, end_index : u64) -> bool {
    let jmp = match ps { ps64 => 0x8, _ => 0x4};
    for i in start_index..=end_index {
        match process.read::<f64>(addr.add(i * jmp)) {
            Ok(x) if x == val => {return true;},
            _ => ()
        }
    }
    false
}

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
    asr::print_message(format!("Split triggered: {}",name).as_str());
    timer::split();
}

pub struct GVarTrack<T: Clone + bytemuck::Pod> {
    address : Address,
    watcher : Watcher<T>,
    name : &'static str,
}
impl<T: Clone + bytemuck::Pod> GVarTrack<T> {
    pub fn new(ptrs : &HashMap<&'static str,Address>, name : &'static str) -> GVarTrack<T> {
        GVarTrack {
            watcher: Watcher::<T>::new(),
            address: ptrs.get(&name).unwrap_or(&Address::NULL).clone(),
            name
        }
    }
    pub fn update_value(&mut self, process: &Process, ptrs : &HashMap<&'static str,Address>) -> &Pair<T> {
        if self.address.is_null() {
            self.address = *ptrs.get(self.name).unwrap_or(&Address::NULL);
        }
        let value = process.read::<T>(self.address).unwrap_or_else(|_e| T::zeroed());
        self.watcher.update_infallible(value)
    }
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
        let value = self.pointer.unwrap().deref(&process).unwrap_or_else(|_e| T::zeroed());
        self.watcher.update_infallible(value)
    }
}