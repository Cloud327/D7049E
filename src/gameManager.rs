extern crate kiss3d;
extern crate nalgebra as na;

use std::path::Path;
use crate::ECS::attackRateComponent::AttackRateComponent;
use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, healthComponent::HealthComponent, idComponent::IdComponent, eventEnum::EventEnum, 
    typeEnum::TypeEnum, typeComponent::TypeComponent, attackDamageComponent::AttackDamageComponent, renderableComponent::RenderableComponent, transformComponent::TransformComponent};
use kiss3d::resource::MeshManager;
use kiss3d::scene::SceneNode;
use kiss3d::window::{Canvas, CanvasSetup, NumSamples};
use kiss3d::{window::Window, event::Action};
use kiss3d::light::Light;
use kiss3d::event::{Key, MouseButton, WindowEvent};
use na::{Vector3, Translation3};

use std::{ops::Deref, borrow::Borrow, sync::{RwLockReadGuard, LockResult}};
use ::nalgebra::{Point3, OPoint, Matrix3};

use rapier3d::na as nalgebra;
use rapier3d::prelude::*;
use na::Matrix4;



pub struct GameManager{
    entityManager: EntityManager,
    eventManager: EventManager,
    window: Window,
}


impl GameManager{
    pub fn new() -> Self {
        Self {
            entityManager: EntityManager::new(),
            eventManager: EventManager::new(),
            window: Window::new("Game"),
        }
    }

    pub fn initialize(){
        let path1 = Path::new("src/resources/mushroom.obj");
        let path2 = Path::new("src/resources/mushroom.mtl");
        
    }

    pub fn gameloop(&mut self){

        /* Run the game loop, stepping the simulation once per frame. */

        let path1 = Path::new("src/resources/mushroom/mushroom.obj");
        let path2 = Path::new("src/resources/mushroom/mushroom.mtl");
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();


        let mut meshManager = MeshManager::new();
        let mut objNames: Vec<String> = Vec::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.0, 100.0).build();
        collider_set.insert(collider);
        let mut sn = self.window.add_cube(100.0, 0.0, 100.0);
        sn.set_color(0.9, 0.6, 0.7);
        //sn.set_lines_color(Point::new(1.0, 1.0, 1.0));
        /* Create the bounding ball. */
        let rigid_body = RigidBodyBuilder::new_dynamic()
                .translation(vector![0.0, 30.0, 0.0])
                .build();
        //let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();

        let mut points:Vec<Point<Real>>;
        points = Vec::new();
        let objects = MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            let tri = mesh.borrow_mut().to_trimesh().unwrap();
            let m = mesh.borrow_mut().coords().read().unwrap().data().clone().unwrap();
            // mesh.borrow_mut().coords().read().unwrap().data().clone().unwrap()
            // m.coords().read().unwrap().data().unwrap().iter_mut()
            for point in m.into_iter(){
                points.push(Point::new(point[0], point[1], point[2]));
                
            }
            meshManager.add(mesh, &name[..]);
            objNames.push(name[..].to_string());
            let scale = Matrix3::new(1.0, 0.0, 0.0,
                                                                    0.0, 1.0, 0.0,
                                                                    0.0, 0.0, 1.0);
            //self.window.add_trimesh(tri, scale);

            
        });

        let collider = ColliderBuilder::convex_hull(&points).unwrap().restitution(0.7).build();
        let mushroom_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, mushroom_body_handle, &mut rigid_body_set);

        /* Create other structures necessary for the simulation. */
        //let gravity = vector![0.0, -9.81, 0.0];
        let gravity = vector![0.0, -1.62, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = JointSet::new();
        //let mut multibody_joint_set = JointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let physics_hooks = ();
        let event_handler = ();

        self.window.set_light(Light::StickToCamera);




        let mut nodes: Vec<SceneNode> = Vec::new();

        for name in objNames{
            nodes.push(self.window.add_mesh(meshManager.get(&name).unwrap(), Vector3::new(1.0, 1.0, 1.0)));
        }

        while true {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                //&mut multibody_joint_set,
                &mut ccd_solver,
                &physics_hooks,
                &event_handler,
            );

            let ball_body = &rigid_body_set[mushroom_body_handle];


            println!("{}", ball_body.translation().y);

            for node in nodes.iter_mut(){
                let t = ball_body.translation();
                let temp = Translation3::new(t[0], t[1], t[2]);
                node.set_local_translation(temp);
            }
            //println!("{}", nodes[0].data().local_translation());

            
            let escape = self.window.get_key(Key::Escape);
            if matches!(escape, Action::Press){
                break;
            }   

            self.window.render();
        }
    }

    fn eventloop(&mut self){
        let event = self.eventManager.readEvent();

        // Makes the object 
        if let EventEnum::takeDamageEvent{id, damage} = event {
            let mut healthCompList = self.entityManager.borrowComponentVecMut::<HealthComponent>().unwrap();
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = healthCompList.iter_mut().zip(idCompList.iter_mut());

            let iter = zip.filter_map(|(healthComp, idComp)| Some((healthComp.as_mut()?, idComp.as_mut()?)));
            for (healthComp, idComp) in iter {
                if idComp.getId() == id {
                    println!("health {} at id: {}", healthComp.getHealth(), id);
                    healthComp.decreaseHealth(damage);
                    println!("health {} at id: {}", healthComp.getHealth(), id);
                } else {
                    println!("No such id");
                }
            }
        }

        // Do attack with all object of type = towers
        // Do attack = create projectile object with heading towards x, y
        if let EventEnum::towerAttackEvent{x, y, z} = event {
            let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = typeCompList.iter_mut().zip(idCompList.iter_mut());

            let iter = zip.filter_map(|(typeComp, idComp)| Some((typeComp.as_mut()?, idComp.as_mut()?)));
            for (typeComp, idComp) in iter {
                if matches!(typeComp.getType(), TypeEnum::towerType{}){
                    println!("found tower at id: {}", idComp.getId());
                    // TODO: Do attack
                }
            }
        }


        // Create the necessary components for a tower
        if let EventEnum::spawnTowerEvent{x, y, z} = event {
            let tower = self.entityManager.newObject();
            self.entityManager.addComponentToObject(tower, TypeComponent::new(TypeEnum::towerType { }));
            self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(10));
            self.entityManager.addComponentToObject(tower, AttackRateComponent::new(1));
            // TODO: Create renderable component
            
        }

        // All events here
    }
}


pub fn test(){

    let mut gm = GameManager::new();

    let redEnemy = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(redEnemy, HealthComponent::new(65));
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(redEnemy, IdComponent::new(redEnemy));
    gm.entityManager.addComponentToObject(redEnemy, TypeComponent::new(TypeEnum::enemyType { }));
    //gm.entityManager.addComponentToObject(redEnemy, RenderableComponent::new());


    let whiteTower = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(whiteTower, AttackDamageComponent::new(8));
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(whiteTower, IdComponent::new(whiteTower));
    gm.entityManager.addComponentToObject(whiteTower, TypeComponent::new(TypeEnum::towerType { }));


    let blueEnemy = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(blueEnemy, HealthComponent::new(90));
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(blueEnemy, IdComponent::new(blueEnemy));
    gm.entityManager.addComponentToObject(blueEnemy, TypeComponent::new(TypeEnum::enemyType { }));


    gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20, z: 2});
    gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    

    gm.eventloop();
    gm.eventloop();
}