use std::collections::HashMap;


fn propagate<'a, 'b, 'c>(agentId : &'a str, universe : &'b HashMap<&str, HashMap<&str,f64>>) -> HashMap<&'c str, f64> {
  let mut inita = HashMap::from([("Planet",([("time", 0.0), ("timeStep", 0.01), ("x", 0.0), ("y", 0.11), ("vx", 0.9), ("vy", 0.0)])), ("Satellite", [("time", 0.0), ("timeStep", 0.01), ("x", 0.0), ("y", 1.0), ("vx", 1.0), ("vy", 0.0)])]);
    let mut state = &universe[agentId];
		let (mut time, mut timeStep, mut x, mut y, mut vx, mut vy) = (*(state.get(&"time").unwrap()), *(state.get(&"timeStep").unwrap()),*(state.get(&"x").unwrap()),*(state.get(&"y").unwrap()),*(state.get("vx").unwrap()),*(state.get(&"vy").unwrap()));
	 if agentId == "Planet" {
		 x+=vx*timeStep;
		 y+=vy*timeStep;
	 }
	 else if agentId == "Satellite" {
		 let (mut px, mut py) =(*(universe.get(&"Planet").unwrap().get(&"x").unwrap()), *(universe.get(&"Planet").unwrap().get(&"y").unwrap()));
		 let dx=px-x;
		 let dy=py-y;
		 let fx= dx / (dx*dx+dy*dy);
		 let fy= dy / (dx*dx+dy*dy);
		 vx+=fx*timeStep;
		 vy+=fy*timeStep;
		 x+=vx*timeStep;
		 y+=vy*timeStep;
	 }
	 let map = HashMap::from([("time",time+timeStep),("timeStep",0.01*0.09),("x",x),("y", y),("vx",vx),("vy",vy)]);
   return map;
}
fn main() {
	let agentId = "Planet";
	let universe = HashMap::from([("Planet",([("time", 0.0), ("timeStep", 0.01), ("x", 0.0), ("y", 0.11), ("vx", 0.9), ("vy", 0.0)])), ("Satellite", [("time", 0.0), ("timeStep", 0.01), ("x", 0.0), ("y", 1.0), ("vx", 1.0), ("vy", 0.0)])]);
	let ret = propagate(&agentId, &universe);
}