use std::{path::Path, ops::Deref, borrow::Borrow, sync::{RwLockReadGuard, LockResult}, rc::Rc};

use kiss3d::{window::{self, Window}, ncollide3d::math::Translation, event::{Action, Key}, light::Light, resource::{MeshManager, GPUVec, Texture}, scene::SceneNode};
use ::nalgebra::{Translation3, Vector3, Point3, OPoint, Matrix3, Isometry3};
use crate::{mapManager::MapManager, ECS::rigidBodyComponent::RigidBodyComponent};
use crate::physicsManager::PhysicsManager;
use crate::nodeHandler::NodeHandler;
use rapier3d::prelude::*;
use crate::AttackRateComponent;
use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, healthComponent::HealthComponent, idComponent::IdComponent, eventEnum::EventEnum, 
    typeEnum::TypeEnum, typeComponent::TypeComponent, attackDamageComponent::AttackDamageComponent, renderableComponent::RenderableComponent};


pub struct GameManager{
    entityManager: EntityManager,
    eventManager: EventManager,
    mapManager: MapManager,
    window: Window,
    physicsManager: PhysicsManager,
    nodeHandler: NodeHandler,
    
    

}


impl GameManager{
    pub fn new() -> Self {
        Self {
            entityManager: EntityManager::new(),
            eventManager: EventManager::new(),
            mapManager: MapManager::new(),
            window: Window::new("Game"),
            physicsManager: PhysicsManager::new(),
            nodeHandler: NodeHandler::new(),

            
        }
    }


    


    pub fn initialize(&mut self){
        let path1 = Path::new("src/resources/mushroom/mushroom.obj");
        let path2 = Path::new("src/resources/mushroom/mushroom.mtl");
        self.nodeHandler.addNodes(TypeEnum::towerType, path1, path2);
        self.nodeHandler.addNodes(TypeEnum::enemyType, path1, path2);

        
        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.0, 100.0).build();
        self.physicsManager.addCollider(collider);
        self.window.add_cube(100.0, 0.0, 100.0);


        

       

        self.window.set_light(Light::StickToCamera);
        
        let mut nodes: Vec<SceneNode> = Vec::new();
        
        //for name in objNames{
        //    nodes.push(self.window.add_mesh(meshManager.get(&name).unwrap(), Vector3::new(1.0, 1.0, 1.0)));
        //}
    }

    /*fn spawnTower(&mut self, x: usize, y: usize, z: usize){
        let tower = self.entityManager.newObject();
        self.entityManager.addComponentToObject(tower, TypeEnum::towerType);
        self.entityManager.addComponentToObject(tower, AttackDamageComponent::new(10));
        self.entityManager.addComponentToObject(tower, AttackRateComponent::new(1));

        // Get a tuple 
        let towerNodes = self.nodeHandler.getNodes(TypeEnum::towerType).unwrap();

        let mut sceneNodes: Vec<SceneNode> = Vec::new();
        for name in towerNodes.1.clone(){
            let mesh = towerNodes.0.get(&name);
            let mut temp = self.window.add_mesh(mesh.unwrap(), Vector3::new(1.0, 1.0, 1.0));
            temp.set_local_translation(Translation3::new(x as f32, y as f32, z as f32));
            sceneNodes.push(temp);
        }
        self.entityManager.addComponentToObject(tower, RenderableComponent::new(sceneNodes))
    }*/

    /*fn placeInWorld(position: Translation3){
        /* Create the rigid body. */
        let rigidBody = RigidBodyBuilder::new_dynamic()
        .translation(vector![0.0, 30.0, 0.0])
        .build();
    }*/


    fn generate3Dobject(&mut self, obj_dir: &Path, mtl_dir: &Path)-> (MeshManager, Vec<String>, Vec<Point<Real>>){
        let mut meshManager = MeshManager::new();
        let mut objNames: Vec<String> = Vec::new();
        let mut points:Vec<Point<Real>>;
        points = Vec::new();
        let objects = MeshManager::load_obj(obj_dir, mtl_dir, "obj")
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

        return (meshManager, objNames, points);
        
        //let collider = ColliderBuilder::convex_hull(&points).unwrap().restitution(0.7).build();
        //let handle = self.physicsManager.addRigidBody(rigidBody);
        //self.physicsManager.addColliderWithParent(collider, handle);

    }
/* PSEUDOCODE

initialize(){
    loadMap()
    loadObjects()
}

loadMap(){
    mapManager.initialize()
}

mapmanager.initialize(){

}

loadObjects(){
    send
}


gameLoop(){
    while(True){
        physicsManager.step()

        for renderable in entityManager.componentVec[renderableComponents]{
            renderable.update()
            renderable.render()
        }

         doEvent()
    }

}
*/ 



    fn updateNodes(&mut self){

        let mut renderableList = self.entityManager.borrowComponentVecMut::<RenderableComponent>().unwrap();
            let mut rigidBodyList = self.entityManager.borrowComponentVecMut::<RigidBodyComponent>().unwrap();
            let zip = renderableList.iter_mut().zip(rigidBodyList.iter_mut());

            let iter = zip.filter_map(|(renderable, rigidBody)| Some((rigidBody.as_mut()?, renderable.as_mut()?)));
            for (rigidBody, renderable) in iter {
                for node in renderable.getSceneNodes(){
                    node.write().unwrap().set_local_translation(rigidBody.getTranslation());
                } 
            }
    }

    pub fn gameloop(&mut self){
        /* Run the game loop, stepping the simulation once per frame. */

    
        while true {
            self.physicsManager.step();


            self.updateNodes();


            



            //for rigid_body_handle in self.island_manager.active_dynamic_bodies() {
            //    let rigid_body = &self.rigid_body_set[*rigid_body_handle];
            //    rigid_body_handle.
            //}
            //let ball_body = &self.rigid_body_set[mushroom_body_handle];


            //println!("{}", ball_body.translation().y);

            //for node in nodes.iter_mut(){
            //    let t = ball_body.translation();
            //    let temp = Translation3::new(t[0], t[1], t[2]);
            //    node.set_local_translation(temp);
            //}
            //println!("{}", nodes[0].data().local_translation());

            
            let escape = self.window.get_key(Key::Escape);
            if matches!(escape, Action::Press){
                break;
            }   

            self.window.render();
            
        }

            

            //while self.window.render() {}

        

    }

    fn doEvent(&mut self){
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



        


        // All events here
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


    gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20, z:30});
    gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    

    gm.doEvent();
    gm.doEvent();
}