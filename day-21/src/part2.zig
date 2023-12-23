const std = @import("std");

const Tile = enum {
    Garden,
    Rock,
    Start,
};

const Direction = enum {
    Up,
    Down,
    Left,
    Right,
};

const Pos = struct {
    x: usize,
    y: usize,
};

const MoveCache = struct {
    step: usize,
    x: usize,
    y: usize,
};

const Grid = struct {
    slice: []Tile,
    width: usize,
    height: usize,
    allocator: std.mem.Allocator,

    fn deinit(self: Grid) void {
        self.allocator.free(self.slice);
    }

    fn from_string(allocator: std.mem.Allocator, input: []const u8) !Grid {
        var grid: Grid = undefined;

        grid.allocator = allocator;
        grid.width = std.ascii.indexOfIgnoreCase(input, "\n") orelse unreachable;
        grid.height = input.len / grid.width;
        grid.slice = try allocator.alloc(Tile, grid.width * grid.height);

        var skipped: usize = 0;
        for (input, 0..) |c, i| {
            grid.slice[i - skipped] = switch (c) {
                '.' => .Garden,
                '#' => .Rock,
                'S' => .Start,
                else => {
                    skipped += 1;
                    continue;
                },
            };
        }

        return grid;
    }

    fn get(self: Grid, x: usize, y: usize) ?Tile {
        if (x >= self.width or y >= self.height) {
            return null;
        }
        return self.slice[x + y * self.width];
    }

    fn walk(self: Grid, x: usize, y: usize, remaining_steps: usize, visited: *std.AutoHashMap(MoveCache, void), end_pos: *std.AutoHashMap(Pos, void)) !void {
        // std.debug.print("Step: {d}, ({d}, {d})\n", .{ remaining_steps, x, y });
        if (remaining_steps == 0) {
            try end_pos.put(Pos{ .x = x, .y = y }, {});
            return;
        }
        var next_x: usize = x;
        var next_y: usize = y;
        for (&[_]Direction{ .Up, .Down, .Left, .Right }) |direction| {
            switch (direction) {
                .Up => {
                    if (y == 0) continue;
                    next_x = x;
                    next_y = y - 1;
                },
                .Down => {
                    if (y >= self.height) continue;
                    next_x = x;
                    next_y = y + 1;
                },
                .Left => {
                    if (x == 0) continue;
                    next_x = x - 1;
                    next_y = y;
                },
                .Right => {
                    if (x >= self.width) continue;
                    next_x = x + 1;
                    next_y = y;
                },
            }

            const tile = self.get(next_x, next_y);
            if (tile != null and tile != .Rock and try visited.fetchPut(MoveCache{ .x = next_x, .y = next_y, .step = remaining_steps }, {}) == null) {
                try self.walk(next_x, next_y, remaining_steps - 1, visited, end_pos);
            }
        }
    }
};

fn process(allocator: std.mem.Allocator, input: []const u8, steps: usize) !usize {
    const grid = try Grid.from_string(allocator, input);
    defer grid.deinit();

    var start_x: usize = undefined;
    var start_y: usize = undefined;
    for (0..grid.height) |y| {
        for (0..grid.width) |x| {
            if (grid.get(x, y) == .Start) {
                start_x = x;
                start_y = y;
                break;
            }
        }
    }

    var visited = std.AutoHashMap(MoveCache, void).init(allocator);
    defer visited.deinit();
    var end_pos = std.AutoHashMap(Pos, void).init(allocator);
    defer end_pos.deinit();

    try grid.walk(start_x, start_y, steps, &visited, &end_pos);

    return end_pos.count();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = @embedFile("input.txt");

    const out = try process(allocator, input, 64);
    std.debug.print("OUT: {d}\n", .{out});
}

test "example" {
    const input =
        \\...........
        \\.....###.#.
        \\.###.##..#.
        \\..#.#...#..
        \\....#.#....
        \\.##..S####.
        \\.##..#...#.
        \\.......##..
        \\.##.#.####.
        \\.##..##.##.
        \\...........
    ;

    try std.testing.expectEqual(@as(usize, 16), try process(std.testing.allocator, input, 6));
}
