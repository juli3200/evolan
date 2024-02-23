use std::cell::Ref;
use super::*;



// impl for thread sharing
unsafe impl Sync for World {}

impl std::fmt::Display for World{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text.push_str(&format!("Bots: {}\n", self.settings_.n_of_bots));
        text.push_str(&format!("Dim: {:?}\n", self.settings_.dim));

        write!(f, "{}", text)
    }
}

impl std::fmt::Display for Bot{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text.push_str(&format!("id: {}\n", self.id));
        text.push_str(&format!("x: {}\n", self.x));
        text.push_str(&format!("y: {}\n", self.y));
        text.push_str(&format!("cluster: {:?}\n", self.cluster));
        text.push_str(&format!("build_cluster: {}\n", self.build_cluster));

        write!(f, "{}", text)
    }

}

fn find_bot(world: &World, id: usize) -> Option<usize>{
    world.bot_vec.iter().position(|bot| bot.id == id)
}

impl World{
    pub fn new(settings_: Settings, selection_criteria: criteria::Criteria, name: String) -> Self {
        let dim = &settings_.dim;
        let n_of_bots = &settings_.n_of_bots;

        // checking input
        if dim.0 == Dow::MAX || dim.1 == Dow::MAX{panic!("dim.0/dim.1 must be smaller than Dow::Max; buffer needed")}
        if dim.0 as usize * (dim.1 as usize) < *n_of_bots as usize{
            panic!("number of objects must be smaller than dim.0*dim.1")}

        // the neuron lib is a library which is used for the creation of the genes
            let mut neuron_lib: Vec<usize> = Vec::new();
            neuron_lib.push((INPUT_NEURONS as usize).clone());

            for _ in 0..settings_.inner_layers{
                neuron_lib.push(settings_.inner_neurons.clone());
            }
            neuron_lib.push((OUTPUT_NEURONS as usize).clone());
        //

        // the bot vec contains every bot
            let mut bot_vec: Vec<Bot> = vec![];
            for i in 0..*n_of_bots as usize {
                bot_vec.push(Bot::new(neurons::create_genome(&neuron_lib, &settings_), i));
            }

        //

        // the grid is a 2d vec with Blocks in it
        let mut grid = Vec::new();
        for y in 0..dim.1{
            let mut row = Vec::new();
            for x in 0..dim.0{
                row.push(objects::Block::new(Kind::Empty, x, y));
            }
            grid.push(row);
        }

        // create the path in the cache
        let _ = fs::create_dir_all(format!(".cache/worlds/{name}/generations/"));

        World { settings_,
                selection_criteria,
                name,
                generation: 0,
                time: 0,
                age_of_gen: 0,
                killed_bots: vec![],
                bots_alive: settings_.n_of_bots,
                bot_vec,
                cluster_vec: vec![],
                cluster_ready_vec: vec![],
                barrier_block_vec: vec![],
                grid,
                neuron_lib,
                grid_store: vec![], }
    }

    pub fn spawn_barrier_blocks(&mut self, barrier_blocks_pos: Vec<(Dow, Dow)>){
        // this function adds the barrier blocks

        // check input
        if self.settings_.n_of_bots as usize + barrier_blocks_pos.len() + self.barrier_block_vec.len() > self.settings_.dim.0 as usize * self.settings_.dim.1 as usize{
            panic!("number of objects must be smaller than dim.0*dim.1")
        }

        for coord in barrier_blocks_pos.into_iter() {
            let index = self.barrier_block_vec.len();

            self.barrier_block_vec.push(objects::BarrierBlock::new(coord.0, coord.1)); // create new barrier block

            // create the raw pointer witch is passed to the Block on the coordinate 
            self.grid[coord.1 as usize][coord.0 as usize].edit_guest(Kind::BarrierBlock);
            
        }
    }

    pub fn spawn_bots(&mut self){
        let mut rng = rand::thread_rng();

    

        for (i, bot) in self.bot_vec.iter_mut().enumerate(){

            // gen coords and check validity
            let coords = loop{
                let x = rng.gen_range(0..self.settings_.dim.0) as usize;
                let y = rng.gen_range(0..self.settings_.dim.1) as usize;

                // check coords
                match self.grid[y][x].guest{
                    Kind::Empty => {break (x, y);}
                    _ =>{continue;}
                }
            };

            bot.spawn(coords.0 as Dow, coords.1 as Dow);
            // add the raw pointer to the grid
            self.grid[coords.1][coords.0].edit_guest(Kind::Bot(bot.id));
        }

        self.bots_alive = self.settings_.n_of_bots;

    }

