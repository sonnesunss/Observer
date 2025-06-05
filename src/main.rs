// ⚠️ 将行为与数据绑定是trait的核心要义，必须涉及到self参数
pub trait Subscriber {
    fn response(&self, value: i32);
}

pub trait Publisher {
    fn add_observer(&mut self, subscriber: Box<dyn Subscriber>);
    fn remove_observer(&mut self);
    fn notify(&self);
}

pub struct Observer;

#[allow(dead_code)]
pub struct Publish {
    observer_list: Vec<Box<dyn Subscriber>>,
    // 假定主题是一个变量字段，当它改变时即发出通知
    subject: i32,
}

impl Publish {
    pub fn new() -> Self {
        Publish {
            observer_list: Vec::new(),
            subject: 0,
        }
    }

    pub fn set_subject(&mut self, value: i32) {
        self.subject = value;
        self.notify();
    }
}

impl Publisher for Publish {
    fn notify(&self) {
        for obs in &self.observer_list {
            obs.response(self.subject);
        }
    }

    fn add_observer(&mut self, subscriber: Box<dyn Subscriber>) {
        self.observer_list.push(subscriber);
    }

    // 简化演示， 移除最后一个观察者
    fn remove_observer(&mut self) {
        self.observer_list.pop();
    }
}

impl Subscriber for Observer {
    fn response(&self, value: i32) {
        println!("Observer received new value: {}", value);
    }
}

fn main() {
    let mut publisher = Publish::new();
    let observer = Box::new(Observer);

    publisher.add_observer(observer);
    publisher.set_subject(42);
    publisher.set_subject(52);
}
