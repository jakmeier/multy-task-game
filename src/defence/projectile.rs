/*
<<< projectile >>
*/

//! 

use constants::*;
use super::collision::*;
use super::enemy::Enemy;

use piston_window::*;
use gfx_device_gl::Resources;
use gfx_device_gl::command::CommandBuffer;
use gfx_graphics::GfxGraphics;
use std::f64;

pub struct Projectile {
	x: f64, y: f64, 
	vx: f64, vy: f64, 
	impact: ProjectileImpact,
	power: f64,
	life_time: f64,
}

enum ProjectileImpact {
	SingleTarget,
	AoE{radius: f64},
}

impl Projectile {

//	CONSTRUCTORS	CONSTRUCTORS	CONSTRUCTORS	CONSTRUCTORS	CONSTRUCTORS

	// The most basic projectile, hitting one target only with the standard power
	pub fn new(x: f64, y: f64, dest_x: f64, dest_y: f64, power: f64, range: f64) -> Projectile {
		let distance = ((y-dest_y)*(y-dest_y) + (x-dest_x)*(x-dest_x)).sqrt();
		let vx = PROJECTILE_VELOCITY/ distance * (dest_x-x);
		let vy = PROJECTILE_VELOCITY/ distance * (dest_y-y);
		Projectile {
			x:x, y:y,
			vx:vx, vy:vy, 
			impact: ProjectileImpact::SingleTarget,
			power: power, 
			life_time: range / PROJECTILE_VELOCITY,
		}
	}
	pub fn new_aoe(x: f64, y: f64, dest_x: f64, dest_y: f64, power: f64, range: f64, r: f64) -> Projectile {
		let distance = ((y-dest_y)*(y-dest_y) + (x-dest_x)*(x-dest_x)).sqrt();
		let vx = PROJECTILE_VELOCITY/ distance * (dest_x-x);
		let vy = PROJECTILE_VELOCITY/ distance * (dest_y-y);
		Projectile {
			x:x, y:y,
			vx:vx, vy:vy, 
			impact: ProjectileImpact::AoE{radius: r},
			power: power, 
			life_time: range / PROJECTILE_VELOCITY,
		}
	}
	
//	METHODS		METHODS		METHODS		METHODS		METHODS		METHODS		METHODS	

	pub fn update (&mut self, dt: f64, enemies: &mut Vec<Box<Enemy>>) {
		self.life_time -= dt;
		let dx = self.vx * dt;
		let dy = self.vy * dt;
		if let Some(i) = enemies_with_segment(enemies, self.x, self.y, dx, dy){
			self.collide(&mut enemies[i]);
		}
		self.x += dx;
		self.y += dy;
	}
	
	fn collide (&mut self, enemy: &mut Box<Enemy>) {
		match self.impact {
			ProjectileImpact::SingleTarget => {
				enemy.attack_enemy(self.power);
				self.life_time = -1.0;
			},
			_ => {},
		}
	}
	
	pub fn is_dead(&self) -> bool { self.life_time < 0.0 }
	
	pub fn draw(	&self, 
					g: &mut GfxGraphics<Resources, CommandBuffer>, 
					view: math::Matrix2d,  
					dx: f64, dy: f64, 
					sprite_array: &[Texture<Resources>]) 
	{
		let sprite;
		let w; let h;
		match self.impact {
			ProjectileImpact::SingleTarget => {
				sprite = &sprite_array[0];
				w = PROJECTILE_SIZE.0;
				h = PROJECTILE_SIZE.1;
			}
			ProjectileImpact::AoE{..} => {
				sprite = &sprite_array[1];
				let (half_w, half_h) = PROJECTILE_SIZE;
				w = 2.0 * half_w;
				h = 2.0 * half_h;
			}
		}
		let (sprite_w, sprite_h) = sprite.get_size();
		let x_scale = w*dx/(sprite_w as f64);
		let y_scale = h*dy/(sprite_h as f64);
		let alpha = (self.vy/self.vx).atan() * 180.0 / f64::consts::PI;
		image(sprite, view.trans(self.x*dx,self.y*dy).scale(x_scale, y_scale).rot_deg(alpha), g);
	}
}
