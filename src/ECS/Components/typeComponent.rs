use crate::ECS::typeEnum::TypeEnum;


pub struct TypeComponent{
    objectType: TypeEnum
}

impl TypeComponent{
    pub fn new(objectType: TypeEnum)-> Self{
        Self{
            objectType: objectType
        }
    }

    pub fn update(){

    }

    pub fn getType(&self) -> TypeEnum{
        return self.objectType
    }

    pub fn getTypeString(&self, objectType: TypeEnum) -> &str{
        if matches!(objectType, TypeEnum::enemyType){
            return "Enemy";
        }
        else if matches!(objectType, TypeEnum::projectileType){
            return "Projectile";
        }
        else if matches!(objectType, TypeEnum::baseType){
            return "Base";
        }
        else if matches!(objectType, TypeEnum::wallType){
            return "Wall";
        }
        else{
            return "None?";
        }
    }


}