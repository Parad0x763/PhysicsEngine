use crate::core::Vector3;

pub struct Particle {
    /// Holds the linear postion of the particle.
    pub position: Vector3,
    /// Holds the linear velocity of the particle.
    pub velocity: Vector3,
    /// Holds the acceleration of the particle. This value
    /// can be used to set the acceleration due to gravity (its primary use),
    /// or any other constant acceleration.
    pub acceleration: Vector3,
    /// Holds the amount of damping applied to linear motion.
    /// Damping is required to remove energy added through numerical instability
    /// in the integrator.
    pub damping: f32,
    mass: f32,
    /// Holds the inverse of the mass of the particle.
    /// It is more useful to hold the inverse mass because integration is simpler,
    /// and because in real-time simulation it is more usefule to have objects with
    /// infinite mass (immovable) than zero mass
    /// (completely unstable in numerical simulation).
    inverse_mass: f32,
}

impl Particle {
    pub fn new(
        position: Vector3,
        velocity: Vector3,
        acceleration: Vector3,
        damping: f32,
        mass: f32
    ) -> Particle {
        return Particle {
            position,
            velocity,
            acceleration,
            damping,
            mass,
            inverse_mass: 1.0 / mass
        };
    }
    
    pub fn integrate(&mut self, duration: f32) { // maybe use the Duration struct that will be given from the Instance struct
        // We don't integrate things with infinite mass.
        if self.inverse_mass <= 0.0 { return; }

        assert!(duration > 0.0);

        // Update linear position
        self.position.add_scaled_vector(&self.velocity, duration);
        
        // Work out the acceleration from the force.
        // (We'll add to this vector when we come to generate forces.)
        let resulting_acc: Vector3 = self.acceleration;
        
        // Update the linear velocity from the acceleration
        self.velocity.add_scaled_vector(&resulting_acc, duration);
        
        // Impose drag
        self.velocity *= self.damping.powf(duration);
        
        // Clear the forces
        self.clear_accumulator();
    }
    
    pub fn clear_accumulator(&mut self) {
        self.velocity = Vector3::default();
        self.acceleration = Vector3::default();
    }
    
    pub fn calculate_kinetic_energy(&self) -> f32 {
        return 0.5 * self.mass * self.velocity.square_magnitude();
    }
    
    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }
    
    pub fn get_mass(&self) -> f32 {
        return self.mass;
    }
    
    pub fn set_inverse_mass(&mut self) {
        self.inverse_mass = 1.0 / self.mass;
    }
    
    pub fn get_inverse_mass(&self) -> f32 {
        return self.inverse_mass;
    }
}
