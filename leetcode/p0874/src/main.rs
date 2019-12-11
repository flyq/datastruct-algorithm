fn main() {
    println!("Hello, world!");
    let a = vec![2,-1,-1,-1,-2,1,7,-2,9,2,3,-1,4,9,7,7,2,4,2,-2,1,5,8,-2,-2,4,2,9,7,5,5,-2,2,2,1,-1,-1,1,6,6,-1,7,-1,7,1,8,2,-1,8,7,-1,2,-2,2,2,4,9,-1,4,-1,-2,8,-1,3,6,-2,7,-2,6,7,9,6,-2,-1,3,6,2,8,6,6,-2,-2,4,2,4,1,2,2,2,8,6,4,6,7,-1,1,-2,-1,-1,7];
    let b = vec![[75,61],[-27,-13],[-85,77],[-40,-30],[-71,-34],[41,-39],[73,-54],[-19,16],[76,50],[-12,-9],[-25,-100],[45,-86],[-43,-88],[50,-23],[-46,-89],[-66,91],[-57,-46],[-82,51],[78,98],[65,-61],[83,-14],[24,-17],[28,77],[-63,-3],[77,-39],[18,-63],[10,-91],[-11,-15],[-75,-80],[68,92],[21,-70],[91,-53],[-68,-64],[9,-68],[1,40],[-73,20],[56,15],[-90,-43],[-100,99],[-19,7],[14,76],[-80,-2],[24,-34],[47,7],[25,73],[-99,-39],[-55,-9],[85,31],[14,0],[-68,94],[-25,25],[44,-77],[-94,59],[92,-47],[40,5],[-68,-58],[87,39],[-43,93],[-83,-77],[-95,81],[82,37],[66,21],[-5,73],[-75,32],[30,70],[22,-68],[-27,31],[-91,80],[82,-58],[-95,-24],[15,22],[-10,38],[85,96],[68,26],[81,-18],[23,-47],[-80,-78],[30,18],[-56,4],[1,33],[-21,2],[-69,85],[41,92],[-72,79],[-48,-34],[-34,63],[48,-78],[17,73],[16,28],[-13,-14],[16,24],[11,-27],[44,52],[-78,67],[93,65],[-32,-33],[6,-2],[67,-100],[95,77],[-6,28],[10,81],[-45,48],[80,-34],[-49,46],[-38,17],[7,-81],[-29,52],[46,-82],[5,-71],[79,-87],[39,-82],[-78,-82],[-85,19],[74,-55],[22,45],[-40,-24],[44,97],[41,-21],[-17,-92],[17,49],[-1,-33],[39,-36],[37,-38],[41,-29],[72,-88],[-100,57],[-95,74],[-27,-16],[57,-34],[74,-85],[62,92],[44,0],[83,57],[90,96],[-65,70],[-58,99],[-70,-86],[75,-74],[-63,11],[-64,20],[-35,-40],[-86,-71],[-77,-62],[4,-95],[97,76],[36,-62],[-1,90],[99,91],[55,89],[80,77],[40,54],[79,-11],[44,-36],[-35,21],[-13,-86],[-55,84],[27,94],[74,91],[-77,-45],[-90,44],[-80,-35],[-38,80],[34,-28],[45,-77],[1,28],[-88,-50],[-100,87],[19,93],[-26,-39],[-83,-100],[-6,43],[89,42],[-35,-95],[-67,-96],[14,22],[-25,8],[-31,-9],[-94,48],[82,-3],[39,95],[44,47],[-62,-71],[73,-30],[92,-11],[2,85],[-91,97],[99,-18],[-57,-17],[57,73],[-41,9],[44,9],[17,-96],[-95,-16],[40,-3],[-48,-41],[95,18],[-34,-94],[15,-90],[42,11],[-65,-57]];
    let mut c = vec![];
    for i in 0..b.len() {
        let mut  temp = vec![];
        temp.extend_from_slice(&b[i]);
        c.push(temp);
    }

    println!("{}", Solution2::robot_sim(a, c));
    
        
}
pub struct Solution{}
pub struct Solution2{}

