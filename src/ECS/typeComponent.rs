use super::typeEnum::TypeEnum;


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

    pub fn getType(self) -> TypeEnum{
        return self.objectType
    }


}