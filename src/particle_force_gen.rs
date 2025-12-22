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
