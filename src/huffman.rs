use log::{ info, error, debug, trace };
use std::collections::HashMap;
use std::collections::BTreeMap;

extern crate minheap;
use minheap::MinHeap;


#[derive(Debug)]
pub struct SimpleInfo {
    weight: u64,
}

#[derive(Debug)]
pub struct CompoundInfo {
    weight: u64,
    low_id: u32,
    high_id: u32,

}

#[derive(Debug)]
pub enum VertexInfo {
    Simple(SimpleInfo),
    Compound(CompoundInfo)
}

impl VertexInfo {
    pub fn get_weight(&self) -> u64 {
        match self {
            Simple(a) => a.weight,
            Compound(a) => a.weight,
        }

    }

    pub fn new_simple(weight:u64) -> Self {
        Simple(SimpleInfo { weight: weight })
    }

}

use std::cmp::Ordering;



impl Ord for VertexInfo {
    fn cmp(&self, other: &Self) -> Ordering {

        self.get_weight().cmp(&other.get_weight())
        /*
        match (self, other) {
            (Simple(a), Simple(b)) =>  a.weight.cmp(&b.weight),
            (Simple(a), Compound(b)) =>  a.weight.cmp(&b.weight),
            (Compound(a), Simple(b)) =>  a.weight.cmp(&b.weight),
            (Compound(a), Compound(b)) =>  a.weight.cmp(&b.weight),
            _ => Ordering::Less
        }
        */
    }
}

impl PartialOrd for VertexInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        match (self, other) {
            (Simple(a), Simple(b)) =>  a.weight.partial_cmp(&b.weight),
            (Simple(a), Compound(b)) =>  a.weight.partial_cmp(&b.weight),
            (Compound(a), Simple(b)) =>  a.weight.partial_cmp(&b.weight),
            (Compound(a), Compound(b)) =>  a.weight.partial_cmp(&b.weight),
        }
    }
}

impl PartialEq for VertexInfo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Simple(a), Simple(b)) =>  a.weight == b.weight,
            (Simple(a), Compound(b)) =>  a.weight == b.weight,
            (Compound(a), Simple(b)) =>  a.weight == b.weight,
            (Compound(a), Compound(b)) =>  a.weight == b.weight,
        }
    }
}

impl Eq for VertexInfo { }

use VertexInfo::Simple;
use VertexInfo::Compound;


//#[derive(Debug)]
pub struct HuffmanInfo {
    vertex_heap: MinHeap<VertexInfo>,
    vertex_list: HashMap::<u32,VertexInfo>,
    huffman_codes: BTreeMap::<u32,String>,
    next_id : u32,
    min_len: usize,
    max_len: usize,
}



impl HuffmanInfo {

    pub fn new() -> Self {
        HuffmanInfo {
            vertex_heap : MinHeap::<VertexInfo>::new(),
            vertex_list : HashMap::<u32,VertexInfo>::new(),
            huffman_codes: BTreeMap::<u32,String>::new(),
            next_id: 0,
            min_len: usize::MAX,
            max_len: 0,
        }
        
    }

    pub fn add_simple_vertex(&mut self,weight:u64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.vertex_heap.insert(id,Simple(SimpleInfo{weight}));
        self.vertex_list.insert(id,Simple(SimpleInfo{weight}));
        debug!("Added Simple Vertex {}  w={}",id,weight);
        id
    }

    pub fn add_combined_vertex(&mut self,weight:u64, low_id: u32, high_id: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.vertex_heap.insert(id,Compound(CompoundInfo {weight: weight, low_id: low_id, high_id: high_id}));
        self.vertex_list.insert(id,Compound(CompoundInfo {weight: weight, low_id: low_id, high_id: high_id}));
        debug!("Added Combined Vertex {}  w={} low={} high={}",id,weight,low_id, high_id);
        id
    }

    pub fn get_min(&mut self) -> Option<(u32, VertexInfo )> {
        self.vertex_heap.get_min_entry()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.vertex_list.len(), self.vertex_heap.len())
    }

    pub fn combine(&mut self, id1: u32, id2: u32) -> Option<u32> {
        //TODO add error checking
        let mut low_id = id2;
        let mut high_id = id1;
        let data1 = self.vertex_list.get(&id1);
        let data2 = self.vertex_list.get(&id2);

        match (data1,data2) {
            (Some(v1), Some(v2)) => {
                if v1 < v2 {
                    low_id = id1;
                    high_id = id2;
                }
                debug!("v1 {:#?} v2 {:#?}",v1,v2);
                let combined_weight = v1.get_weight() + v2.get_weight();
                Some(self.add_combined_vertex(combined_weight,low_id,high_id))
            }
            _ => {

                error!("Invalid id ({} {})",id1,id2);
                None
            }

        }
    }

    pub fn generate_huffman(&mut self, id: u32, prefix: String) {

        if let Some(v_info) = self.vertex_list.get(&id) {
            trace!("Generate Huffman: {} {:#?}",id, v_info);
            match v_info {
                Simple(_weight) => {
                        if prefix.len() > self.max_len {
                            self.max_len = prefix.len();
                        }
                        if prefix.len() < self.min_len {
                            self.min_len = prefix.len();
                        }
                        debug!("Defining code for ID {} -> {} (max_len {})",id,prefix,self.max_len);
                        self.huffman_codes.insert(id,prefix);
                    }
                Compound(combined) => {
                    let low_id = combined.low_id.clone();
                    let high_id = combined.high_id.clone();

                    self.generate_huffman(low_id,prefix.clone()+"0");
                    self.generate_huffman(high_id,prefix.clone()+"1");
                }
            }
        }
        else {
            error!("generated huffman - invalid Id ({})",id);
        }

    }


    pub fn process(&mut self) {
        info!("Starting Processing");

        while self.vertex_heap.len() > 1 {
            let v1 = self.get_min().unwrap();
            let v2 = self.get_min().unwrap();
            let _v_new = self.combine(v1.0,v2.0);
        }

        let (id, _initial) = self.get_min().unwrap();
        self.generate_huffman(id,"".to_string());
        info!("Min, Max len of huffman codes is {},{}",self.min_len,self.max_len,);
    }

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn init () {
        let _ = env_logger::builder().is_test(true).try_init();
        info!("Init {}",module_path!())
    }

    fn setup_basic() -> HuffmanInfo {

        let mut h = HuffmanInfo::new();
        assert_eq!(h.add_simple_vertex(10),0);
        assert_eq!(h.add_simple_vertex(20),1);
        assert_eq!(h.add_simple_vertex(30),2);
        assert_eq!(h.add_simple_vertex(5),3);
        assert_eq!(h.size(),(4,4));
        h
    }

    #[test]
    fn test_add() {
        init();
        let mut h = setup_basic();
        assert_eq!(h.get_min(),Some((3,VertexInfo::new_simple(5))));

    }

    #[test]
    fn test_combine() {
        init();
        let mut h = setup_basic();
        let v1 = h.get_min().unwrap();
        let v2 = h.get_min().unwrap();
        assert_eq!(v1,(3,VertexInfo::new_simple(5)));
        assert_eq!(v2,(0,VertexInfo::new_simple(10)));
        let v_new = h.combine(v1.0,v2.0);
        assert_eq!(v_new,Some(4));
    }

    #[test]
    fn test_process() {
        init();
        let mut h = setup_basic();
        h.process();
        println!("Huffman codes {:#?}",h.huffman_codes);
        trace!("Vertexes {:#?}",h.vertex_list);

    }


 }
