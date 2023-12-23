const std = @import("std");

const Direction = enum { Up, Down, Left, Right };

const MoveCache = struct {
    x: usize,
    y: usize,
    direction: Direction,
};

const Crucible = struct {
    x: usize,
    y: usize,
    heat_loss: usize,
    direction: Direction,
    steps: usize,
    steps_in_same_direction: u2 = 1,
};

const CrucibleContext = struct {
    target_x: usize,
    target_y: usize,

    fn distance(self: CrucibleContext, other: Crucible) usize {
        const dx: isize = @as(isize, @intCast(self.target_x)) - @as(isize, @intCast(other.x));
        const dy: isize = @as(isize, @intCast(self.target_y)) - @as(isize, @intCast(other.y));
        return std.math.absCast(dx) + std.math.absCast(dy);
    }
};

fn leastHeatloss(ctx: CrucibleContext, a: Crucible, b: Crucible) std.math.Order {
    return std.math.order(a.heat_loss * 2 + ctx.distance(a), b.heat_loss * 2 + ctx.distance(b));
    // return switch (std.math.order(a.heat_loss, b.heat_loss)) {
    //     .eq => std.math.order(ctx.distance(a), ctx.distance(b)),
    //     else => |order| order,
    // };
    //std.debug.print("a dist: {d}, b dist {d}\n", .{ ctx.distance(a), ctx.distance(b) });
    // var order: std.math.Order = undefined;
    // order = std.math.order(a.heatLoss, b.heatLoss);
    // if (order != .eq) return order;
    // order = std.math.order(ctx.distance(a), ctx.distance(b));
    // if (order != .eq) return order;
    // order = std.math.order(a.steps, b.steps);
    // return order;
}

const Grid = struct {
    slice: []u4,
    width: usize,
    height: usize,
    allocator: std.mem.Allocator,

    fn from_string(allocator: std.mem.Allocator, input: []const u8) !Grid {
        var self: Grid = undefined;
        self.width = std.ascii.indexOfIgnoreCase(input, "\n") orelse unreachable;
        self.height = @as(usize, input.len / self.width);
        self.allocator = allocator;
        self.slice = try allocator.alloc(u4, self.width * self.height);

        var other_chars: usize = 0;
        for (input, 0..) |c, i| {
            if (!std.ascii.isDigit(c)) {
                other_chars += 1;
                continue;
            }
            self.slice[i - other_chars] = @as(u4, @intCast(c - 0x30));
        }
        //self.slice = slice;
        return self;
    }

    fn deinit(self: Grid) void {
        self.allocator.free(self.slice);
    }

    fn get(self: Grid, x: usize, y: usize) u4 {
        return self.slice[x + y * self.width];
    }
};

const Pos = struct {
    x: usize,
    y: usize,
};

fn process(allocator: std.mem.Allocator, input: []const u8) !usize {
    const grid = try Grid.from_string(allocator, input);
    defer grid.deinit();
    std.debug.print("Width {d}, height {d}: {any}\n", .{ grid.width, grid.height, grid.slice });

    var queue = std.PriorityQueue(Crucible, CrucibleContext, leastHeatloss).init(allocator, .{
        .target_x = grid.width - 1,
        .target_y = grid.height - 1,
    });
    defer queue.deinit();

    var cache = std.AutoHashMap(MoveCache, usize).init(allocator);
    defer cache.deinit();
    var comeFrom = std.AutoHashMap(MoveCache, MoveCache).init(allocator);
    defer comeFrom.deinit();

    try queue.add(Crucible{
        .x = 0,
        .y = 0,
        .heat_loss = 0,
        .direction = .Right,
        .steps = 0,
        .steps_in_same_direction = 0,
    });

    const min_loss: usize = while (queue.removeOrNull()) |crucible| {
        //std.debug.print("Queue len {d}", .{queue.len});

        //std.debug.print("\n{any}\n", .{crucible});
        if (crucible.x == queue.context.target_x and crucible.y == queue.context.target_y) {
            // while (queue.removeOrNull()) |cru| {
            //     std.debug.print("{any}\n", .{cru});
            // }

            break crucible.heat_loss;
        }

        for (switch (crucible.direction) {
            .Down => [_]Direction{ .Left, .Down, .Right },
            .Up => [_]Direction{ .Left, .Up, .Right },
            .Right => [_]Direction{ .Up, .Right, .Down },
            .Left => [_]Direction{ .Up, .Left, .Down },
        }) |direction| {
            var next: Crucible = undefined;
            if (crucible.direction == direction) {
                if (crucible.steps_in_same_direction == 3) continue;
                next.steps_in_same_direction = 1 + crucible.steps_in_same_direction;
            } else {
                next.steps_in_same_direction = 0;
            }

            next.y = crucible.y;
            next.x = crucible.x;
            next.heat_loss = crucible.heat_loss;
            next.direction = direction;
            next.steps = crucible.steps + 1;

            switch (direction) {
                .Down => {
                    if (next.y == grid.height - 1) continue;
                    next.y += 1;
                },
                .Up => {
                    if (next.y == 0) continue;
                    next.y -= 1;
                },
                .Right => {
                    if (next.x == grid.width - 1) continue;
                    next.x += 1;
                },
                .Left => {
                    if (next.x == 0) continue;
                    next.x -= 1;
                },
            }

            next.heat_loss += grid.get(next.x, next.y);

            const cache_index = MoveCache{ .x = next.x, .y = next.y, .direction = direction };
            const cached_heatloss = cache.get(cache_index) orelse 0xFFFFFFFF;
            if (next.heat_loss < cached_heatloss) {
                std.debug.print("queue {}: {any}\n", .{ queue.len, next });
                try comeFrom.put(cache_index, MoveCache{ .x = crucible.x, .y = crucible.y, .direction = .Down });
                try cache.put(cache_index, next.heat_loss);
                try queue.add(next);
            }
            //std.debug.print("\nCACHE SKIPPED {d} vs {d}: {any}\n", .{ cached_heatloss.?, next.heatLoss, next });
        }

        // std.debug.print("\n\n", .{});
    } else 0;

    var path = std.AutoHashMap(MoveCache, MoveCache).init(allocator);
    defer path.deinit();

    var index = MoveCache{ .x = grid.width - 1, .y = grid.height - 1, .direction = .Down };

    while (comeFrom.get(index)) |move| {
        std.debug.print("Path {any}\n", .{move});
        try path.put(index, move);
        index.x = move.x;
        index.y = move.y;
    }

    for (grid.slice, 0..) |c, i| {
        const x = i % grid.width;
        const y = i / grid.width;

        if (x == 0) std.debug.print("\n", .{});
        const move = path.get(MoveCache{ .x = x, .y = y, .direction = .Down });
        if (move != null) {
            const arrow: u8 = switch (move.?.direction) {
                .Down => 'v',
                .Left => '<',
                .Right => '>',
                .Up => '^',
            };
            std.debug.print("{c}", .{arrow});
        } else {
            std.debug.print("{d}", .{c});
        }
    }

    return min_loss;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = @embedFile("input.txt");

    const out = try process(allocator, input);
    std.debug.print("OUT: {d}\n", .{out});
}

test "example" {
    const input =
        \\2413432311323
        \\3215453535623
        \\3255245654254
        \\3446585845452
        \\4546657867536
        \\1438598798454
        \\4457876987766
        \\3637877979653
        \\4654967986887
        \\4564679986453
        \\1224686865563
        \\2546548887735
        \\4322674655533
    ;
    try std.testing.expectEqual(@as(usize, 102), try process(std.testing.allocator, input));
}
