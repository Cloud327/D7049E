use std::{path::Path, ops::Deref, borrow::Borrow, sync::{RwLockReadGuard, LockResult}, rc::Rc};

use kiss3d::{window::{self, Window}, ncollide3d::math::Translation, event::{Action, Key}, light::Light, resource::{MeshManager, GPUVec, Texture}, scene::SceneNode};
use ::nalgebra::{Translation3, Vector3, Point3, OPoint, Matrix3};


use rapier3d::na::{self as nalgebra, ArrayStorage, Const};
use rapier3d::prelude::*;

use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, healthComponent::HealthComponent, idComponent::IdComponent, eventEnum::EventEnum, 
    typeEnum::TypeEnum, typeComponent::TypeComponent, attackDamageComponent::AttackDamageComponent, renderableComponent::RenderableComponent};


pub struct GameManager{
    entityManager: EntityManager,
    eventManager: EventManager,
    window: Window,
    gravity: rapier3d::na::Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: JointSet,
    //let mut multibody_joint_set = JointSet::new();
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    
    

}


impl GameManager{
    pub fn new() -> Self {
        Self {
            entityManager: EntityManager::new(),
            eventManager: EventManager::new(),
            window: Window::new("Game"),
            gravity: vector![0.0, -1.62, 0.0],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: JointSet::new(),
            //let mut multibody_joint_set = JointSet::new();
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            
        }
    }

    pub fn initialize(&mut self){
        let path1 = Path::new("src/resources/mushroom/mushroom.obj");
        let path2 = Path::new("src/resources/mushroom/mushroom.mtl");
        

        
        let mut meshManager = MeshManager::new();
        let mut objNames: Vec<String> = Vec::new();
        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.0, 100.0).build();
        self.collider_set.insert(collider);
        let mut sn = self.window.add_cube(100.0, 0.0, 100.0);
        sn.set_texture(Texture::new());
        //sn.set_color(0.9, 0.6, 0.7);
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
            let m = mesh.borrow_mut().coords().read().unwrap().data().clone().unwrap();
            for point in m.into_iter(){
                points.push(Point::new(point[0], point[1], point[2]));
                
            }
            meshManager.add(mesh, &name[..]);
            objNames.push(name[..].to_string());
        });
        
        let collider = ColliderBuilder::convex_hull(&points).unwrap().restitution(0.7).build();
        let mushroom_body_handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set.insert_with_parent(collider, mushroom_body_handle, &mut self.rigid_body_set);

        /* Create other structures necessary for the simulation. */
        //let gravity = vector![0.0, -9.81, 0.0];
        /*
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
        */

        self.window.set_light(Light::StickToCamera);
        
        
        

        let mut nodes: Vec<SceneNode> = Vec::new();
        
        for name in objNames{
            nodes.push(self.window.add_mesh(meshManager.get(&name).unwrap(), Vector3::new(1.0, 1.0, 1.0)));
        }
    }




    pub fn gameloop(&mut self){
        /* Run the game loop, stepping the simulation once per frame. */

    
        while true {
            self.physics_pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.island_manager,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.rigid_body_set,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                //&mut multibody_joint_set,
                &mut self.ccd_solver,
                &self.physics_hooks,
                &self.event_handler,
            );
    

            for rigid_body_handle in self.island_manager.active_dynamic_bodies() {
                let rigid_body = &self.rigid_body_set[*rigid_body_handle];
                rigid_body_handle.
            }
            let ball_body = &self.rigid_body_set[mushroom_body_handle];


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

            

            //while self.window.render() {}

            

    }

    fn eventloop(&mut self){
        let event = self.eventManager.readEvent();

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

        if let EventEnum::towerAttackEvent{x, y} = event {
            // Do attack with all object of type = towers
            // Do attack = create projectile object
            let mut typeCompList = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
            let mut idCompList = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = typeCompList.iter_mut().zip(idCompList.iter_mut());

            let iter = zip.filter_map(|(typeComp, idComp)| Some((typeComp.as_mut()?, idComp.as_mut()?)));
            for (typeComp, idComp) in iter {
                if matches!(typeComp.getType(), TypeEnum::towerType{}){
                    println!("found tower at id: {}", idComp.getId());
                    // Do attack
                }

            }
            println!("{}, {}", x, y);
        }
    }
}

pub fn test2(){
    let mut gm = GameManager::new();

    gm.gameloop();
}

pub fn test1(){

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


    gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20});
    gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    

    gm.eventloop();
    gm.eventloop();
}