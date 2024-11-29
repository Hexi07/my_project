use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk::update]
fn save_msg(msg: String) {
    MSG.with(|text| *text.borrow_mut() = msg);
}

#[ic_cdk::query]
fn get_msg() -> String {
    MSG.with(|text| text.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
