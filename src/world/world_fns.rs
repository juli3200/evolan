use super::*;

// impl for thread sharing
unsafe impl <'a>Sync for World<'a> {}

impl <'a>std::fmt::Display for World<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text.push_str(&format!("Bots: {}\n", self.settings_.n_of_bots));
        text.push_str(&format!("Dim: {:?}\n", self.settings_.dim));

        write!(f, "{}", text)
    }
}

impl <'a>World<'a>{
    pub fn new(settings_: settings::Settings, selection_criteria: criteria::Criteria, name: String) -> Self {
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
            let mut bot_vec: Vec<objects::Bot> = vec![];
            for i in 0..*n_of_bots {
                bot_vec.push(objects::Bot::new(neurons::create_genome(&neuron_lib, &settings_), i));
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
        let _ = fs::create_dir_all(format!("cache/worlds/{name}/generations/"));

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
            self.grid[coords.1][coords.0].edit_guest(Kind::Bot(i as u16));
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
        // todo: create fn in calculate.rs
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
        // disable killing for better performance
        if self.settings_.killing_enabled{
            self.killed_bots.sort_by(|a, b| b.cmp(a));
            // removing the killed bots from the bot_vec
            for index in self.killed_bots.iter(){
                self.bot_vec.remove(*index as usize);
            }
            // setting new ids
            for (index, bot) in self.bot_vec.iter_mut().enumerate(){
                bot.id = index as u16;
            }

            // resetting self.grid
            for row in self.grid.iter_mut(){
                for block in row{
                    match block.guest {
                        Kind::Bot(_) => block.guest = Kind::Empty,
                        Kind::Empty => block.guest = Kind::Empty,
                        Kind::BarrierBlock => block.guest = Kind::BarrierBlock
                    }
                }
            }

            // refilling the grid with the bots
            for bot in self.bot_vec.iter(){
                self.grid[bot.y as usize][bot.x as usize].edit_guest(Kind::Bot(bot.id));
            }
            self.killed_bots = vec![];
        }

        self.grid_store.push(tools::store_gen::store_step(&*self));
        
        
    }

    fn select(&mut self){

        let (selected_bot_vec, survivors_grid) = self.selection_criteria.select(self);

        self.grid_store.push(survivors_grid);

        let mut new_bot_vec: Vec<objects::Bot> = vec![];

        if selected_bot_vec.len() == 0{

            for i in 0..self.settings_.n_of_bots{
                new_bot_vec.push(objects::Bot::new(neurons::create_genome(&self.neuron_lib, &self.settings_), i));
            }

        }

        else{
            
            for i in 0..self.settings_.n_of_bots{
                let b = selected_bot_vec[i as usize%selected_bot_vec.len()];
                let b2 = selected_bot_vec[(i+1) as usize%selected_bot_vec.len()];
                

                let new_bot = match self.settings_.inherit{
                    true => objects::Bot::inherit((&b, &b2), &self.neuron_lib, i, &self.settings_),
                
                    false => objects::Bot::clone_(&b, &self.neuron_lib, i, &self.settings_),
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
        
        // todo: check all ready bots and check if they are next to each other
        // if they are next to each other, they form a cluster

        // vec of all coords of the ready bots
        let cluster_ready_coords = self.cluster_ready_vec.iter()
        .map(|id| (self.bot_vec[*id as usize].x, self.bot_vec[*id as usize].y)).collect::<Vec<_>>();

        let neighbour_coords = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (i, id) in self.cluster_ready_vec.iter().enumerate(){
            let mut neighbours = vec![];
            for n in neighbour_coords.iter(){
                let x = cluster_ready_coords[i].0 as isize + n.0;
                let y = cluster_ready_coords[i].1 as isize + n.1;
                if x < 0 || x >= self.settings_.dim.0 as isize || y < 0 || y >= self.settings_.dim.1 as isize{
                    continue;
                }
                ///
                /// error
                /// delete this
                if cluster_ready_coords.contains(&(x as Dow, y as Dow));
                match self.grid[y as usize][x as usize].guest{
                    Kind::Bot(id) => neighbours.push(id),
                    _ => {}
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

        tools::store_gen::store_generation(&*self);
        self.grid_store = vec![];

        self.spawn_bots();

    }

}
