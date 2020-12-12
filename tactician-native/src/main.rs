use druid::kurbo::{Circle, Line};
use druid::widget::Container;
use druid::{
    AppLauncher, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, PlatformError, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WindowDesc,
};
use tactician_core::nalgebra::Vector2;
use tactician_core::objects::{CelestialObject, Ship, Simulator};
use tactician_core::physics::PhysicsDetails;
mod conversion_ext;
use conversion_ext::MathVec2ToUiVec2;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())
}

fn ui_builder() -> impl Widget<()> {
    let sun = CelestialObject {
        phys: PhysicsDetails::new(1e15), // sun weighs a million billion kilos
        radius: 20.0,
    };

    let planet = CelestialObject {
        phys: PhysicsDetails {
            pos: Vector2::new(0.0, 250.0),
            mass: 1e14,
            velocity: Vector2::new(-40.0, 0.0),
        }, // planet weighs 1mil kilos
        radius: 10.0,
    };

    let planet2 = CelestialObject {
        phys: PhysicsDetails {
            pos: Vector2::new(-80.0, 320.0),
            mass: 1e5,
            velocity: Vector2::new(-20.0, -1.0),
        }, // planet weighs 1mil kilos
        radius: 5.0,
    };

    let ship = Ship {
        phys: PhysicsDetails {
            pos: Vector2::new(-70.0, 240.0),
            mass: 30.0,
            velocity: Vector2::new(-20.0, -20.0),
        }, // ship weighs 30 kilos
        current_accel: 0.0,
        max_accel: 3.0,
    };

    let simulator = Simulator {
        sun: sun,
        planets: vec![planet, planet2],
        ships: vec![ship],
    };
    // The label text will be computed dynamically based on the current locale and count
    Container::new(AnimWidget {
        simulator,
        ship_pos_buffer: vec![Vec::new(), Vec::new(), Vec::new()],
    })
    .background(Color::BLACK)
}

struct AnimWidget {
    simulator: Simulator,
    ship_pos_buffer: Vec<Vec<Vector2<f64>>>,
}

impl Widget<()> for AnimWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (), _env: &Env) {
        // println!("event!");
        match event {
            Event::MouseDown(_) => {
                // self.t = 0.0;
                ctx.request_anim_frame();
            }
            Event::AnimFrame(interval) => {
                ctx.request_paint();
                let interval_seconds = (*interval as f64) * 1e-9;
                println!("{} fps", 1.0/interval_seconds);
                self.simulator.update(interval_seconds * 2.0);
                for (i, planet) in self.simulator.planets.iter().enumerate() {
                    self.ship_pos_buffer.get_mut(i).unwrap().insert(0, planet.phys.pos);
                }
                for (j, ship) in self.simulator.ships.iter().enumerate() {
                    self.ship_pos_buffer.get_mut(self.simulator.planets.len() + j).unwrap().insert(0, ship.phys.pos);
                }
                ctx.request_anim_frame();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {
        println!("lifecycle!");
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {
        println!("update!");
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &(),
        _env: &Env,
    ) -> Size {
        println!("layout!");
        bc.constrain((100.0, 100.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        // println!("paint!");
        // draw trace
        for sub_buffer in self.ship_pos_buffer.iter() {
            let mut pos_iter = sub_buffer.iter().step_by(3);
            let maybe_prev = pos_iter.next();
            match maybe_prev {
                Some(prev) => {
                    let mut prev_pos = prev;
                    for cur_pos in pos_iter {
                        let past_pos_as_pt = prev_pos.convert(ctx.size().width, ctx.size().height);
                        let cur_pos_as_pt = cur_pos.convert(ctx.size().width, ctx.size().height);
                        ctx.stroke(
                            Line::new(past_pos_as_pt, cur_pos_as_pt),
                            &Color::grey8(200),
                            1.0,
                        );
                        prev_pos = cur_pos;
                    }
                }
                None => (),
            };
        }

        // Draw the sun!
        let sun_center = (&self.simulator.sun)
            .phys
            .pos
            .convert(ctx.size().width, ctx.size().height);
        ctx.fill(
            Circle::new(sun_center, (&self.simulator.sun).radius),
            &Color::WHITE,
        );

        // draw planets
        for planet in self.simulator.planets.iter() {
            let planet_center = (planet)
                .phys
                .pos
                .convert(ctx.size().width, ctx.size().height);
            ctx.fill(
                Circle::new(planet_center, planet.radius),
                &Color::grey8(200),
            );
        }

        // draw our little ship
        for ship in self.simulator.ships.iter() {
            let ship_center = ship.phys.pos.convert(ctx.size().width, ctx.size().height);
            ctx.fill(Circle::new(ship_center, 3.0), &Color::grey8(128));
        }
        // ctx.paint_with_z_index(1, move |ctx| {
        //     let ambit = center + 45.0 * Vec2::from_angle((0.75 + t) * 2.0 * 3.14159265);
        //     ctx.stroke(Line::new(center, ambit), &Color::BLACK, 1.0);
        // });
    }
}