#[derive(PartialEq)]
pub enum Dir {
    N,
    E,
    S,
    W,
}
#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut dir = Dir::N;
        let mut p = Point {
            x: 0,
            y: 0,
        };
        for i in  0..commands.len() {
            if commands[i] < 0 {
                dir = Solution::direct_change(dir, commands[i]);
                continue;
            } else if commands[i] == 0 {
                continue;
            } else {
                match dir {
                    Dir::N => {
                        let end = Point {
                            x: p.x,
                            y: p.y + commands[i],
                        };
                        let temp = Solution::have_obstacle(p, end, Dir::N, &obstacles);
                        match temp {
                            None => p.y += commands[i],
                            _ => p = temp.unwrap(),
                        }
                    }
                    Dir::S => {
                        let end = Point {
                            x: p.x,
                            y: p.y - commands[i],
                        };
                        let temp = Solution::have_obstacle(p, end, Dir::S, &obstacles);
                        match temp {
                            None => p.y -= commands[i],
                            _ => p =  temp.unwrap(),
                        }
                    }
                    Dir::E => {
                        let end = Point {
                            x: p.x + commands[i],
                            y: p.y,
                        };
                        let temp = Solution::have_obstacle(p, end, Dir::E, &obstacles);
                        match temp {
                            None => p.x += commands[i],
                            _ =>  p = temp.unwrap(),
                        }
                    }
                    Dir::W => {
                        let end = Point {
                            x: p.x - commands[i],
                            y: p.y,
                        };
                        let temp = Solution::have_obstacle(p, end, Dir::W, &obstacles);
                        match temp {
                            None => p.x -= commands[i],
                            _ => p = temp.unwrap(),
                        }
                    }
                }
                let temp = p.x.pow(2) + p.y.pow(2);
                if temp > max {
                    max = temp;
                }
            }
        }
        max
    }

    pub fn direct_change(now: Dir, change: i32) -> Dir {
        match now {
            Dir::N => {
                if change == -1 {
                    return Dir::E;
                } else {
                    return Dir::W;
                }
            }
            Dir::E => {
                if change == -1 {
                    return Dir::S;
                } else {
                    return Dir::N;
                }
            }
            Dir::S => {
                if change == -1 {
                    return Dir::W;
                } else {
                    return Dir::E;
                }
            }
            Dir::W => {
                if change == -1 {
                    return Dir::N;
                } else {
                    return Dir::S;
                }
            }
        }
    }

    pub fn have_obstacle(start: Point, end: Point, dir: Dir, obs: &Vec<Vec<i32>>) -> Option<Point> {
        if start.x == end.x && (dir == Dir::N || dir == Dir::S) {
            let same_x = start.x;
            if start.y > end.y && dir == Dir::S {
                let mut max_y = end.y - 1;
                for i in 0..obs.len() {
                    if obs[i][0] == same_x && obs[i][1] >= end.y && obs[i][1] < start.y && obs[i][1] > max_y {
                        max_y = obs[i][1];
                    }
                }
                if max_y != end.y - 1 {
                    let temp = Point {
                        x: same_x,
                        y: max_y+1,
                    };
                    return Some(temp);
                }
            } else if start.y < end.y && dir == Dir::N {
                let mut min_y = end.y + 1;
                for i in 0..obs.len() {
                    if obs[i][0] == same_x && obs[i][1] <= end.y && obs[i][1] > start.y && obs[i][1] < min_y {
                        min_y = obs[i][1];
                    }
                }
                if min_y != end.y + 1 {
                    let temp = Point {
                        x: same_x,
                        y: min_y - 1,
                    };                    
                    return Some(temp);
                }
            }            
        } else if start.y == end.y && (dir == Dir::E || dir == Dir::W) {
            let same_y =  start.y;
            if start.x > end.x && dir == Dir::W {
                let mut max_x = end.x - 1;
                for i in 0..obs.len() {
                    if obs[i][1] == same_y && obs[i][0] >= end.x && obs[i][0] < start.x && obs[i][0] > max_x {
                        max_x = obs[i][0];
                    }
                }
                if max_x != end.x - 1 {
                    let temp = Point {
                        x: max_x + 1,
                        y: same_y,
                    };
                    return Some(temp);
                }
            } else if start.x < end.x && dir == Dir::E {
                let mut min_x = end.x + 1;
                for i in 0..obs.len() {
                    if obs[i][1] == same_y && obs[i][0] <= end.x && obs[i][0] > start.x && obs[i][0] < min_x {
                        min_x = obs[i][0];
                    }
                }
                if min_x != end.x + 1 {
                    let temp = Point {
                        x: min_x - 1,
                        y: same_y,
                    };
                    return Some(temp);
                }
            }
        }
        None
    }
}


