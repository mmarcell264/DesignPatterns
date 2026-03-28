trait TemplateMehod {
    fn tempalte_method(&self) {
        self.base_opreation1();
        self.reqired_operations1();
        self.base_opreation2();
        self.hook1();
        self.reqired_operations2();
        self.base_opreation3();
        self.hook2();
    }

    fn base_opreation1(&self) {
        println!("TemplateMethod says: I am doing the bulk of the work");
    }
    fn base_opreation2(&self) {
        println!("TemplateMethod says: But I let subclesses override some operations");
    }

    fn base_opreation3(&self) {
        println!("TemplateMethod says: I am doing the bulk of the work anyway");
    }

    fn hook1(&self) {}
    fn hook2(&self) {}

    fn reqired_operations1(&self);
    fn reqired_operations2(&self);
}

struct ConcrateStruct1;

impl TemplateMehod for ConcrateStruct1 {
    fn reqired_operations1(&self) {
        println!("ConcreteStruct1 says: Implemented Operation1");
    }

    fn reqired_operations2(&self) {
        println!("ConcreteStruct1 says: Implemented Operation2");
    }
}

struct ConcrateStruct2;

impl TemplateMehod for ConcrateStruct2 {
    fn reqired_operations1(&self) {
        println!("ConcreteStruct2 says: Implemented Operation2");
    }

    fn reqired_operations2(&self) {
        println!("ConcreteStruct2 says: Implemented Operation2");
    }
}

fn client_code(concrete: impl TemplateMehod) {
    concrete.tempalte_method();
}

pub fn template_main() {
    println!("Same client code can work with different concrete implementations:");
    client_code(ConcrateStruct1);
    println!();

    println!("Same client code can work with different concrete implementations:");
    client_code(ConcrateStruct2);
}
