# 后端

能够进行任务排程的 $事物$ 称为 TaskContainer。他们都表现出 TaskQueue 的 trait

任务是单独的个体，不能被标记完成，不能被排程，只有当任务被 taskContainer 包含时才可以被排程；

```rust
struct Task{
id:String,
init_priorty:usize,
complete_time:usize,
begin_date:Date,
end_date:Date,
}

trait TaskContainer{
fn peekTask(&self)->Task //考虑实现为iter
fn popTask(&mut self)->Task
fn priorty(&self)->usize
}
```

其中最上层实现为 SummarizeContaier,最下层实现为 OneTask:

```rust
static Disabled:usize = -1;

struct OnceTask{
current_priorty:usize,
task:String
}
```
