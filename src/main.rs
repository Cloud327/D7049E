use ECS::gameObject::{GameObject, self};
use ECS::transformComponent::{self, TransformComponent};
use ECS::healthComponent::{self, HealthComponent};
use kiss3d::nalgebra as na;
mod ECS;
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;
use na::{Point3, UnitQuaternion, Vector3, Matrix4};
use std::cell::RefCell;
use std::rc::Rc;

const TRANSFORMCOMPONENT_ID: i32= 327;




fn main() {
    let mut transformVector = Vec::new();
    
    let mut t1 = TransformComponent{
        id: 327,
                transform: Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                        0.0, 1.0, 0.0, 0.0,
                                        0.0, 0.0, 1.0, 0.0,
                                        0.0, 0.0, 0.0, 1.0)
    };

    let mut t2 = TransformComponent{
        id: 123,
                transform: Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                        0.0, 1.0, 0.0, 0.0,
                                        0.0, 0.0, 1.0, 0.0,
                                        0.0, 0.0, 0.0, 1.0)
    };

    let mut h1 = HealthComponent{
        health: 69,
    };
    let mut h2 = HealthComponent{
        health: 96,
    };
    
    transformVector.push(t1);
    transformVector.push(t2);

    let testObject1 = GameObject{
        components: vec![("TransformComponent", Box::new(t1)), ("HealthComponent", Box::new(h2))]

    };

    let testObject2 = GameObject{
        components: vec![("TransformComponent", Box::new(t2)), ("HealthComponent", Box::new(h1))]

    };

    let temp = testObject1.getComponent("TransformComponent");
    //println!("{}", temp.unwrap());
    
    
    /*let testObject = GameObject{
        components: vec![("TransformComponent", transformVector.len()-1)
        ]

    };

    

    let i = testObject.getComponentIndex("TransformComponent");

    println!("{}", transformVector[i.unwrap()].id)
    
    */

    

    /*let mut window = Window::new("Kiss3d: custom_mesh");

    let a = Point3::new(-1.0, -1.0, 0.0);
    let b = Point3::new(1.0, -1.0, 0.0);
    let c = Point3::new(0.0, 1.0, 0.0);

    let vertices = vec![a, b, c];
    let indices = vec![Point3::new(0u16, 1, 2)];

    let mesh =Rc::new(RefCell::new(Mesh::new(vertices, indices, None, None, false)));
    let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
    c.set_color(1.0, 0.0, 0.0);
    c.enable_backface_culling(false);

    window.set_light(Light::StickToCamera);

    //let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    let testObject = GameObject{
        components: vec![
            Box::new(TransformComponent{
                id: TRANSFORMCOMPONENT_ID,
                transform: Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                        0.0, 1.0, 0.0, 0.0,
                                        0.0, 0.0, 1.0, 0.0,
                                        0.0, 0.0, 0.0, 1.0)
            })
        ]

    };

    while window.render() {
        
       //c;
    }*/
}