/*
执行结果：
通过
显示详情
执行用时 :
352 ms
, 在所有 rust 提交中击败了
50.00%
的用户
内存消耗 :
2.7 MB
, 在所有 rust 提交中击败了
100.00%
的用户

检测每个一障碍物是否在这条线上
*/





use std::collections::HashSet;
impl Solution2 {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut dir = Dir::N;
        let mut p = Point {
            x: 0,
            y: 0,
        };
        let mut set = HashSet::new();
        for i in 0..obstacles.len() {
            set.insert(obstacles[i].clone());
        }

        for i in  0..commands.len() {
            if commands[i] < 0 {
                dir = Solution2::direct_change(dir, commands[i]);
                continue;
            } else if commands[i] == 0 {
                continue;
            } else {
                match dir {
                    Dir::N => {
                        let end = Point {
                            x: p.x,
                            y: p.y + commands[i],
                        };
                        let temp = Solution2::have_obstacle(p, end, Dir::N, &set);
                        match temp {
                            None => p.y += commands[i],
                            _ => p = temp.unwrap(),
                        }
                    }
                    Dir::S => {
                        let end = Point {
                            x: p.x,
                            y: p.y - commands[i],
                        };
                        let temp = Solution2::have_obstacle(p, end, Dir::S, &set);
                        match temp {
                            None => p.y -= commands[i],
                            _ => p =  temp.unwrap(),
                        }
                    }
                    Dir::E => {
                        let end = Point {
                            x: p.x + commands[i],
                            y: p.y,
                        };
                        let temp = Solution2::have_obstacle(p, end, Dir::E, &set);
                        match temp {
                            None => p.x += commands[i],
                            _ =>  p = temp.unwrap(),
                        }
                    }
                    Dir::W => {
                        let end = Point {
                            x: p.x - commands[i],
                            y: p.y,
                        };
                        let temp = Solution2::have_obstacle(p, end, Dir::W, &set);
                        match temp {
                            None => p.x -= commands[i],
                            _ => p = temp.unwrap(),
                        }
                    }
                }
                let temp = p.x.pow(2) + p.y.pow(2);
                if temp > max {
                    max = temp;
                }
            }
        }
        max
    }

    pub fn direct_change(now: Dir, change: i32) -> Dir {
        match now {
            Dir::N => {
                if change == -1 {
                    return Dir::E;
                } else {
                    return Dir::W;
                }
            }
            Dir::E => {
                if change == -1 {
                    return Dir::S;
                } else {
                    return Dir::N;
                }
            }
            Dir::S => {
                if change == -1 {
                    return Dir::W;
                } else {
                    return Dir::E;
                }
            }
            Dir::W => {
                if change == -1 {
                    return Dir::N;
                } else {
                    return Dir::S;
                }
            }
        }
    }

    pub fn have_obstacle(start: Point, end: Point, dir: Dir, obs: &HashSet<Vec<i32>>) -> Option<Point> {
        match dir {
            Dir::N => {
                for i in start.y+1..end.y+1 {
                    let temp = vec![start.x, i];
                    if obs.contains(&temp) {
                        let res = Point {
                            x: start.x,
                            y: i - 1,
                        };
                        return Some(res);
                    }
                }
            }
            Dir::S => {
                for i in (end.y..start.y).rev() {
                    let temp = vec![start.x, i];
                    if obs.contains(&temp) {
                        let res = Point {
                            x: start.x,
                            y: i + 1,
                        };
                        return Some(res);
                    }
                }
            }
            Dir::E => {
                for i in start.x+1..end.x+1 {
                    let temp = vec![i, start.y];
                    if obs.contains(&temp) {
                        let res = Point {
                            x: i - 1,
                            y: start.y
                        };
                        return Some(res);
                    }
                }
            }
            Dir::W => {
                for i in (end.x..start.x).rev() {
                    let temp = vec![i, start.y];
                    if obs.contains(&temp) {
                        let res = Point {
                            x: i + 1,
                            y: start.y
                        };
                        return Some(res);
                    }
                }
            }
        }
        None        
    }
}

/* 

执行结果：
通过
显示详情
执行用时 :
24 ms
, 在所有 rust 提交中击败了
50.00%
的用户
内存消耗 :
3.7 MB
, 在所有 rust 提交中击败了
100.00%
的用户

检测每个一障碍物是否在这条线上
*/
