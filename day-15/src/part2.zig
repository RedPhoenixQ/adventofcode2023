const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = @embedFile("input.txt");

    const out = try process(allocator, input);
    std.debug.print("OUT: {d}\n", .{out});
}

const Lenses = struct {
    const Box = std.StringArrayHashMap(usize);
    boxes: [256]Box = undefined,

    fn init(allocator: std.mem.Allocator) Lenses {
        var self: Lenses = undefined;
        for (&self.boxes) |*box| {
            box.* = Box.init(allocator);
        }
        return self;
    }

    fn deinit(self: *Lenses) void {
        for (&self.boxes) |*box| {
            box.*.deinit();
        }
    }

    const IntructionError = error{
        InvalidFocalLength,
    };

    fn handle_instruction(self: *Lenses, instruction: []const u8) !void {
        if (instruction[instruction.len - 1] == '-') {
            const label = instruction[0 .. instruction.len - 1];
            try self.remove(label);
        } else if (instruction[instruction.len - 2] == '=') {
            const label = instruction[0 .. instruction.len - 2];
            const focal_length: usize = switch (instruction[instruction.len - 1]) {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                else => return error.InvalidFocalLength,
            };
            try self.add_or_set_lens(label, focal_length);
        }
    }

    fn remove(self: *Lenses, label: []const u8) !void {
        const box_index = holiday_string_helper(label);
        _ = self.boxes[box_index].orderedRemove(label);
    }

    fn add_or_set_lens(self: *Lenses, label: []const u8, focal_length: usize) !void {
        const box_index = holiday_string_helper(label);
        try self.boxes[box_index].put(label, focal_length);
    }
};

fn process(allocator: std.mem.Allocator, input: []const u8) !usize {
    var lenses = Lenses.init(allocator);
    defer lenses.deinit();

    var left: usize = 0;
    for (input, 0..) |c, i| {
        if (c == ',') {
            try lenses.handle_instruction(input[left..i]);
            left = i + 1;
        }
    }
    try lenses.handle_instruction(input[left..]);

    var sum: usize = 0;
    for (lenses.boxes, 1..) |box, box_i| {
        for (box.values(), box.keys(), 1..) |focal_length, label, slot| {
            const lens_power = box_i * slot * focal_length;
            std.debug.print("{s}: {d} box * {d} slow * {d} focal = {d}\n", .{ label, box_i, slot, focal_length, lens_power });
            sum += lens_power;
        }
    }

    return sum;
}

fn holiday_string_helper(string: []const u8) usize {
    var hash: usize = 0;
    for (string) |c| {
        hash += c;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}

test "example" {
    const input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    try std.testing.expectEqual(@as(usize, 145), try process(std.testing.allocator, input));
}
