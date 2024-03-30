use crate::path::Path;

pub trait PathGenerator {
    type Item: Copy + PartialEq;

    fn paths(&self) -> Vec<Path<Self::Item>> {
        let mut open_paths = Vec::new();
        let mut closed_paths = Vec::new();
        open_paths.push(Path::new(self.start()));
        while let Some(open_path) = open_paths.pop() {
            for next_step in self.next(&open_path) {
                let paths = if self.close(&next_step) {
                    &mut closed_paths
                } else {
                    &mut open_paths
                };
                paths.push(open_path.extend(next_step));
            }
        }
        closed_paths
    }

    fn start(&self) -> Self::Item;
    fn next(&self, path: &Path<Self::Item>) -> Vec<Self::Item>;
    fn close(&self, next: &Self::Item) -> bool;
}
