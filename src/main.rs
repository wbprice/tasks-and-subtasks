#[derive(Debug, Copy, Clone)]
enum Activity {
    GoToStore,
    GoToWork,
    Play
}

#[derive(Debug, Copy, Clone)]
enum Subactivity {
    Step,
    Jump,
    Kick,
    Walk
}

#[derive(Debug, Clone)]
struct Task {
    activity: Activity,
    subtasks: Box<Vec<Subtask>>
}

impl Task {
    fn new(activity: Activity) -> Task {
        Task {
            activity,
            subtasks: Box::new(vec![])
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Subtask {
    subactivity: Subactivity
}

impl Subtask {
    fn new(subactivity: Subactivity) -> Subtask {
        Subtask {
            subactivity
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut task1 = Task::new(Activity::GoToWork);
    task1.subtasks.push(Subtask::new(Subactivity::Jump));
    task1.subtasks.push(Subtask::new(Subactivity::Jump));
    task1.subtasks.push(Subtask::new(Subactivity::Jump));


    let mut task2 = task1.clone();
    task2.subtasks.push(Subtask::new(Subactivity::Walk));
    task2.subtasks.push(Subtask::new(Subactivity::Walk));
    task2.subtasks.push(Subtask::new(Subactivity::Walk));

    dbg!(&task1);
    dbg!(&task2);

    assert_eq!(task1.subtasks.len(), 3); 
    assert_eq!(task2.subtasks.len(), 6); 
}
