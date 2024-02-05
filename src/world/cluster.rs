///
/// clusters are colonies of bots
/// all decisions are made by all bots together:
///    - movement
///    - if a bot is able to join
/// a bot can only leave if the cluster decides to desolve itself completely
/// or if the generation ends
/// 
/// clusters do not reproduce, but can help a bot to survive
/// 


//todo: make input and output functions for clusters
// make caqlculate for clusters

#[derive(Clone, Debug)]
pub struct Cluster{
    // u16 are indiecies to world.bot_vec
    participants: Vec<u16>,

    // params are evaluated by combining all params of childern
    // eg if cluster has two participants one facinging north and the other facing east
    // the cluster will face norteast

    // all movements vectors are combined to get the cluster movement
    angle:Option<u8>,

    // if most of participants.built_cluster is false the cluster is deleted
    build_cluster: bool,

}

impl Cluster {
    pub fn new(participants: Vec<u16>) -> Cluster {
        Cluster {
            participants,
            angle: None,
            build_cluster: true,
        }
    }

    pub fn add_participant(&mut self, bot_index: u16) {
        self.participants.push(bot_index);
    }

    
}

