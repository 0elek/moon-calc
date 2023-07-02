# moon_calc

```rust
use moon_calc::Moon;
use std::time::SystemTime;

let moon = Moon::new(SystemTime::now());

```

```rust
moon.julian_date // julian date
moon.phase // phase 0 is new moon, 0.5 is full moon...
moon.age // age in days
moon.illumination // fraction of the moon that is illuminated 0 - 1
moon.lunation // lunation number https://robinfo.oma.be/en/astro-info/moon/lunation/
moon.distance // distance in earth radii

// functions 

moon.distance_km() // returns distance in km
moon.is_waning() // returns true if waning (decreasing)
moon.is_waxing() // returns true if waxing (increasing)
moon.phase_name() // returns name of phase
moon.phase_emoji() // returns emoji of phase in the northen hemisphere 

```
