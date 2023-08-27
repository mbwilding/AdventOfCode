#[derive(PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Default for Pos {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}

pub struct Tree {
    pub height: u8,
    pub visible: bool,
    pub trees_that_make_me_visible: Vec<Pos>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            height: 0,
            visible: false,
            trees_that_make_me_visible: vec![],
        }
    }
}

pub struct Forest {
    pub trees: Vec<Vec<Tree>>,
}

impl Forest {
    pub fn calculate_visible_trees(&mut self) -> usize {
        let mut visible_count = 0;

        for row in 0..self.trees.len() {
            for col in 0..self.trees[0].len() {
                let (is_visible, trees_making_visible) = self.check_visibility(row, col);
                if is_visible {
                    self.trees[row][col].visible = true;
                    self.trees[row][col].trees_that_make_me_visible = trees_making_visible;
                    visible_count += 1;
                } else {
                    self.trees[row][col].visible = false;
                }
            }
        }

        visible_count
    }

    fn check_visibility(&self, row: usize, col: usize) -> (bool, Vec<Pos>) {
        let mut trees_making_visible = Vec::new();
        let current_tree_height = self.trees[row][col].height;
        let mut top_visible = false;
        let mut bottom_visible = false;
        let mut left_visible = false;
        let mut right_visible = false;

        // If the tree is on the boundary, it is visible
        if row == 0 || row == self.trees.len() - 1 || col == 0 || col == self.trees[0].len() - 1 {
            return (true, trees_making_visible);
        }

        // Check from top
        for row in (0..row).rev() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if row == 0 {
                top_visible = true;
            }
        }

        // Check from bottom
        for row in row + 1..self.trees.len() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if row == self.trees.len() - 1 {
                bottom_visible = true;
            }
        }

        // Check from left
        for col in (0..col).rev() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if col == 0 {
                left_visible = true;
            }
        }

        // Check from right
        for col in col + 1..self.trees[0].len() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if col == self.trees[0].len() - 1 {
                right_visible = true;
            }
        }

        if top_visible || bottom_visible || left_visible || right_visible {
            (true, trees_making_visible)
        } else {
            (false, vec![])
        }
    }
}
