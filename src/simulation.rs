use crate::environment::Environment;
use crate::projectile::Projectile;
use crate::tuple::Tuple;

pub struct Simulation {
    environment: Environment,
    projectiles: Vec<Projectile>,
}

impl Simulation {
    pub fn new(environment: Environment, projectiles: Vec<Projectile>) -> Self {
        Simulation {
            environment,
            projectiles,
        }
    }

    pub fn tick(&mut self) {
        for projectile in &mut self.projectiles {
            projectile.vel = projectile.vel + self.environment.gravity + self.environment.wind;
            projectile.pos = projectile.pos + projectile.vel;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_tick_moves_projectile() {
        // Set up environment with gravity and wind
        let gravity = Tuple::vector(0.0, -0.1, 0.0); // Downward gravity
        let wind = Tuple::vector(-0.01, 0.0, 0.0); // Leftward wind
        let environment = Environment::new(gravity, wind);

        // Create a projectile with initial position and velocity
        let initial_pos = Tuple::point(0.0, 1.0, 0.0);
        let initial_vel = Tuple::vector(1.0, 1.0, 0.0);
        let projectile = Projectile::new(initial_pos, initial_vel);

        // Create simulation with one projectile
        let mut simulation = Simulation::new(environment, vec![projectile]);

        // Store initial position for comparison
        let initial_position = simulation.projectiles[0].pos;

        // Tick the simulation
        simulation.tick();

        // Verify the projectile has moved
        let final_position = simulation.projectiles[0].pos;

        // Position should have changed
        assert_ne!(initial_position.x, final_position.x);
        assert_ne!(initial_position.y, final_position.y);

        // Verify expected movement:
        // New velocity = initial_vel + gravity + wind = (1.0, 1.0, 0.0) + (0.0, -0.1, 0.0) + (-0.01, 0.0, 0.0)
        //              = (0.99, 0.9, 0.0)
        // New position = initial_pos + new_velocity = (0.0, 1.0, 0.0) + (0.99, 0.9, 0.0)
        //              = (0.99, 1.9, 0.0)
        assert_eq!(final_position.x, 0.99);
        assert_eq!(final_position.y, 1.9);
        assert_eq!(final_position.z, 0.0);
        assert_eq!(final_position.is_point(), true); // Should remain a point
    }

    #[test]
    fn observe_projectile_trajectory() {
        let gravity = Tuple::vector(0.0, -0.1, 0.0);
        let wind = Tuple::vector(-0.01, 0.0, 0.0);
        let environment = Environment::new(gravity, wind);

        let initial_pos = Tuple::point(0.0, 1.0, 0.0);
        let initial_vel = Tuple::vector(1.0, 1.0, 0.0);
        let projectile = Projectile::new(initial_pos, initial_vel);

        let mut simulation = Simulation::new(environment, vec![projectile]);

        println!("\n=== Projectile Trajectory ===");
        println!(
            "Initial: Position ({:.3}, {:.3}, {:.3})",
            simulation.projectiles[0].pos.x,
            simulation.projectiles[0].pos.y,
            simulation.projectiles[0].pos.z
        );

        for i in 1..=10 {
            simulation.tick();
            let pos = simulation.projectiles[0].pos;
            println!(
                "Tick {:2}: Position ({:6.3}, {:6.3}, {:6.3})",
                i, pos.x, pos.y, pos.z
            );
            let vel = simulation.projectiles[0].vel;
            println!(
                "         Velocity ({:6.3}, {:6.3}, {:6.3})",
                vel.x, vel.y, vel.z
            );
        }
        println!("=== End Trajectory ===\n");
    }
}
