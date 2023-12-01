#[derive(PartialEq, Default)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

pub struct Tree {
    pub height: u8,
    pub scenic_score: u32,
    pub visible: bool,
    pub trees_in_range: Vec<Pos>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            height: 0,
            scenic_score: 0,
            visible: false,
            trees_in_range: vec![],
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
                let (is_visible, trees_making_visible, scenic_score) =
                    self.check_visibility(row, col);
                if is_visible {
                    self.trees[row][col].visible = true;
                    self.trees[row][col].trees_in_range = trees_making_visible;
                    self.trees[row][col].scenic_score = scenic_score;
                    visible_count += 1;
                } else {
                    self.trees[row][col].visible = false;
                }
            }
        }

        visible_count
    }

    pub fn calculate_max_scenic_score(&mut self) -> u32 {
        self.trees
            .iter()
            .map(|x| x.iter().map(|tree| tree.scenic_score).sum::<u32>())
            .sum()
    }

    // Removed optimisations, want the GUI to show all the trees, not just for visible ones
    fn check_visibility(&self, row: usize, col: usize) -> (bool, Vec<Pos>, u32) {
        let mut trees_in_range = Vec::new();
        let current_tree_height = self.trees[row][col].height;
        let mut top_visible = false;
        let mut bottom_visible = false;
        let mut left_visible = false;
        let mut right_visible = false;

        // If the tree is on the boundary
        let border =
            row == 0 || row == self.trees.len() - 1 || col == 0 || col == self.trees[0].len() - 1;

        let mut visibility_counts = [0; 4];

        // Check from top
        for row in (0..row).rev() {
            if self.trees[row][col].height < current_tree_height {
                trees_in_range.push(Pos { row, col });
                visibility_counts[0] += 1;
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
                trees_in_range.push(Pos { row, col });
                visibility_counts[1] += 1;
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
                trees_in_range.push(Pos { row, col });
                visibility_counts[2] += 1;
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
                trees_in_range.push(Pos { row, col });
                visibility_counts[3] += 1;
            } else {
                break;
            }
            if col == self.trees[0].len() - 1 {
                right_visible = true;
            }
        }

        let scenic_score: u32 = visibility_counts.iter().product();

        if border {
            return (true, trees_in_range, scenic_score);
        }

        if top_visible || bottom_visible || left_visible || right_visible {
            (true, trees_in_range, scenic_score)
        } else {
            (false, vec![], scenic_score)
        }
    }
}
