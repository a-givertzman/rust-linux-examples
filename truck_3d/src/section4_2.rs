mod app; // Load the dropped submodule
use app::*; // Use the trait app::App
use truck_platform::*;
use truck_rendimpl::*;
use truck_stepio::{r#in::{ruststep::{self, tables::EntityTable}, CartesianPointHolder, FaceAnyHolder, ShellHolder, Table}, out::StepModel};
use truck_topology::Solid;

use std::sync::Arc;
use winit::window::Window;

// Declare the application handler, a struct with a scene
struct MyApp {
    scene: WindowScene,
}

// Make MyApp an application handler by implementing App
#[async_trait(?Send)]
impl App for MyApp {
    // constructor
    async fn init(window: Arc<Window>) -> Self {
        // Use default setting except position and posture
        let mut camera: Camera = Camera::default();
        // specify position and posture
        camera.matrix = Matrix4::look_at_rh(
            // camera position
            Point3::new(5.0, 6.0, 5.0),
            // The camera looks to the center of the model.
            Point3::new(0.0, 1.5, 0.0),
            // the y-up coordinate
            Vector3::unit_y(),
        )
        // The matrix output from `look_at` needs to be inverted,
        // since cgmath uses the "self-centric" theory of moving the world with respect to the camera,
        // while truck uses the "world-centric" theory of moving the camera with respect to the world.
        .invert()
        .unwrap();

        // Use default setting except the position
        let mut light: Light = Light::default();
        // It is safe to place the camera in the same position as the flash.
        light.position = camera.position();

        // the setting of scene
        let scene_desc = WindowSceneDescriptor {
            studio: StudioConfig {
                // A scene has only one camera.
                camera,
                // The argument is `Vec` since a scene can have several lights.
                lights: vec![light],
                // There are the other options. Look later!
                ..Default::default()
            },
            ..Default::default()
        };

        // Create the scene
        let mut scene = WindowScene::from_window(window, &scene_desc).await;

        // Load the polygon from a wavefront obj file.
        let polygon: PolygonMesh = polymesh::obj::read(include_bytes!("teapot.obj").as_ref()).unwrap();

        let step_string = std::fs::read_to_string("truck_3d/src/as1-oc-214.stp").unwrap();
        // parse step file
        let exchange = ruststep::parser::parse(&step_string).unwrap();
        // convert the parsing results to a Rust struct
        let table = Table::from_data_section(&exchange.data[0]);
        // let shell = truck_topology::Shell::new();
        let shell = EntityTable::<ShellHolder>::get_owned(&table, 0).unwrap();
        println!("table | shell {:?}", table.shell.len());

        for face in shell.cfs_faces {
            match face {
                truck_stepio::r#in::FaceAny::FaceSurface(face_surface) => truck_topology::Shell face_surface,
                truck_stepio::r#in::FaceAny::OrientedFace(oriented_face) => todo!(),
            };
        }
        let instance: PolygonInstance = scene
            .instance_creator() // <- instance is created by instance creator.
            .create_instance(&shell, &Default::default());
        // Sign up the polygon to the scene.
        scene.add_object(&instance);

        // let mut faces = table.shell.into_iter().map(|(k, v)| {
        //     let f = truck_topology::Face::try_from(face).unwrap();
        //     shell.push(f);
        // }).collect();
        // let solid = Solid::new(&step_shell.cfs_faces);
        // let solid = Solid::from(step_shell.cfs_faces);
        
        // get `CartesianPoint` registered in #102
        // let step_point = EntityTable::<CartesianPointHolder>::get_owned(&table, 102).unwrap();
        // // convert `CartesianPoint` in STEP to `Point3` in cgmath
        // let cgmath_point = Point3::from(&step_point);

        // let solid: Solid = polymesh::obj::read(include_bytes!("teapot.obj").as_ref()).unwrap();
        // let compressed = polygon.compress();
        // // step format display
        // let display = CompleteStepDisplay::new(StepModel::from(&compressed), Default::default());
        // // content of step file
        // let step_string: String = display.to_string();
        // let step_path = filename.to_string() + ".step";
        // std::fs::write(&step_path, &step_string).unwrap();

        // Once the polygon data is in the form of an "instance".
        // This may seem wasteful to the beginning user, but this redundancy is useful for saving memory.
        let instance: PolygonInstance = scene
            .instance_creator() // <- instance is created by instance creator.
            .create_instance(&polygon, &Default::default());
        // Sign up the polygon to the scene.
        scene.add_object(&instance);

        // Return the application handler
        MyApp { scene }
    }

    // This method is called every frame.
    fn render(&mut self) {
        // scene draws a picture to the window.
        self.scene.render_frame();
    }
}

// Run!
fn main() { MyApp::run() }