    fn calculate_step(&mut self){
        // for every bot in self.bot_vec 
        // the function bot.neurons_to_compute is called
        // this returns a Vec of vecs(one per bot) of vecs(one per necessary gene)
        // the neurons are sorted per layer
        let input_neurons: Vec<Vec<Vec<[f64; 5]>>> = self.bot_vec.par_iter()// the process is computed in parallel with .par_iter() method
        .map(|bot: &objects::Bot| bot.calculate_input(/*make &self immutable*/&*self))
        // collect the outputs of all bots in a Vec<Vec<[f64; 2]>>
        .collect::<Vec<_>>();

        // pass to calculate.rs
        let mut output: Vec<Vec<usize>>  = vec![];
        if !self.settings_.gpu{
            // returns a vec of vec(bot) of output neurons
            output = input_neurons.par_iter().
            map(|bot| crate::calculate::calc_step(bot, &self.settings_)).collect::<Vec<_>>();
            
        }
        
        //  pass to bot.react(vec<usize>)
        // copy bot vec
        let mut bot_vec_copy = self.bot_vec.clone();

        // edit bot vec
        for (index, bot) in bot_vec_copy.iter_mut().enumerate() {
            bot.react(self, &output[index]);
        }

        // replace bot vec with edited vec

        self.bot_vec = bot_vec_copy;
        self.age_of_gen += 1;

        // update clusters
        self.update_clusters();

        // disable killing for better performance
        if self.settings_.killing_enabled{
            // removing the killed bots from the bot_vec
            for b in self.killed_bots.iter(){
                if let Some(id) = find_bot(self, *b){
                    let bot: &Bot = &self.bot_vec[id];
                    assert_eq!(bot.id, *b);
                    // if bot was recently added to cluster continue
                    if bot.cluster.is_some(){continue;}
                    self.bot_vec.retain(|b2| *b != b2.id);
                    self.bots_alive -= 1;
                }
            }

            // clearing vec of all corrupted bots and bots in clusters
            self.killed_bots = vec![];

            // resetting self.grid
            for row in self.grid.iter_mut(){
                for block in row{
                    match block.guest {
                        Kind::Bot(_) => block.guest = Kind::Empty,
                        Kind::Cluster(_) => block.guest = Kind::Empty,
                        Kind::Empty => block.guest = Kind::Empty,
                        Kind::BarrierBlock => block.guest = Kind::BarrierBlock
                    }
                }
            }

            // refilling the grid with the bots
            for bot in self.bot_vec.iter(){
                match bot.cluster{
                    Some(_) => {
                        self.grid[bot.y as usize][bot.x as usize].edit_guest(Kind::Cluster(bot.id));
                    },
                    None => {
                        self.grid[bot.y as usize][bot.x as usize].edit_guest(Kind::Bot(bot.id));
                    }
                    
                }
            }
        }

        self.grid_store.push(tools::store_gen::store_step(&*self));

        
        
    }

    fn select(&mut self){
        let sc = self.selection_criteria.clone();

        let (selected_bot_vec, survivors_grid) = sc.select(self);

        self.grid_store.push(survivors_grid);

        let mut new_bot_vec: Vec<objects::Bot> = vec![];

        if selected_bot_vec.len() == 0{

            for i in 0..self.settings_.n_of_bots as usize{
                new_bot_vec.push(objects::Bot::new(neurons::create_genome(&self.neuron_lib, &self.settings_), i));
            }

        }

        else{
            
            for i in 0..self.settings_.n_of_bots as usize{
                let b = selected_bot_vec[i%selected_bot_vec.len()];
                let b2 = selected_bot_vec[(i+1)%selected_bot_vec.len()];
                

                let new_bot = match self.settings_.inherit{
                    true => Bot::inherit((&b, &b2), &self.neuron_lib, i, &self.settings_),
                
                    false => Bot::clone_(&b, &self.neuron_lib, i, &self.settings_),
                };
               
                new_bot_vec.push(new_bot);
            }
            
        }
        self.bot_vec = new_bot_vec;


        // resetting self.grid
        for row in self.grid.iter_mut(){
            for block in row{
                match block.guest {
                    Kind::Bot(_) => block.guest = Kind::Empty,
                    Kind::Cluster(_) => block.guest = Kind::Empty,
                    Kind::Empty => block.guest = Kind::Empty,
                    Kind::BarrierBlock => block.guest = Kind::BarrierBlock
                }
            }
        }
        self.spawn_bots();

    }

