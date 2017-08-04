
pub use descartes::{N, P3, P2, V3, V4, M4, Iso3, Persp3, ToHomogeneous, Norm, Into2d, Into3d,
                    WithUniqueOrthogonal, Inverse, Rotate};
use compact::CVec;
use kay::{Fate, World, ActorSystem, External};
use kay::swarm::Swarm;

use glium::backend::glutin_backend::GlutinFacade;

use {Batch, Instance, Scene, SceneDescription, Thing, RenderContext};

mod control;
pub mod movement;
mod project;

pub use self::control::{TargetProvider, TargetProviderID, MSG_TargetProvider_submitted};
pub use self::movement::{Movement, EyeListener, EyeListenerID, MSG_EyeListener_eye_moved};
pub use self::project::{ProjectionRequester, ProjectionRequesterID,
                        MSG_ProjectionRequester_projected_3d};

#[derive(Compact, Clone)]
pub struct Renderer {
    id: RendererID,
    inner: External<RendererState>,
}

pub struct RendererState {
    pub scenes: Vec<Scene>,
    pub render_context: RenderContext,
}

impl ::std::ops::Deref for Renderer {
    type Target = RendererState;

    fn deref(&self) -> &RendererState {
        &self.inner
    }
}

impl ::std::ops::DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut RendererState {
        &mut self.inner
    }
}

impl Renderer {
    pub fn spawn(
        id: RendererID,
        window: &External<GlutinFacade>,
        scenes: &CVec<SceneDescription>,
        world: &mut World,
    ) -> Renderer {
        id.setup(world);
        Renderer {
            id: id,
            inner: External::new(RendererState {
                scenes: scenes
                    .iter()
                    .map(|description| description.to_scene())
                    .collect(),
                render_context: RenderContext::new(window.clone()),
            }),
        }
    }
}

impl Renderer {
    /// Critical
    pub fn add_eye_listener(&mut self, scene_id: usize, listener: EyeListenerID, _: &mut World) {
        self.scenes[scene_id].eye_listeners.push(listener);
    }

    /// Critical
    pub fn add_batch(&mut self, scene_id: usize, batch_id: u16, thing: &Thing, _: &mut World) {
        let batch = Batch::new(thing.clone(), &self.render_context.window);
        self.scenes[scene_id].batches.insert(batch_id, batch);
    }

    /// Critical
    pub fn update_thing(
        &mut self,
        scene_id: usize,
        thing_id: u16,
        thing: &Thing,
        instance: &Instance,
        is_decal: bool,
        _: &mut World,
    ) {
        let thing = Batch::new_thing(
            thing.clone(),
            *instance,
            is_decal,
            &self.render_context.window,
        );
        self.scenes[scene_id].batches.insert(thing_id, thing);
    }

    /// Critical
    pub fn add_instance(
        &mut self,
        scene_id: usize,
        batch_id: u16,
        instance: Instance,
        _: &mut World,
    ) {
        self.scenes[scene_id]
            .batches
            .get_mut(&batch_id)
            .unwrap()
            .instances
            .push(instance);
    }

    /// Critical
    pub fn add_several_instances(
        &mut self,
        scene_id: usize,
        batch_id: u16,
        instances: &CVec<Instance>,
        _: &mut World,
    ) {
        self.scenes[scene_id]
            .batches
            .get_mut(&batch_id)
            .unwrap()
            .instances
            .extend_from_slice(instances);
    }
}

pub trait Renderable {
    fn setup_in_scene(&mut self, renderer_id: RendererID, scene_id: usize, world: &mut World);
    fn render_to_scene(&mut self, renderer_id: RendererID, scene_id: usize, world: &mut World);
}


pub fn setup(system: &mut ActorSystem) {
    system.add(Swarm::<Renderer>::new(), |_| {});
    auto_setup(system);
    control::auto_setup(system);
    movement::auto_setup(system);
    project::auto_setup(system);
}

mod kay_auto;
pub use self::kay_auto::*;
