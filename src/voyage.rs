#[derive(Copy, Clone)]
pub struct Event {
   // encapsulates an event of the spaceship at a particular distance
   // and time  along with the velocity of the ship, and time on the
   // ship clock
   pub here: f64, // distance as viewed from earth frame
   // 14 light years ~= 1.3E17; minimum distance increment
   // ath this point is 16m.
   // It may be better to work witu u64 (or u128) and
   // lose the first few sub-metres instead of the last
   // few sub-16-metres
   pub now: f64,   // time elapsed as per earth frame. (how about u64?)
   pub speed: f64, // at 'reference' 14ly,1.5g half-way speed of
   // 2.989E8m/s minimum increment is 6E-8, but reference
   // speed increment is about 0.08. All seems well.
   pub mtime: f64, // Time elapsed on Mercy's ship
}

impl Default for Event {
   fn default() -> Self {
      Self {
         here: 0.0,
         now: 0.0,
         speed: 0.0,
         mtime: 0.0,
      }
   }
}

fn fwd(ship: Event, acceleration: f64) -> Event {
   // return a new Event with the moved ship
   let dmtime = (9.0e16 - ship.speed * ship.speed).sqrt() / 3.0e8;
   let speedup = acceleration * dmtime;
   let newspeed = (speedup + ship.speed) / (1.0 + speedup * ship.speed / 9.0e16);
   Event {
      here: ship.here + ship.speed,
      now: ship.now + 1.0,
      speed: newspeed,
      mtime: ship.mtime + dmtime,
   }
}

pub fn arrive(ship: Event, acceleration: f64, distance: f64) -> Event {
   let mut aship = ship;
   while aship.here < distance && ship.speed >= 0.0 {
      aship = fwd(aship, acceleration);
   }
   return aship;
}

fn _dfwd(speed: f64, acceleration: f64) -> Event {
   fwd(
      Event {
         here: 0.0,
         now: 0.0,
         speed: speed,
         mtime: 0.0,
      },
      acceleration,
   )
}

fn _travel(distance: f64, acceleration: f64, start_speed: f64) -> Event {
   // we're using a mutable ship, but fwd() returns a new event. Is
   // this the most efficient way?
   let mut ship = Event {
      here: 0.0,
      now: 0.0,
      speed: start_speed,
      mtime: 0.0,
   };

   while ship.here < distance && ship.speed >= 0.0 {
      ship = fwd(ship, acceleration);
   }

   // let overshoot = ship.here - distance;
   // let uspeed = ship.speed / 3.0e8;
   // let unow = ship.now / 3600.0 / 24.0 / 365.0;
   // let umtime = ship.mtime / 3600.0 / 24.0 / 365.0;
   // println!("Overshoot: {overshoot}\nSpeed: {uspeed}c\nTime: {unow} years\nMercy's stopwatch: {umtime} years");
   return ship;
}

pub fn _fly() {
   let journeys_end = 14.0 * 3e8 * 3600.0 * 24.0 * 365.0;
   let acceleration = 1.5 * 9.81;
   let ship_halfway = _travel(journeys_end / 2.0, acceleration, 0.0);
   println!();
   let ship_end = _travel(
      journeys_end - ship_halfway.here,
      -acceleration,
      ship_halfway.speed,
   );

   let unow = (ship_halfway.now + ship_end.now) / 3600.0 / 24.0 / 365.0;
   let umtime = (ship_halfway.mtime + ship_end.mtime) / 3600.0 / 24.0 / 365.0;
   println!("\nTotal time: {unow} years\nTotal Mercy Time: {umtime} years");
}
