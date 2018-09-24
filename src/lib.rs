
enum Either<T, U> {
    Left(T),
    Right(U),
}

struct MemoryPool<T> {
    data: Vec<Either<T, usize>>
}

impl<T> MemoryPool<T> {

    fn new() -> MemoryPool<T> {
        MemoryPool {
            data: vec![]
        }
    }

    fn push(item: T) -> usize {
        0
    }
}

#[test]
fn create_memory_pool() {
    let mp = MemoryPool::<i64>::new();
}
