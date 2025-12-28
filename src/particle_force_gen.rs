use crate::{core::Vector3, particle::Particle};

pub struct ParticleForceRegistration {
    particle: Particle,
}

impl ParticleForceGenerator for ParticleForceRegistration {
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        
    }
}

pub struct ParticleForceRegistry {
    registry: Vec<ParticleForceRegistration>,
}

impl ParticleForceRegistry {
    /// Registers the given force generator to apply to the given particle
    fn add(&self, particle: &Particle, force_gen: Box<dyn ParticleForceGenerator>) {}
    /// Removes the given registered pair from the registry.
    /// If the pair is not registered, this method will have no effect.
    fn remove(&self, particle: &Particle, force_gen: Box<dyn ParticleForceGenerator>) {}
    /// Clears all registrations from the registry. This will not delete the particles or the force
    /// generators themselves, just the records of their connection.
    fn clear(&mut self) {
        self.registry.clear();
    }
    /// Calls all the force generators to update the forces of their corresponding particles
    fn update_forces(&mut self, duration: f32) {
        for r in self.registry.iter_mut() {
            let mut particle: Particle = Particle::new(
                r.particle.position,
                r.particle.velocity,
                r.particle.acceleration,
                r.particle.damping,
                r.particle.get_mass(),
            );
            r.update_force(&mut particle, duration);
        }
    }
}

pub trait ParticleForceGenerator {
    /// Overload this in implementations of the interface to calculate and
    /// update the force applied to the given particle
    fn update_force(&mut self, particle: &mut Particle, duration: f32);        
}

pub struct ParticleGravity {
    gravity: Vector3
}

impl ParticleGravity {
    fn new(gravity: &Vector3) -> ParticleGravity {
        return ParticleGravity {
            gravity: *gravity
        };
    }
}

impl ParticleForceGenerator for ParticleGravity {
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        // Check that we do not have infinite mass.
        if !particle.has_finite_mass() { return; }
        
        // Apply the mass-scaled force to the particle
        particle.add_force(self.gravity * particle.get_mass());
    }
}

pub struct ParticleDrag {
    /// Holds the velocity drag coefficient
    k1: f32,
    /// Holds the velocity sqaured drag coefficient
    k2: f32,
}

impl ParticleDrag {
    fn new(k1: f32, k2: f32) -> ParticleDrag {
        return ParticleDrag {
            k1: k1,
            k2: k2
        }
    }
}

impl ParticleForceGenerator for ParticleDrag {
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        let mut force: Vector3 = particle.get_velocity();
        
        // Calculate the total drag coefficient
        let mut drag_coefficient: f32 = force.magnitude();
        drag_coefficient = self.k1 * drag_coefficient + self.k2 * drag_coefficient * drag_coefficient;
        
        // Calculate the final force and apply it
        force.normalize();
        force *= -drag_coefficient;
        particle.add_force(force);
    }
}

struct ParticleSpring {
    /// The particle at the other end of the spring
    other: Particle,
    /// Holds the spring constant
    spring_constant: f32,
    /// Holds the rest lenght of the spring
    rest_length: f32,
}

impl ParticleSpring {
    fn new(other: Particle, spring_constant: f32, rest_length: f32) -> ParticleSpring {
        return ParticleSpring {
            other,
            spring_constant,
            rest_length
        }
    }
}

impl ParticleForceGenerator for ParticleSpring {
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        // Calculate the vector of the spring
        let mut force: Vector3 = particle.get_position();
        force -= &self.other.get_position();
        
        // Calculate the magnitude of the force
        let mut magnitude: f32 = force.magnitude();
        magnitude = (magnitude - self.rest_length).abs();
        magnitude *= self.spring_constant;
        
        // Calculate the final force and apply it
        force.normalize();
        force *= -magnitude;
        particle.add_force(force);
    }
}

pub struct ParticleAnchoredSpring {
    anchor: Vector3,
    spring_constant: f32,
    rest_length: f32,
}

impl ParticleAnchoredSpring {
    fn new(
        anchor: Vector3,
        spring_constant: f32,
        rest_length: f32
    ) -> ParticleAnchoredSpring {
        return ParticleAnchoredSpring {
            anchor,
            spring_constant,
            rest_length
        }
    }
    /// Updates the anchor position
    /// Could be used for having the camara follow the player as they move
    fn set_anchor(&mut self, anchor_update: Vector3) {
        self.anchor.update_by_vector3(anchor_update);
    }
}

impl ParticleForceGenerator for ParticleAnchoredSpring {
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        // Calculate the vector of the spring
        let mut force: Vector3 = particle.get_position();
        force -= &self.anchor;
        
