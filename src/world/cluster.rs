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
    angle:u8,
    build_cluster: bool,
    


}

