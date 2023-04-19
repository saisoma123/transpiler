use std::collections::HashMap;
use std::time::Instant;

fn propagate<'a, 'b, 'c>(agent_id : &'a str, universe : &'b HashMap<&str, HashMap<&str,f64>>) -> HashMap<&'c str, f64> {
		let now = Instant::now();
		let state = &universe[agent_id];
		let (time, time_step, mut x, mut y, mut vx, mut vy) = (*(state.get(&"time").unwrap()), *(state.get(&"time_step").unwrap()),*(state.get(&"x").unwrap()),*(state.get(&"y").unwrap()),*(state.get("vx").unwrap()),*(state.get(&"vy").unwrap()));
	 if agent_id == "Planet" {
		 x+=vx*time_step;
		 y+=vy*time_step;
	 }
	 else if agent_id == "Satellite" {
		 let (px, py) =(*(universe.get(&"Planet").unwrap().get(&"x").unwrap()), *(universe.get(&"Planet").unwrap().get(&"y").unwrap()));
		 let dx=px-x;
		 let dy=py-y;
		 let fx= dx / (dx*dx+dy*dy);
		 let fy= dy / (dx*dx+dy*dy);
		 vx+=fx*time_step;
		 vy+=fy*time_step;
		 x+=vx*time_step;
		 y+=vy*time_step;
	 }
	 let map = HashMap::from([("time",time+time_step),("time_step",0.01*0.09),("x",x),("y", y),("vx",vx),("vy",vy)]);
	 let elapsed_time = now.elapsed();
		println!("Running slow_function() took {} seconds.", elapsed_time.as_millis());
   return map;
}
fn main() {
	let now = Instant::now();
	let agent_id = "Planet";
	let inita = HashMap::from([("Planet",HashMap::from([("time", 0.0), ("time_step", 0.01), ("x", 0.0), ("y", 0.11), ("vx", 0.9), ("vy", 0.0)])), ("Satellite", HashMap::from([("time", 0.0), ("time_step", 0.01), ("x", 0.0), ("y", 1.0), ("vx", 1.0), ("vy", 0.0)]))]);
	let _ret = propagate(&agent_id, &inita);
	let elapsed_time = now.elapsed();
	println!("Running slow_function() took {} seconds.", elapsed_time.as_millis())
}