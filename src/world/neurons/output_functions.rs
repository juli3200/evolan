/*
    // angle is turned +90 or -90
    TurnRight,
    TurnLeft,

    // zero is backwards 1 is forwards
    MoveStraight(bool),
    // left or right movement
    MoveSideways(bool),
    // move in x_direction; 1 positive x, -1 negative
    MoveX(bool),
    // move in y direction
    MoveY(bool),
    // move in rnd deirection
    MoveRandom(u8),

    // send letter
    SendComm(u8),

    // can live for a specific time; really high value to be fired
    PlaceBarrierBlock,

    // mutation and modification
    // these Neurons need an extrem high value to be fired
    Mutate,
    // modification
    Modify,

    // kill neuron can be deactivated
    // kill bot in front
    Kill */
