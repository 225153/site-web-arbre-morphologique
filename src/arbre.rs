
pub struct Derive {
    pub mot: String,
    pub schema: String,
}

pub struct RacineNode {
    pub racine: [char; 3],              
    pub left: Option<Box<RacineNode>>,  
    pub right: Option<Box<RacineNode>>, 
}


pub fn morphologic_cmp(tree_racine : [char;3], racine : [char;3])-> i8 {
    let mut i = 0 ; 
    while i < 3 {
        if tree_racine[i] == racine[i] {
            i = i + 1 ;
        }
        else {
            if tree_racine[i] > racine[i]{
                return -1 ;
            }
            else {
                return 1 ;
            }
        }
    }
    return 0 ;
}


pub struct Tree{
    pub racine : Option<Box<RacineNode>> ,
};


impl RacineNode{
    pub fn new(racine: [char; 3]) -> Self {
        RacineNode{
            racine,
            left: None ,
            right : None,
        }
    }
    pub fn verify_node(&self, ch : [char;3])-> bool {
        let cmp = morphologic_cmp(self.racine, ch) ; 
        if cmp == 0 {return true ;}
        else {
            if cmp == -1 {
                if self.left == None {
                    return false ;
                }
                else {
                    return self.left.as_ref().unwrap().verify_node(ch);
                }
            }
            else {
                if self.right == None {
                    return false ;
                }
                else{
                    return self.right.as_ref().unwrap().verify_node(ch)
                }
            }
        }

    }
}
impl Tree {
    pub fn new() -> Self { 
        Tree { racine: None }
    }
    pub fn verify(&self, ch : [char;3])-> bool {
        if self.racine.is_none() {
            return false ;
        }
        let node = self.racine.as_ref().unwrap() ; 
        return node.verify_node(ch) ; 

    }
}
