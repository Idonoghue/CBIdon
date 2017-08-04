
use glium;

pub use descartes::{N, P3, P2, V3, V4, M4, Iso3, Persp3, ToHomogeneous, Norm, Into2d, Into3d,
                    WithUniqueOrthogonal, Inverse, Rotate};

use glium::Surface;
use glium::backend::glutin_backend::GlutinFacade;
use kay::External;

use {Batch, Scene};

pub struct RenderContext {
    pub window: External<GlutinFacade>,
    batch_program: glium::Program,
}

impl RenderContext {
    #[allow(redundant_closure)]
    pub fn new(window: External<GlutinFacade>) -> RenderContext {
        RenderContext {
            batch_program: program!(&*window, 140 => {
                vertex: include_str!("shader/solid_140.glslv"),
                fragment: include_str!("shader/solid_140.glslf")
            }).unwrap(),
            window: window.steal(),
        }
    }

    pub fn submit<S: Surface>(&self, scene: &Scene, target: &mut S) {
        let view: [[f32; 4]; 4] =
            *Iso3::look_at_rh(&scene.eye.position, &scene.eye.target, &scene.eye.up)
                .to_homogeneous()
                .as_ref();
        let perspective: [[f32; 4]; 4] = *Persp3::new(
            target.get_dimensions().0 as f32 /
                target.get_dimensions().1 as f32,
            scene.eye.field_of_view,
            0.1,
            50000.0,
        ).to_matrix()
            .as_ref();

        let uniforms =
            uniform! {
            view: view,
            perspective: perspective
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let decal_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: false,
                ..Default::default()
            },
            ..Default::default()
        };

        // draw a frame
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        let mut render_debug_text = String::from("Renderer:\n");

        let mut batches_todo = scene.batches.iter().collect::<Vec<_>>();
        batches_todo.sort_by_key(|&(batch_id, _)| batch_id);

        for (i,
             &Batch {
                 ref vertices,
                 ref indices,
                 ref instances,
                 is_decal,
                 ..
             }) in batches_todo
        {
            if instances.len() > 1 {
                render_debug_text.push_str(&format!("batch{}: {} instances\n", i, instances.len()));
            }
            let instance_buffer = glium::VertexBuffer::new(&*self.window, instances).unwrap();
            target
                .draw(
                    (vertices, instance_buffer.per_instance().unwrap()),
                    indices,
                    &self.batch_program,
                    &uniforms,
                    if is_decal { &decal_params } else { &params },
                )
                .unwrap();
        }

        // let size_points = self.window.get_window().unwrap().get_inner_size_points().unwrap();
        // let size_pixels = self.window.get_window().unwrap().get_inner_size_pixels().unwrap();
        // let ui = self.imgui.frame(size_points, size_pixels, 1.0 / 60.0);

        // ui.window(im_str!("Debug Info"))
        //     .size((600.0, 200.0), ImGuiSetCond_FirstUseEver)
        //     .build(|| for (key, &(ref text, ref color)) in
        //         scene.persistent_debug_text.iter().chain(scene.debug_text.iter()) {
        //         ui.text_colored(*color, im_str!("{}:\n{}", key, text));
        //     });

        // self.imgui_renderer.render(target, ui).unwrap();
    }
}