    fn update_clusters(&mut self){
        // this function is called from calculate_step
        // it checks if the bots are ready to form a cluster
        // and if a cluster needs to be deleted
        
        // build new clusters: 

        fn search_neighbours(world: &World, bot_index: usize, neighbours: &mut Vec<usize>){
            let neighbour_coords = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            
            // extract the bot from the bot_register

            // check if bot is dead (should not happen, but just in case)
            let bot;
            if let Some(id) = find_bot(world, bot_index){
                bot = &world.bot_vec[id];
                assert_eq!(bot.id, bot_index);}

            else {
                return;
            }

            for n in neighbour_coords.iter(){
            

                // get coords of bot
                let x = bot.x as isize + n.0;
                let y = bot.y as isize + n.1;
                // check coords ( 0<=x<=dim.0, 0<=y<=dim.1)
                if x < 0 || x >= world.settings_.dim.0 as isize || y < 0 || y >= world.settings_.dim.1 as isize{
                    continue;
                }

                let guest = world.grid[y as usize][x as usize].guest.clone();

                // search for neighbours in world.grid
                match guest{
                
                    Kind::Bot(neighbour) => {
                        // check if: 
                        // 1. neighbour is ready
                        // 2. neighbour is not already in the neighbours vec
                        // 3. neighbour is not already in a cluster
                        // if all conditions are met, add the id to the neighbours vec
                        if world.generation == 30 && world.age_of_gen == 20{
                            println!("{}", *bot)
                        }
                        if let Some(neighbour_bot) =  find_bot(world, neighbour){
                            let neighbour_bot: &Bot = &world.bot_vec[neighbour_bot];
                            if world.generation == 30 && world.age_of_gen == 20{
                                println!("{}", *neighbour_bot)
                            }
                            if neighbour_bot.build_cluster && !neighbours.contains(&neighbour) && neighbour_bot.cluster.is_none() {
                                neighbours.push(neighbour);
                                // search for neighbours of the neighbour with recursion
                                search_neighbours(world, neighbour, neighbours);
                            }
                        }},

                        
                    _ => {}
                }
            }
            
            

        }

        let ready_bots = self.cluster_ready_vec.clone();

        for id in ready_bots.iter(){
            // if bots where already added to a cluster, continue
            // this can happen if the bot was added with a bot before
            let bot;

            if let Some(bot_index) = find_bot(self, *id){
                bot = &self.bot_vec[bot_index];
            }
            else {
                continue;
            }


            if !bot.build_cluster{println!("{}", *bot);continue;}

            // vec of all bots next to the bot who want to form a cluster
            let mut neighbours: Vec<usize> = vec![];

            // check all neighbours and add the id to the neighbours vec with recursion
            search_neighbours(self, *id, &mut neighbours);

            if neighbours.len() > 1{
                // create a new cluster with the neighbours
                self.cluster_vec.push(cluster::Cluster::new(neighbours.clone()));

                let cluster_id = self.cluster_vec.len() - 1;

                for b in neighbours.into_iter(){
                    
                    // set the cluster of the bot
                    if let Some(b_index) = find_bot(self, b){

                        self.bot_vec[b_index].cluster = Some(cluster_id);
                        //println!("Bot {} is in cluster {}", self.bot_vec[b_index].id, cluster_id);
                    }
                }


            }
        } 
              

    }

    pub fn calculate_generation(&mut self){
        for _ in 0..self.settings_.generation_steps{
            self.calculate_step();
        }
   
        self.select();

        self.age_of_gen = 0;
        self.generation += 1;
        self.bots_alive = self.settings_.n_of_bots;
        self.cluster_vec = vec![];
        self.cluster_ready_vec = vec![];

        tools::store_gen::store_generation(&*self);
        self.grid_store = vec![];

        self.spawn_bots();

    }


}