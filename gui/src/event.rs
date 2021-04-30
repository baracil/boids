
#[derive(Debug)]
pub enum Event {
    Click(ClickPar),
    Drag(DragPar)
}

#[derive(Debug)]
pub struct ClickPar {
    action_id:String,
}



impl ClickPar {
    pub fn new(action_id:&String) -> Self {
        Self{action_id:action_id.to_owned()}
    }

    pub fn action_id(&self) -> &str {
        &self.action_id
    }
}


#[derive(Debug)]
pub struct DragPar {
    action_id:String,
    value:f32,
    in_progress:bool,
}

impl DragPar {

    pub fn in_progress(action_id:String,value:f32) ->Self {
        Self{action_id:action_id.to_owned(),value, in_progress:true}
    }

    pub fn done(action_id:String,value:f32) ->Self {
        Self{action_id:action_id.to_owned(),value, in_progress:false}
    }

}