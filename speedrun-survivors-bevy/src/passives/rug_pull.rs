/**
continuously spawns new rugs

each rug moves along fixed path (straight heading) towards a random direction determined at spawn

any enemy contacted moves for x:f32 seconds into the direction the rug is moving
possibly deal x damage on impact / while affected

gameplay stats
rug spawn rate
rug speed
rug pull time
rug impact(?) damage

*/
pub struct RugPullPlugin;