        // Calculate the magnitude of the force
        let mut magnitude: f32 = force.magnitude();
        magnitude = (self.rest_length - magnitude) * self.spring_constant;
        
        // Calculate the final force and apply it
        force.normalize();
        force *= magnitude;
        particle.add_force(force);
    }
}

pub struct ParticleBungee {
    other: Particle,
    spring_constant: f32,
    rest_length: f32,
}

impl ParticleBungee {
    fn new(
        other: Particle,
        spring_constant: f32,
        rest_length: f32
    ) -> ParticleBungee {
        return ParticleBungee {
            other,
            spring_constant,
            rest_length
        };
    }
}

impl ParticleForceGenerator for ParticleBungee {
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        // Calculate the vector of the spring
        let mut force: Vector3 = particle.get_position() - &self.other.get_position();
        
        // Check if the bungee is compressed
        let mut magnitude: f32 = force.magnitude();
        if magnitude <= self.rest_length { return; }
        
        // Calculate the magnitude of the force
        magnitude = self.spring_constant * (self.rest_length - magnitude);
        
        // Calculate the final force and apply it
        force.normalize();
        force *= -magnitude;
        particle.add_force(force);
    }
}

pub struct ParticleBuoyancy {
    /// Maximum submersion depth of the object before it generates its maximum buoyancy force
    max_depth: f32,
    /// The volume of the object
    volume: f32,
    /// The height of the water plane above y = 0. The place will be parallel to the XZ plane.
    water_height: f32,
    /// The density of the liquid. Pure water has a density of 1000 kg per cubic meter
    liquid_density: f32,
}

impl ParticleBuoyancy {
    fn new(
        max_depth: f32,
        volume: f32,
        water_height: f32,
        liquid_density: f32, // default is intended to be 1000.0
    ) -> ParticleBuoyancy {
        return ParticleBuoyancy {
            max_depth,
            volume,
            water_height,
            liquid_density
        }
    }
}

impl ParticleForceGenerator for ParticleBuoyancy {
    /// Assuming that the buoyancy is acting in the up direction
    /// Default density is 1000.0 kgm^3
    /// Ocean water has a density of 1020 to 1030 kgm^3 up to 1250 kgm^3 for the Dead Sea
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        // Calculate the submersion depth
        let depth: f32 = particle.get_position().y;
        
        // Check if we're out of the water.
        if depth >= self.water_height + self.max_depth { return; }
        let mut force: Vector3 = Vector3::default();        
        
        // Check if we're at maximum depth.
        if depth <= self.water_height - self.max_depth {
            force.y = self.liquid_density * self.volume;
            particle.add_force(force);
            return;
        }
        
        // Otherwise we are partly submerged.
        force.y = self.liquid_density * self.volume *
            (depth - self.max_depth - self.water_height) / 2.0
            * self.max_depth;
        particle.add_force(force);
    }
}

pub struct ParticleFakeSpring {
    /// Location of the anchored end of the spring
    anchor: Vector3,
    /// Holds the spring constant
    spring_constant: f32,
    /// Holds the damping on the oscillation of the spring
    damping: f32,
}

impl ParticleFakeSpring {
    fn new(
        anchor: Vector3,
        spring_constant: f32,
        damping: f32
    ) -> ParticleFakeSpring {
        return ParticleFakeSpring {
            anchor,
            spring_constant,
            damping
       };
    }
}

impl ParticleForceGenerator for ParticleFakeSpring {
    fn update_force(&mut self, particle: &mut Particle, duration: f32) {
        // Check that we do not have infinite mass
        if !particle.has_finite_mass() { return; }
        
        // Calculate the relative position of the particle to the anchor
        let position: Vector3 = particle.get_position() - &self.anchor;
        
        // Calculate the constants and check that they are in bounds
        let gamma: f32 = 0.5 * (4.0 * self.spring_constant - self.damping * self.damping).sqrt();
        if gamma == 0.0 { return; }
        let c: Vector3 = position * (self.damping / (2.0 * gamma)) +
            &(particle.get_velocity() * (1.0 / gamma));
        
        // Calculate the target postion
        let mut target: Vector3 = position * (gamma * duration).cos() + 
            &(c * (gamma * duration).sin());
        target *= (-0.5 * duration * self.damping).exp();
        
        // Calculate the resulting acceleration, and therefore the force
        let acceleration: Vector3 = (target - &position) * (1.0f32 / duration * duration) -
            &(particle.get_velocity() * duration);
        particle.add_force(acceleration * particle.get_mass());
    }
}
