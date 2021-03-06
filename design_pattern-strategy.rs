#![crate_name="design_pattern-strategy"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Example of design pattern inspired from Head First Design Patterns"]
//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-0.12-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-12

// Variations are encapsulated into external objects.
// Here, it is the fly function.
trait FlyBehaviour {
	fn fly(&self);
}

struct FlyWithWings;

impl FlyBehaviour for FlyWithWings {
	fn fly(&self) {
		println!("I can fly using my wings!");
	}
}

struct DoNotFly;

impl FlyBehaviour for DoNotFly {
	fn fly(&self) {
		println!("I can't fly!");
	}
}

// The object has reference to the variation.
struct Duck<'a> {
	fly_behaviour: Box<FlyBehaviour + 'a>,
}

impl<'a> Duck<'a> {
	// a method calls the funciton in the variation.
	fn fly(&self) {
		self.fly_behaviour.fly();
	}
	fn set_fly_behaviour(&mut self, fly_behaviour: Box<FlyBehaviour + 'a>) {
		self.fly_behaviour = fly_behaviour;
	}
}

fn main() {
	let dnf = DoNotFly;
	let fww = FlyWithWings;
	let mut ducky = Duck { fly_behaviour: box fww };
	ducky.fly();
	// so functions can change dynamically
	ducky.set_fly_behaviour(box dnf);
	ducky.fly();
}
