fn main() {
    let app = App {
        inner: std::rc::Rc::new(AppInner {
            repo: std::rc::Rc::new(RepoImpl),
        }),
    };
    app.serv().act();
}

struct S;

impl S {
    fn say(&self) {
        println!("Hello");
    }
}

trait Repo {
    fn find(&self) -> S;
}

trait HasRepo {
    fn repo(&self) -> std::rc::Rc<dyn Repo>;
}

trait ServInner: HasRepo {
    fn act_impl(&self) {
        let repo = self.repo();
        let s = repo.find();
        s.say();
    }
}

trait Serv {
    fn act(&self);
}

trait HasServ {
    fn serv(&self) -> std::rc::Rc<dyn Serv>;
}

struct RepoImpl;

impl Repo for RepoImpl {
    fn find(&self) -> S {
        S
    }
}

struct AppInner {
    repo: std::rc::Rc<dyn Repo>,
}

impl HasRepo for AppInner {
    fn repo(&self) -> std::rc::Rc<dyn Repo> {
        std::rc::Rc::clone(&self.repo)
    }
}

impl ServInner for AppInner {}

impl Serv for AppInner {
    fn act(&self) {
        self.act_impl();
    }
}

struct App {
    inner: std::rc::Rc<dyn Serv>,
}

impl HasServ for App {
    fn serv(&self) -> std::rc::Rc<dyn Serv> {
        std::rc::Rc::clone(&self.inner)
    }
}
