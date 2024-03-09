// main.rs

// 구조체(Struct)
struct StructName {
    field1: i32,
    field2: String,
}

// 열거형(Enum)
enum EnumName {
    Variant1,
    Variant2(i32),
    Variant3 { field: String },
}

// 특성(Trait)
trait TraitName {
    fn method_name(&self) -> String;
}

impl TraitName for StructName {
    fn method_name(&self) -> String {
        format!("field1: {}, field2: {}", self.field1, self.field2)
    }
}

// 제네릭(Generic)
struct GenericStruct<T> {
    field: T,
}

fn generic_function<T>(param: T) -> T {
    param
}

// Match 문
fn match_example(value: i32) -> String {
    match value {
        1 => "하나".to_string(),
        2 => "둘".to_string(),
        _ => "기타".to_string(),
    }
}

// 모듈 시스템(Modules)
mod my_module {
    pub fn public_function() {
        println!("접근 가능한 함수");
    }
}

// 특성 객체(Trait Objects)
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: i32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Circle with radius {}", self.radius);
    }
}

// 비동기 프로그래밍(Asynchronous Programming)
async fn fetch_data() -> Result<(), ()> {
    // 비동기 작업
    println!("Fetching data...");
    Ok(())
}

// 속성(Attribute) 활용
#[cfg(target_os = "linux")]
fn are_we_on_linux() {
    println!("리눅스에서 실행 중");
}

// 커스텀 타입 유도(Derive)
#[derive(Debug, Clone, Copy)]
struct MyStruct {
    field: i32,
}

fn main() {
    // 구조체 사용 예시
    let my_struct = StructName { field1: 10, field2: "Hello".to_string() };
    println!("구조체: {}", my_struct.method_name());

    // 열거형 사용 예시
    let my_enum = EnumName::Variant2(20);
    if let EnumName::Variant2(val) = my_enum {
        println!("열거형: Variant2({})", val);
    }

    // 제네릭 사용 예시
    let generic_struct = GenericStruct { field: "Generic Field" };
    println!("제네릭: {}", generic_struct.field);

    // Match 문 사용 예시
    println!("Match 문: {}", match_example(2));

    // 모듈 사용 예시
    my_module::public_function();

    // 특성 객체 사용 예시
    let circle = Circle { radius: 5 };
    let drawable_objects: Vec<Box<dyn Drawable>> = vec![Box::new(circle)];
    for object in drawable_objects {
        object.draw();
    }

    // 비동기 프로그래밍 사용 예시
    let future = fetch_data();
    futures::executor::block_on(future);

    // 속성 사용 예시 (특정 OS에서만 실행)
    #[cfg(target_os = "linux")]
    are_we_on_linux();

    // 커스텀 타입 유도 사용 예시
    let my_struct = MyStruct { field: 100 };
    println!("Derive: {:?}", my_struct);
}